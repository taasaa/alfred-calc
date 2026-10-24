//! HTTP fetching with an on-disk cache that reports staleness instead of hiding it.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Cached {
    pub body: String,
    /// Set when the network refresh failed and older cached data is being shown.
    pub stale_note: Option<String>,
}

fn cache_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("alfred_workflow_cache") {
        if !dir.is_empty() {
            return PathBuf::from(dir);
        }
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join("Library/Caches/alfred-calc")
}

fn age(path: &PathBuf) -> Option<Duration> {
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    SystemTime::now().duration_since(modified).ok()
}

fn human_age(d: Duration) -> String {
    let secs = d.as_secs();
    if secs < 90 {
        format!("{secs}s")
    } else if secs < 5400 {
        format!("{}m", secs / 60)
    } else if secs < 172800 {
        format!("{}h", secs / 3600)
    } else {
        format!("{}d", secs / 86400)
    }
}

/// Fetch `url`, using `file` as a cache. Fresh cache short-circuits the network.
/// On network failure a stale cache is returned with a note, because silently
/// serving old rates is worse than saying how old they are.
pub fn get(url: &str, file: &str, ttl: Duration) -> Result<Cached, String> {
    let dir = cache_dir();
    let path = dir.join(file);
    let cached = fs::read_to_string(&path).ok();

    if let Some(cache_age) = age(&path) {
        if cache_age < ttl {
            if let Some(body) = cached.clone() {
                return Ok(Cached { body, stale_note: None });
            }
        }
    }

    match fetch(url) {
        Ok(body) => {
            if let Err(err) = fs::create_dir_all(&dir).and_then(|_| fs::write(&path, &body)) {
                return Ok(Cached {
                    body,
                    stale_note: Some(format!("cache write failed: {err}")),
                });
            }
            Ok(Cached { body, stale_note: None })
        }
        Err(err) => match cached {
            Some(body) => {
                let note = age(&path)
                    .map(|d| format!("rates {} old", human_age(d)))
                    .unwrap_or_else(|| "cached rates, age unknown".to_string());
                Ok(Cached {
                    body,
                    stale_note: Some(format!("{note}; refresh failed: {err}")),
                })
            }
            None => Err(err),
        },
    }
}

fn fetch(url: &str) -> Result<String, String> {
    let response = ureq::get(url)
        .timeout(Duration::from_secs(6))
        .call()
        .map_err(|err| err.to_string())?;
    response.into_string().map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_cache_age() {
        assert_eq!(human_age(Duration::from_secs(30)), "30s");
        assert_eq!(human_age(Duration::from_secs(600)), "10m");
        assert_eq!(human_age(Duration::from_secs(7200)), "2h");
        assert_eq!(human_age(Duration::from_secs(259200)), "3d");
    }

    /// A refresh that fails must surface the cached body *with* an explanation,
    /// never silently pretend the numbers are current.
    #[test]
    fn stale_cache_is_reported_when_refresh_fails() {
        let dir = std::env::temp_dir().join(format!("alfred-calc-fetch-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("temp cache dir");
        fs::write(dir.join("rates.json"), "{\"cached\":true}").expect("seed cache");
        std::env::set_var("alfred_workflow_cache", &dir);

        // Port 9 is the discard port; nothing listens, so the connection is refused.
        // A zero TTL forces the refresh attempt.
        let got = get("http://127.0.0.1:9/rates", "rates.json", Duration::from_secs(0))
            .expect("stale fallback should succeed");

        std::env::remove_var("alfred_workflow_cache");
        fs::remove_dir_all(&dir).ok();

        assert_eq!(got.body, "{\"cached\":true}");
        let note = got.stale_note.expect("stale note must be present");
        assert!(note.contains("refresh failed"), "note was: {note}");
    }
}
