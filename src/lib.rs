use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use codesnap::{
    config::{CodeBuilder, CodeSnap, Content},
    snapshot::{image_snapshot::ImageSnapshot, snapshot_data::SnapshotData},
};

#[wasm_bindgen]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl ImageData {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Config {
    pub config: CodeSnap,
}

#[wasm_bindgen]
pub fn codesnap(code: &str, language: &str, config: Option<Config>) -> ImageData {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(code)
            .language(language)
            .build()
            .unwrap(),
    );

    let snap_config = match config {
        Some(cfg) => cfg.config,
        None => CodeSnap::from_default_theme().unwrap(),
    }
    .content(code_content)
    .build()
    .unwrap();

    let image_data = ImageSnapshot::from_config(snap_config)
        .unwrap()
        .png_data()
        .unwrap();

    match image_data {
        SnapshotData::Image {
            data,
            width,
            height,
        } => ImageData {
            data,
            width,
            height,
        },
        _ => panic!("Expected image data but received different snapshot data"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_codesnap_without_config() {
        let code = "fn main() { println!(\"Hello, world!\"); }";
        let language = "rust";
        let config = None;

        let result = codesnap(code, language, config);

        assert!(result.width > 0);
        assert!(result.height > 0);
        assert!(!result.data.is_empty());
    }

    #[test]
    fn test_codesnap_with_config() {
        let code = "fn main() { println!(\"Hello, world!\"); }";
        let language = "rust";
        let config = Some(Config {
            config: CodeSnap::from_config(
                r###"{
                    "theme": "candy",
                    "background": "#000000"
                }"###,
            )
            .unwrap(),
        });

        let result = codesnap(code, language, config);

        assert!(result.width > 0);
        assert!(result.height > 0);
        assert!(!result.data.is_empty());
    }
}
