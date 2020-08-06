mod error;
mod pages;
mod op;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{ Window, Storage };
use kvdb_web::Database;
use titso_core::Titso as Core;
use error::JsResult;
use pages::Layout;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub struct Titso {
    window: Window,
    layout: Layout,
    core: RefCell<Option<Core>>,
    db: Database
}

impl Titso {
    async fn init() -> JsResult<Titso> {
        let window = web_sys::window().ok_or("not found window")?;
        let document = window.document().ok_or("not found document")?;

        let layout = Layout::new(&document)?;
        let core = RefCell::new(None);
        let db = Database::open("titso".into(), 1)
            .await
            .map_err(|err| err.to_string())?;

        Ok(Titso { window, layout, core, db })
    }
}

#[wasm_bindgen]
pub async fn start() -> JsResult<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let titso = Titso::init().await?;
    let titso = Rc::new(titso);
    let titso2 = titso.clone();

    titso.layout.hook(titso2)?;

    todo!()
}
