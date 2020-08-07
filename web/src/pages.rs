use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{ Document, HtmlElement, HtmlInputElement, HtmlButtonElement, HtmlTextAreaElement, KeyboardEvent };
use gloo_events::EventListener;
use crate::error::{ JsResult, cast_failed };
use crate::{ op, Titso };


pub struct Layout {
    pub unlock: UnlockPage,
    pub query: QueryPage,
    pub profile: ProfilePage
}

pub struct UnlockPage {
    pub page: HtmlElement,
    pub password: HtmlInputElement,
    pub color: HtmlElement,
    pub submit: HtmlButtonElement
}

pub struct QueryPage {
    pub page: HtmlElement,
    pub input: HtmlInputElement,
    pub show: ShowPage
}

pub struct ShowPage {
    pub page: HtmlElement,
    pub fixed: HtmlInputElement,
    pub rule: RulePage,
    pub password: HtmlInputElement,
    pub note: HtmlTextAreaElement,
    pub change: HtmlButtonElement
}

pub struct RulePage {
    pub page: HtmlElement,
    pub count: HtmlInputElement,
    pub chars: HtmlInputElement,
    pub len: HtmlInputElement,
}

pub struct ProfilePage {
    pub page: HtmlElement,
    pub import: HtmlInputElement,
    pub export: HtmlButtonElement,
    pub create: HtmlButtonElement
}

impl Layout {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(Layout {
            unlock: UnlockPage::new(document)?,
            query: QueryPage::new(document)?,
            profile: ProfilePage::new(document)?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        self.unlock.hook(titso.clone())?;
        self.query.hook(titso.clone())?;
        self.profile.hook(titso)
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

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        EventListener::new(
            self.password.as_ref(),
            "keydown",
            move |event| {
                let key = event.dyn_ref::<KeyboardEvent>().map(|ev| ev.key());
                let key = key.as_ref().map(String::as_str);
                match key {
                    Some("Enter") => op::unlock_submit(&titso).unwrap(),
                    _ => ()
                }
            }
        ).forget();

        Ok(())
    }
}

impl QueryPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(QueryPage {
            page: query_selector(document, ".query-page")?,
            input: query_selector(document, ".query-input")?,
            show: ShowPage::new(document)?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        self.show.hook(titso.clone())?;

        EventListener::new(
            self.input.as_ref(),
            "keydown",
            move |event| {
                let key = event.dyn_ref::<KeyboardEvent>().map(|ev| ev.key());
                let key = key.as_ref().map(String::as_str);
                match key {
                    Some("Enter") => op::query_submit(&titso).unwrap(),
                    Some("Esc") | Some("Escape") => op::query_clear(&titso),
                    _ => ()
                }
            }
        ).forget();

        Ok(())
    }
}

impl RulePage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(RulePage {
            page: query_selector(document, ".rule-page")?,
            count: query_selector(document, ".rule-count")?,
            chars: query_selector(document, ".rule-chars")?,
            len: query_selector(document, ".rule-len")?,
        })
    }
}

impl ShowPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(ShowPage {
            page: query_selector(document, ".show-page")?,
            fixed: query_selector(document, ".rule-fixed")?,
            rule: RulePage::new(document)?,
            password: query_selector(document, ".show-password")?,
            note: query_selector(document, ".show-note")?,
            change: query_selector(document, ".submit-change")?,
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        let titso2 = titso.clone();
        let titso3 = titso.clone();

        EventListener::new(
            self.fixed.as_ref(),
            "click",
            move |_event| op::switch_fixed(&titso)
        ).forget();

        EventListener::new(
            self.change.as_ref(),
            "click",
            move |_event| op::change_password(&titso2).unwrap()
        ).forget();

        EventListener::new(
            self.password.as_ref(),
            "click",
            move |_event| op::show_password(&titso3)
        ).forget();

        Ok(())
    }
}

impl ProfilePage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(ProfilePage {
            page: query_selector(document, ".profile-page")?,
            import: query_selector(document, ".import-store")?,
            export: query_selector(document, ".export-store")?,
            create: query_selector(document, ".create-store")?,
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) -> JsResult<()> {
        EventListener::new(
            self.create.as_ref(),
            "click",
            move |event| op::create_new_profile(&titso, event).unwrap()
        ).forget();

        Ok(())
    }
}

fn query_selector<T: JsCast>(document: &Document, input: &str) -> JsResult<T> {
    document.query_selector(input)?
        .ok_or_else(|| format!("not found: {:?}", input))?
        .dyn_into::<T>()
        .map_err(cast_failed)
}
