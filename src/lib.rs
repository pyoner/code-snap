use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use codesnap::{
    config::{CodeBuilder, CodeSnap, Content, SnapshotConfig},
    snapshot::{image_snapshot::ImageSnapshot, snapshot_data::SnapshotData},
};

#[wasm_bindgen]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    ptr: usize,
    len: usize,
    cap: usize,
}

#[wasm_bindgen]
impl ImageData {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Uint8Array {
        unsafe {
            let d = Vec::from_raw_parts(self.ptr as *mut u8, self.len, self.cap);
            let view = Uint8Array::view(&d);
            std::mem::forget(d); // <-- leak it so view remains valid
            view
        }
    }

    pub fn free(&self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.ptr as *mut u8, self.len, self.cap);
            // Vec drops here, freeing the memory
        }
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
        } => {
            // Capture length information before moving `data` into ImageData
            let (ptr, len, cap) = {
                let d = &data;
                (d.as_ptr() as usize, d.len(), d.capacity())
            };
            std::mem::forget(data);
            ImageData {
                ptr,
                len,
                cap,
                width,
                height,
            }
        }
        _ => panic!("Expected image data but received different snapshot data"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_codesnap_without_config() {
        let code = "fn main() { println!(\"Hello, world!\"); }";
        let language = "rust";
        let config = None;

        let result = codesnap(code, language, config);

        assert!(result.width > 0);
        assert!(result.height > 0);

        result.data();
        result.free();
    }

    #[test]
    #[wasm_bindgen_test]
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

        result.data();
        result.free();
    }
}
