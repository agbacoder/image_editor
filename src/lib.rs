use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{ encode, decode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;


#[wasm_bindgen]

pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscale".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"data_1".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"data_44545".into());

    img = img.grayscale();
    log(&"grayscale".into());

    
    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new img".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );
    data_url
}

