

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

