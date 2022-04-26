mod error;
mod common;
mod pages;
mod op;

use std::rc::Rc;
use std::cell::RefCell;
use log::debug;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Window;
use indexed_kv::IndexedKv;
use seckey::ZeroAllocator;
use titso_core::Core as Core;
use error::JsResult;
use common::{ Password, Lock };
use pages::Layout;

#[global_allocator]
static ALLOC: ZeroAllocator<wee_alloc::WeeAlloc> = ZeroAllocator(wee_alloc::WeeAlloc::INIT);


pub struct Titso {
    window: Window,
    layout: Layout,
    core: RefCell<Option<Core>>,
    db: IndexedKv,
    password: RefCell<Password>,
    defense: Lock
}

static FUNCTIONS: titso_core::Functions = titso_core::Functions {
    rng: |buf| getrandom::getrandom(buf).unwrap(),
    zero: seckey::zero,
    malloc: || Box::new(HeapBytes([0; 32]))
};

struct HeapBytes([u8; 32]);

impl titso_core::SecBytes for HeapBytes {
    fn get_and_unlock(&self) -> &[u8; 32] {
        &self.0
    }

    fn get_mut_and_unlock(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }

    fn lock(&self) {}
}

impl Titso {
    async fn init() -> JsResult<Titso> {
        let window = web_sys::window().ok_or("not found window")?;
        let document = window.document().ok_or("not found document")?;

        let layout = Layout::new(&document)?;
        let core = RefCell::new(None);

        debug!("layout ready");

        let db = IndexedKv::open(&window, "titso").await?;

        debug!("db ready");

        Ok(Titso {
            window, layout, core, db,
            password: RefCell::new(Password::new()),
            defense: Lock::new()
        })
    }
}

#[wasm_bindgen]
pub fn start() {
    #[inline]
    async fn start2() -> JsResult<()> {
        let titso = Titso::init().await?;
        let titso = Rc::new(titso);
        let titso2 = titso.clone();

        titso.layout.hook(titso2);

        debug!("hook ready");

        titso.layout.unlock.password.set_disabled(false);

        Ok(())
    }

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();

    debug!("start");

    spawn_local(async {
        start2().await.unwrap();
    });
}
