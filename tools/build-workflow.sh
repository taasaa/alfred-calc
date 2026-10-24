#!/usr/bin/env bash
# Assemble alfred-calc.alfredworkflow from alfred/info.plist and a release binary.
# The archive is what a user double-clicks to install the workflow in Alfred.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

out="$root/alfred-calc.alfredworkflow"
stage="$root/target/workflow"
arm="$root/target/release/alfred-calc"
intel="$root/target/x86_64-apple-darwin/release/alfred-calc"

echo "building release binary"
cargo build --release --quiet

if rustup target list --installed 2>/dev/null | grep -qx x86_64-apple-darwin; then
  echo "building intel slice"
  cargo build --release --quiet --target x86_64-apple-darwin
fi

mkdir -p "$stage"
cp alfred/info.plist "$stage/info.plist"

if [ -f "$intel" ]; then
  lipo -create -output "$stage/alfred-calc" "$arm" "$intel"
else
  echo "warning: intel slice missing, packaging arm64 only" >&2
  cp "$arm" "$stage/alfred-calc"
fi
chmod +x "$stage/alfred-calc"

# -X drops extended attributes so the archive carries no resource forks.
# -FS removes entries that no longer exist in the staged directory.
( cd "$stage" && zip -q -X -FS "$out" info.plist alfred-calc )

echo "built $out"
unzip -l "$out"
