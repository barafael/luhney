#![feature(iter_array_chunks)]

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn help_text() -> String {
    "This program validates credit card numbers.".to_string()
}

// TODO use thiserror to define error types of this function (tooshort, invalid digits, etc.)
#[wasm_bindgen]
pub fn luhn(cc_number: &str) -> bool {
    let cc_number: String = cc_number.chars().filter(|c| !c.is_whitespace()).collect();
    if cc_number.len() < 2 {
        return false;
    }
    let mut acc = 0;
    let mut chunks = cc_number.chars().rev().array_chunks::<2>();

    for [a, b] in chunks.by_ref() {
        let (Some(a), Some(b)) = (a.to_digit(10), b.to_digit(10)) else {
            return false;
        };
        acc += a;
        let b = 2 * b;
        acc += b / 10 + b % 10;
    }
    acc += chunks
        .into_remainder()
        .map(|c| c.map(|c| c.to_digit(10).unwrap_or_default()).sum::<u32>())
        .unwrap_or_default();
    acc % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[test]
fn test_() {
    let s = "4263 9826 4026 9299";
    println!("{}", luhn(s));
}
