use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{ Document, HtmlElement, HtmlInputElement, HtmlButtonElement, HtmlTextAreaElement };
use gloo_events::EventListener;
use crate::error::{ JsResult, JsError, cast_failed };
use crate::{ op, Titso };


pub struct Layout {
    pub unlock: UnlockPage,
    pub query: QueryPage
}

pub struct UnlockPage {
    pub page: HtmlElement,
    pub password: HtmlInputElement,
    pub color: HtmlElement
}

pub struct QueryPage {
    pub page: HtmlElement,
    pub input: HtmlInputElement,
    pub show: ShowPage
}

pub struct ShowPage {
    pub page: HtmlElement,
    pub count: HtmlInputElement,
    pub chars: HtmlInputElement,
    pub len: HtmlInputElement,
    pub password: HtmlInputElement,
    pub note: HtmlTextAreaElement,
    pub change: HtmlButtonElement
}

impl Layout {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(Layout {
            unlock: UnlockPage::new(document)?,
            query: QueryPage::new(document)?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        self.unlock.hook(titso.clone())?;
        self.query.hook(titso)
    }
}

impl UnlockPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(UnlockPage {
            page: query_selector(document, ".unlock-page")?,
            password: query_selector(document, ".password")?,
            color: query_selector(document, ".color-password")?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        EventListener::new(
            self.password.as_ref(),
            "submit",
            move |event| op::unlock_submit(&titso, event).unwrap()
        ).forget();

        Ok(())
    }
}

impl QueryPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(QueryPage {
            page: query_selector(document, ".query-page")?,
            input: query_selector(document, ".query")?,
            show: ShowPage::new(document)?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        self.show.hook(titso.clone())?;

        EventListener::new(
            self.input.as_ref(),
            "submit",
            move |event| op::query_submit(&titso, event).unwrap()
        ).forget();

        Ok(())
    }
}

impl ShowPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(ShowPage {
            page: query_selector(document, ".show-page")?,
            count: query_selector(document, ".rule-count")?,
            chars: query_selector(document, ".rule-chars")?,
            len: query_selector(document, ".rule-len")?,
            password: query_selector(document, ".show-password")?,
            note: query_selector(document, ".show-note")?,
            change: query_selector(document, ".submit-change")?,
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        todo!()
    }
}

fn query_selector<T: JsCast>(document: &Document, input: &str) -> JsResult<T> {
    document.query_selector(input)?
        .ok_or_else(|| format!("not found: {:?}", input))?
        .dyn_into::<T>()
        .map_err(cast_failed)
}
