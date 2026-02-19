use wasm_bindgen::prelude::*;

const WINDOW: web_sys::Window = web_sys::window().expect("グローバルな「Window」がありません");
const DOCUMENT: web_sys::Document = WINDOW.document().expect("「Window」に「Document」がありません");

pub enum WndMode {
    Home,
}

#[derive(Default)]
pub struct Wnd {
    id: String,
    mode: WndMode,
    msg_id: String,
}

impl Wnd {
    pub fn new(id: &str, msg_id: &str) -> Self {
        let wnd = Self::default();
        wnd.id = id;
        wnd.mode = WndMode::Home;
        wnd.msg_id = msg_id;
        wnd
    }

    pub fn draw() {
        let msg_ele = DOCUMENT.get_element_by_id(self.msg_id).unwrap("「msg」エレメントがありません").dyn_into::<web_sys::HtmlInputElement>().unwrap();
        let wnd_ele = DOCUMENT.get_element_by_id(self.msg_id).unwrap("「wnd」エレメントがありません").dyn_into::<web_sys::HtmlInputElement>().unwrap();
        match mode {
            WndMode::Home => {
                msg_ele.text_content = "やりたいことをえらんでください";
            }
        }
    }
}
