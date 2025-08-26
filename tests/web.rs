//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use code_snap::codesnap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_codesnap_without_config_browser() {
    let code = "fn main() { println!(\"Hello, world!\"); }";
    let language = "rust";
    let config = None;

    let result = codesnap(code, language, config);

    assert!(result.width > 0);
    assert!(result.height > 0);
    assert!(result.data().length() > 0);
}

#[wasm_bindgen_test]
fn test_codesnap_with_config_browser() {
    let code = "fn main() { println!(\"Hello, world!\"); }";
    let language = "rust";
    let config = Some(
        r###"{
            "theme": "candy",
            "background": "#000000"
        }"###
            .to_string(),
    );

    let result = codesnap(code, language, config);

    assert!(result.width > 0);
    assert!(result.height > 0);
    assert!(result.data().length() > 0);
}
