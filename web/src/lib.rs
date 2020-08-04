mod error;
mod pages;

use wasm_bindgen::prelude::*;
use error::JsResult;
use pages::Layout;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


struct Titso {
    layout: Layout
}

impl Titso {
    pub fn init() -> JsResult<Titso> {
        let window = web_sys::window().ok_or("not found window")?;
        let document = window.document().ok_or("not found document")?;

        let layout = Layout::new(&document)?;

        Ok(Titso { layout })
    }
}

pub fn start() {
    let titso = Titso::init().unwrap();

    todo!()
}
