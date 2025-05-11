use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn optimize(svg: String) -> String {
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    tree.to_string(&usvg::WriteOptions::default())
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
