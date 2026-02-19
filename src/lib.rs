mod wnd;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(wnd_id: &str, msg_id: &str) {
    let wnd = wnd::Wnd::new(wnd_id, msg_id);
    wnd.draw();
}
