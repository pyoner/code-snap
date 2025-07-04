use wasm_bindgen::prelude::*;

use codesnap::{
    config::{CodeBuilder, CodeSnap, Content, SnapshotConfig},
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

#[wasm_bindgen]
pub fn codesnap(code: &str, language: &str, config: Option<String>) -> ImageData {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(code)
            .language(language)
            .build()
            .unwrap(),
    );

    let snap_config: SnapshotConfig = match config {
        Some(cfg) => CodeSnap::from_config(&cfg),
        None => CodeSnap::from_default_theme(),
    }
    .unwrap()
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
        assert!(!result.data.is_empty());
    }
}
