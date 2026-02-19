use wasm_bindgen::prelude::*;


#[derive(Default)]
pub enum WndMode {
    #[default] Home,
}

#[derive(Default)]
pub struct Wnd {
    id: String,
    mode: WndMode,
    msg_id: String,
}

impl Wnd {
    pub fn new(id: &str, msg_id: &str) -> Self {
        let mut wnd = Self::default();
        wnd.id = id.to_string();
        wnd.mode = WndMode::Home;
        wnd.msg_id = msg_id.to_string();
        wnd
    }

    pub fn draw(&self) {
        let window = web_sys::window().expect("グローバルな「Window」がありません");
        let document = window.document().expect("「Window」に「Document」がありません");
        let msg_ele = document.get_element_by_id(&self.msg_id).expect("「msg」エレメントがありません").dyn_into::<web_sys::HtmlInputElement>().unwrap();
        let wnd_ele = document.get_element_by_id(&self.id).expect("「wnd」エレメントがありません").dyn_into::<web_sys::HtmlInputElement>().unwrap();
        match self.mode {
            WndMode::Home => {
                msg_ele.set_text_content(Some("やりたいことをえらんでください"));
            }
        }
    }
}
