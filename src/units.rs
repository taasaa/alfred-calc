//! Measurement units. Every unit is affine against its category's base unit:
//! `base = value * factor + offset`, which handles temperature without special cases.

#[derive(Clone, Copy)]
pub struct Unit {
    pub symbol: &'static str,
    pub names: &'static [&'static str],
    pub factor: f64,
    pub offset: f64,
}

impl Unit {
    const fn linear(symbol: &'static str, names: &'static [&'static str], factor: f64) -> Unit {
        Unit { symbol, names, factor, offset: 0.0 }
    }
}

pub struct Category {
    pub name: &'static str,
    pub units: &'static [Unit],
}

const LENGTH: &[Unit] = &[
    Unit::linear("m", &["m", "meter", "meters", "metre", "metres"], 1.0),
    Unit::linear("km", &["km", "kilometer", "kilometers", "kilometre", "kilometres"], 1000.0),
    Unit::linear("dm", &["dm", "decimeter", "decimeters"], 0.1),
    Unit::linear("cm", &["cm", "centimeter", "centimeters", "centimetre", "centimetres"], 0.01),
    Unit::linear("mm", &["mm", "millimeter", "millimeters", "millimetre", "millimetres"], 0.001),
    Unit::linear("μm", &["μm", "um", "micrometer", "micrometers", "micron", "microns"], 1e-6),
    Unit::linear("nm", &["nm", "nanometer", "nanometers", "nanometre", "nanometres"], 1e-9),
    Unit::linear("pm", &["pm", "picometer", "picometers"], 1e-12),
    Unit::linear("in", &["in", "inch", "inches"], 0.0254),
    Unit::linear("ft", &["ft", "foot", "feet"], 0.3048),
    Unit::linear("yd", &["yd", "yard", "yards"], 0.9144),
    Unit::linear("mi", &["mi", "mile", "miles"], 1609.344),
    Unit::linear("nmi", &["nmi", "nautical mile", "nautical miles"], 1852.0),
    Unit::linear("ly", &["ly", "lightyear", "lightyears", "light year", "light years"], 9.4607304725808e15),
    Unit::linear("au", &["au", "astronomical unit", "astronomical units"], 1.49597870700e11),
    Unit::linear("pc", &["pc", "parsec", "parsecs"], 3.085677581491367e16),
];

const AREA: &[Unit] = &[
    Unit::linear("m2", &["m2", "sqm", "square meter", "square meters", "square metre", "square metres"], 1.0),
    Unit::linear("km2", &["km2", "square kilometer", "square kilometers", "sqkm"], 1e6),
    Unit::linear("cm2", &["cm2", "square centimeter", "square centimeters"], 1e-4),
    Unit::linear("mm2", &["mm2", "square millimeter", "square millimeters"], 1e-6),
    Unit::linear("ft2", &["ft2", "sqft", "square foot", "square feet"], 0.09290304),
    Unit::linear("mi2", &["mi2", "square mile", "square miles"], 2589988.110336),
    Unit::linear("ac", &["ac", "acre", "acres"], 4046.8564224),
    Unit::linear("ha", &["ha", "hectare", "hectares"], 10000.0),
];

const VOLUME: &[Unit] = &[
    Unit::linear("l", &["l", "liter", "liters", "litre", "litres"], 1.0),
    Unit::linear("ml", &["ml", "milliliter", "milliliters", "millilitre", "millilitres"], 0.001),
    Unit::linear("cl", &["cl", "centiliter", "centiliters"], 0.01),
    Unit::linear("dl", &["dl", "deciliter", "deciliters"], 0.1),
    Unit::linear("hl", &["hl", "hectoliter", "hectoliters"], 100.0),
    Unit::linear("m3", &["m3", "cubic meter", "cubic meters", "cubic metre", "cubic metres"], 1000.0),
    Unit::linear("cm3", &["cm3", "cc", "cubic centimeter", "cubic centimeters"], 0.001),
    Unit::linear("mm3", &["mm3", "cubic millimeter", "cubic millimeters"], 1e-6),
    Unit::linear("ft3", &["ft3", "cubic foot", "cubic feet"], 28.316846592),
    Unit::linear("in3", &["in3", "cubic inch", "cubic inches"], 0.016387064),
    Unit::linear("yd3", &["yd3", "cubic yard", "cubic yards"], 764.554857984),
    Unit::linear("usgal", &["usgal", "us gallon", "us gallons", "gallon", "gallons", "gal"], 3.785411784),
    Unit::linear("ukgal", &["ukgal", "uk gallon", "uk gallons", "imperial gallon", "imperial gallons"], 4.54609),
    Unit::linear("usqt", &["usqt", "us quart", "us quarts", "quart", "quarts", "qt"], 0.946352946),
    Unit::linear("ukqt", &["ukqt", "uk quart", "uk quarts"], 1.1365225),
    Unit::linear("uspt", &["uspt", "us pint", "us pints", "pint", "pints", "pt"], 0.473176473),
    Unit::linear("ukpt", &["ukpt", "uk pint", "uk pints"], 0.56826125),
    Unit::linear("floz", &["floz", "fl oz", "fluid ounce", "fluid ounces"], 0.0295735295625),
];

const WEIGHT: &[Unit] = &[
    Unit::linear("kg", &["kg", "kilogram", "kilograms"], 1.0),
    Unit::linear("g", &["g", "gram", "grams"], 0.001),
    Unit::linear("mg", &["mg", "milligram", "milligrams"], 1e-6),
    Unit::linear("lb", &["lb", "lbs", "pound", "pounds"], 0.45359237),
    Unit::linear("oz", &["oz", "ounce", "ounces"], 0.028349523125),
    Unit::linear("st", &["st", "stone", "stones"], 6.35029318),
    Unit::linear("t", &["t", "tonne", "tonnes", "ton", "tons"], 1000.0),
    Unit::linear("ukt", &["ukt", "uk ton", "uk tons", "long ton", "long tons"], 1016.0469088),
    Unit::linear("ust", &["ust", "us ton", "us tons", "short ton", "short tons"], 907.18474),
];

const SPEED: &[Unit] = &[
    Unit::linear("mps", &["mps", "m/s", "meter per second", "meters per second"], 1.0),
    Unit::linear("kph", &["kph", "kmh", "km/h", "kilometer per hour", "kilometers per hour"], 0.2777777777777778),
    Unit::linear("mph", &["mph", "mile per hour", "miles per hour"], 0.44704),
    Unit::linear("fps", &["fps", "foot per second", "feet per second"], 0.3048),
    Unit::linear("knot", &["knot", "knots", "kn"], 0.5144444444444445),
];

const ROTATION: &[Unit] = &[
    Unit::linear("deg", &["deg", "degree", "degrees"], 1.0),
    Unit::linear("rad", &["rad", "radian", "radians"], 57.29577951308232),
];

const TEMPERATURE: &[Unit] = &[
    Unit { symbol: "k", names: &["k", "kelvin"], factor: 1.0, offset: 0.0 },
    Unit { symbol: "c", names: &["c", "celsius", "centigrade", "°c"], factor: 1.0, offset: 273.15 },
    Unit {
        symbol: "f",
        names: &["f", "fahrenheit", "°f"],
        factor: 5.0 / 9.0,
        offset: (459.67_f64) * 5.0 / 9.0,
    },
];

const PRESSURE: &[Unit] = &[
    Unit::linear("pa", &["pa", "pascal", "pascals"], 1.0),
    Unit::linear("hpa", &["hpa", "hectopascal", "hectopascals"], 100.0),
    Unit::linear("kpa", &["kpa", "kilopascal", "kilopascals"], 1000.0),
    Unit::linear("mpa", &["mpa", "megapascal", "megapascals"], 1e6),
    Unit::linear("bar", &["bar", "bars"], 1e5),
    Unit::linear("mbar", &["mbar", "millibar", "millibars"], 100.0),
    Unit::linear("psi", &["psi", "pound per square inch", "pounds per square inch"], 6894.757293168),
];

const TIME: &[Unit] = &[
    Unit::linear("s", &["s", "sec", "secs", "second", "seconds"], 1.0),
    Unit::linear("ms", &["ms", "millisecond", "milliseconds"], 0.001),
    Unit::linear("μs", &["μs", "us", "microsecond", "microseconds"], 1e-6),
    Unit::linear("ns", &["ns", "nanosecond", "nanoseconds"], 1e-9),
    Unit::linear("min", &["min", "mins", "minute", "minutes"], 60.0),
    Unit::linear("hr", &["hr", "hrs", "hour", "hours"], 3600.0),
    Unit::linear("day", &["day", "days"], 86400.0),
    Unit::linear("week", &["week", "weeks"], 604800.0),
    Unit::linear("month", &["month", "months"], 2628000.0),
    Unit::linear("year", &["year", "years"], 31536000.0),
];

const ENERGY: &[Unit] = &[
    Unit::linear("j", &["j", "joule", "joules"], 1.0),
    Unit::linear("kj", &["kj", "kilojoule", "kilojoules"], 1000.0),
    Unit::linear("mj", &["mj", "megajoule", "megajoules"], 1e6),
    Unit::linear("cal", &["cal", "calorie", "calories"], 4.184),
    Unit::linear("kcal", &["kcal", "kilocalorie", "kilocalories"], 4184.0),
    Unit::linear("whr", &["whr", "wh", "watt hour", "watt hours"], 3600.0),
    Unit::linear("kwhr", &["kwhr", "kwh", "kilowatt hour", "kilowatt hours"], 3.6e6),
    Unit::linear("mwhr", &["mwhr", "mwh", "megawatt hour", "megawatt hours"], 3.6e9),
    Unit::linear("ftlb", &["ftlb", "ft-lb", "foot pound", "foot pounds"], 1.3558179483314004),
    Unit::linear("ev", &["ev", "electronvolt", "electronvolts"], 1.602176634e-19),
];

const POWER: &[Unit] = &[
    Unit::linear("w", &["w", "watt", "watts"], 1.0),
    Unit::linear("kw", &["kw", "kilowatt", "kilowatts"], 1000.0),
    Unit::linear("ps", &["ps", "metric horsepower"], 735.49875),
    Unit::linear("hp", &["hp", "horsepower"], 745.6998715822702),
];

/// Decimal (SI) storage units, base = byte. Binary (IEC) units included separately so
/// `gb` and `gib` never collapse into the same value.
const DATA: &[Unit] = &[
    Unit::linear("b", &["b", "byte", "bytes"], 1.0),
    Unit::linear("bit", &["bit", "bits"], 0.125),
    Unit::linear("kb", &["kb", "kilobyte", "kilobytes"], 1e3),
    Unit::linear("mb", &["mb", "megabyte", "megabytes"], 1e6),
    Unit::linear("gb", &["gb", "gigabyte", "gigabytes"], 1e9),
    Unit::linear("tb", &["tb", "terabyte", "terabytes"], 1e12),
    Unit::linear("pb", &["pb", "petabyte", "petabytes"], 1e15),
    Unit::linear("eb", &["eb", "exabyte", "exabytes"], 1e18),
    Unit::linear("zb", &["zb", "zettabyte", "zettabytes"], 1e21),
    Unit::linear("yb", &["yb", "yottabyte", "yottabytes"], 1e24),
    Unit::linear("kib", &["kib", "kibibyte", "kibibytes"], 1024.0),
    Unit::linear("mib", &["mib", "mebibyte", "mebibytes"], 1_048_576.0),
    Unit::linear("gib", &["gib", "gibibyte", "gibibytes"], 1_073_741_824.0),
    Unit::linear("tib", &["tib", "tebibyte", "tebibytes"], 1_099_511_627_776.0),
    Unit::linear("pib", &["pib", "pebibyte", "pebibytes"], 1_125_899_906_842_624.0),
    Unit::linear("eib", &["eib", "exbibyte", "exbibytes"], 1_152_921_504_606_846_976.0),
];

pub const CATEGORIES: &[Category] = &[
    Category { name: "Length", units: LENGTH },
    Category { name: "Area", units: AREA },
    Category { name: "Volume", units: VOLUME },
    Category { name: "Weight", units: WEIGHT },
    Category { name: "Speed", units: SPEED },
    Category { name: "Rotation", units: ROTATION },
    Category { name: "Temperature", units: TEMPERATURE },
    Category { name: "Pressure", units: PRESSURE },
    Category { name: "Time", units: TIME },
    Category { name: "Energy", units: ENERGY },
    Category { name: "Power", units: POWER },
    Category { name: "Data", units: DATA },
];

/// A resolved unit reference: category index and unit index.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitRef {
    pub category: usize,
    pub unit: usize,
}

