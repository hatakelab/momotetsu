mod wnd;

use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlInputElement,
};
use wnd::*;

#[wasm_bindgen]
pub fn run(wnd_id: &str, msg_id: &str) {
    let wnd = Wnd::new(wnd_id, msg_id);
    wnd.draw();
}
