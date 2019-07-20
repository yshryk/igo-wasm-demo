use std::io::Cursor;

use brotli_decompressor::BrotliDecompress;
use igo::Tagger;
use serde_derive::*;
use wasm_bindgen::prelude::*;
use web_sys;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Serialize)]
pub struct Morpheme<'a> {
    pub surface: &'a str,
    pub feature: &'a str,
}

#[wasm_bindgen]
pub fn init_tagger() -> *mut Tagger {
    utils::set_panic_hook();

    log!("Tagger::new");
    // brotli展開後のサイズは40MB弱
    let decompressed_size = 40 * 1024 * 1024;
    let mut decompressed_zip = Vec::with_capacity(decompressed_size);
    BrotliDecompress(&mut Cursor::new(include_bytes!("../ipadic.zip.br") as &[u8]),
                     &mut decompressed_zip).expect("brotli decompress");

    let mut zip_dir = igo::ZipDir::new(Cursor::new(decompressed_zip)).expect("unzip");
    let tagger = Tagger::load_from_dir(&mut zip_dir).expect("tagger load");

    Box::into_raw(Box::new(tagger))
}

#[wasm_bindgen]
pub fn parse(tagger_ptr: *mut Tagger, text: &str) -> JsValue {
    let tagger = unsafe { Box::from_raw(tagger_ptr) };
    log!("Tagger::parse");
    let results = tagger.parse(text);
    let mut buf = Vec::with_capacity(results.len());
    for m in &results {
        buf.push(Morpheme { surface: m.surface, feature: m.feature });
    }

    let json = JsValue::from_serde(&buf).expect("serialize to json");
    std::mem::forget(tagger);
    json
}