impl UnitRef {
    pub fn unit(&self) -> &'static Unit {
        &CATEGORIES[self.category].units[self.unit]
    }
    pub fn category_name(&self) -> &'static str {
        CATEGORIES[self.category].name
    }
    pub fn symbol(&self) -> &'static str {
        self.unit().symbol
    }

    /// Conventional casing for display: storage units are `MB`/`MiB`, not `mb`.
    pub fn display(&self) -> String {
        match self.unit().symbol {
            "b" => "B".to_string(),
            "kb" => "kB".to_string(),
            "mb" => "MB".to_string(),
            "gb" => "GB".to_string(),
            "tb" => "TB".to_string(),
            "pb" => "PB".to_string(),
            "eb" => "EB".to_string(),
            "zb" => "ZB".to_string(),
            "yb" => "YB".to_string(),
            "kib" => "KiB".to_string(),
            "mib" => "MiB".to_string(),
            "gib" => "GiB".to_string(),
            "tib" => "TiB".to_string(),
            "pib" => "PiB".to_string(),
            "eib" => "EiB".to_string(),
            "c" => "°C".to_string(),
            "f" => "°F".to_string(),
            "k" => "K".to_string(),
            other => other.to_string(),
        }
    }
}

fn normalize(token: &str) -> String {
    token
        .trim()
        .to_lowercase()
        .replace('°', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Resolve a token to a unit. Exact symbol matches win across all categories before
/// any name match, so `b` resolves to byte rather than to a name-alias of something else.
pub fn resolve(token: &str) -> Option<UnitRef> {
    let token = normalize(token);
    if token.is_empty() {
        return None;
    }
    for (category, cat) in CATEGORIES.iter().enumerate() {
        for (unit, def) in cat.units.iter().enumerate() {
            if def.symbol == token {
                return Some(UnitRef { category, unit });
            }
        }
    }
    for (category, cat) in CATEGORIES.iter().enumerate() {
        for (unit, def) in cat.units.iter().enumerate() {
            if def.names.iter().any(|name| *name == token) {
                return Some(UnitRef { category, unit });
            }
        }
    }
    None
}

pub fn convert(value: f64, from: UnitRef, to: UnitRef) -> Option<f64> {
    if from.category != to.category {
        return None;
    }
    let f = from.unit();
    let t = to.unit();
    let base = value * f.factor + f.offset;
    Some((base - t.offset) / t.factor)
}

/// Every unit in a category, for the "no target given" listing.
pub fn siblings(reference: UnitRef) -> Vec<UnitRef> {
    CATEGORIES[reference.category]
        .units
        .iter()
        .enumerate()
        .map(|(unit, _)| UnitRef { category: reference.category, unit })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(token: &str) -> UnitRef {
        resolve(token).unwrap_or_else(|| panic!("unresolved unit {token}"))
    }

    #[test]
    fn resolves_symbols_and_names() {
        assert_eq!(u("km").symbol(), "km");
        assert_eq!(u("kilometers").symbol(), "km");
        assert_eq!(u("Square Metres").symbol(), "m2");
        assert_eq!(u("°C").symbol(), "c");
        assert_eq!(u("lbs").symbol(), "lb");
        assert_eq!(u("b").symbol(), "b", "b must be byte, not a name alias");
        assert_eq!(u("bar").symbol(), "bar");
        assert!(resolve("eur").is_none());
    }

    #[test]
    fn converts_linearly() {
        let km = u("km");
        let m = u("m");
        assert_eq!(convert(5.0, km, m), Some(5000.0));
        assert_eq!(convert(5000.0, m, km), Some(5.0));
        let gib = u("gib");
        let mib = u("mib");
        assert_eq!(convert(1.0, gib, mib), Some(1024.0));
    }

    #[test]
    fn converts_temperature_affinely() {
        let c = u("c");
        let f = u("f");
        let k = u("k");
        let close = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-9;
        assert!(close(convert(100.0, c, f), 212.0));
        assert!(close(convert(32.0, f, c), 0.0));
        assert!(close(convert(0.0, c, k), 273.15));
        assert!(close(convert(273.15, k, c), 0.0));
    }

    #[test]
    fn decimal_and_binary_storage_stay_distinct() {
        let gb = u("gb");
        let gib = u("gib");
        let mb = u("mb");
        assert_eq!(convert(1.0, gb, mb), Some(1000.0));
        assert_eq!(convert(1.0, gib, mb), Some(1073.741824));
        // kb must be one step above the byte, not equal to it.
        assert_eq!(convert(1.0, u("kb"), u("b")), Some(1000.0));
        assert_eq!(convert(2.0, u("tb"), u("gb")), Some(2000.0));
    }

    #[test]
    fn refuses_cross_category() {
        assert_eq!(convert(1.0, u("km"), u("kg")), None);
    }
}
