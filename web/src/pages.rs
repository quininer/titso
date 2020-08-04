use wasm_bindgen::JsCast;
use web_sys::{ Document, Element, HtmlInputElement, HtmlButtonElement, HtmlTextAreaElement };
use crate::error::{ JsResult, JsError, cast_failed };


pub struct Layout {
    unlock: UnlockPage,
    query: QueryPage
}

pub struct UnlockPage {
    page: Element,
    password: HtmlInputElement,
    color: Element,
    submit: HtmlButtonElement
}

pub struct QueryPage {
    page: Element,
    query: HtmlInputElement,
    show: ShowPage
}

pub struct ShowPage {
    page: Element,
    count: HtmlInputElement,
    chars: HtmlInputElement,
    len: HtmlInputElement,
    password: HtmlInputElement,
    note: HtmlTextAreaElement,
    change: HtmlButtonElement
}

impl Layout {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(Layout {
            unlock: UnlockPage::new(document)?,
            query: QueryPage::new(document)?
        })
    }
}

impl UnlockPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(UnlockPage {
            page: query_selector(document, ".unlock-page")?,
            password: query_selector(document, ".password")?,
            color: query_selector(document, ".color-password")?,
            submit: query_selector(document, ".submit-password")?
        })
    }
}

impl QueryPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(QueryPage {
            page: query_selector(document, ".query-page")?,
            query: query_selector(document, ".query")?,
            show: ShowPage::new(document)?
        })
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
}

fn query_selector<T: JsCast>(document: &Document, input: &str) -> JsResult<T> {
    document.query_selector(input)?
        .ok_or_else(|| format!("not found: {:?}", input))?
        .dyn_into::<T>()
        .map_err(cast_failed)
}
