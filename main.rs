

use std::env;


struct PasswordAnalysis {
    length: usize,
    has_lower: bool,
    has_upper: bool,
    has_digit: bool,
    has_symbol: bool,
}



fn analyze_password(password: &str) -> PasswordAnalysis {
    let mut analysis = PasswordAnalysis {
        length: password.len(),
        has_lower: false,
        has_upper: false,
        has_digit: false,
        has_symbol: false,
    };


    for ch in password.chars() {
        if ch.is_lowercase() {
            analysis.has_lower = true;
        } else if ch.is_uppercase() {
            analysis.has_upper = true;
        } else if ch.is_numeric() {
            analysis.has_digit = true;
        } else {
            analysis.has_symbol = true;
        }
    }

    analysis
}


fn charset_size(analysis: &PasswordAnalysis) -> u32 {
    let mut size = 0;

    if analysis.has_lower {
        size += 26;
    }
    if analysis.has_upper {
        size += 26;
    }
    if analysis.has_digit {
        size += 10;
    }
    if analysis.has_symbol {
        size += 32;
    }

    size
}


fn estimate_crack_time(length: usize, charset: u32) -> f64 {
    let combinations = (charset as f64).powi(length as i32);
    let guesses_per_second = 1e10_f64;
    combinations / guesses_per_second
}


fn format_time(seconds: f64) -> String {
    const MINUTE: f64 = 60.0;
    const HOUR: f64 = 60.0 * MINUTE;
    const DAY: f64 = 24.0 * HOUR;
    const YEAR: f64 = 365.0 * DAY;

    if seconds < MINUTE {
        format!("{:.2} seconds", seconds)
    } else if seconds < HOUR {
        format!("{:.2} minutes", seconds / MINUTE)
    } else if seconds < DAY {
        format!("{:.2} hours", seconds / HOUR)
    } else if seconds < YEAR {
        format!("{:.2} days", seconds / DAY)
    } else {
        format!("{:.2} years", seconds / YEAR)
    }
}



fn strength_label(score: u8) -> &'static str {
    match score {
        0..=2 => "Weak",
        3..=4 => "Medium",
        _ => "Strong",
    }
}


