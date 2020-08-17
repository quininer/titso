use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{ Document, HtmlElement, HtmlInputElement, HtmlButtonElement, HtmlAnchorElement, HtmlTextAreaElement, KeyboardEvent };
use gloo_events::{ EventListener, EventListenerOptions };
use crate::error::{ JsResult, cast_failed };
use crate::common::AlertExt;
use crate::{ op, Titso };


pub struct Layout {
    pub unlock: UnlockPage,
    pub query: QueryPage,
    pub profile: ProfilePage
}

pub struct UnlockPage {
    pub page: HtmlElement,
    pub password: HtmlInputElement,
    pub color: HtmlElement
}

pub struct QueryPage {
    pub page: HtmlElement,
    pub input: HtmlInputElement,
    pub show: ShowPage,
    pub lock: HtmlButtonElement,
}

pub struct ShowPage {
    pub page: HtmlElement,
    pub fixed: HtmlInputElement,
    pub rule: RulePage,
    pub password: HtmlInputElement,
    pub note: HtmlTextAreaElement,
    pub submit: HtmlButtonElement,
    pub delete: HtmlButtonElement
}

pub struct RulePage {
    pub page: HtmlElement,
    pub count: HtmlInputElement,
    pub chars: HtmlInputElement,
    pub len: HtmlInputElement,
}

pub struct ProfilePage {
    pub create: HtmlAnchorElement,
    pub import_secret_file: HtmlInputElement,
    pub import_secret: HtmlAnchorElement,
    pub export_secret: HtmlAnchorElement,
    pub import_store_file: HtmlInputElement,
    pub import_store: HtmlAnchorElement,
    pub export_store: HtmlAnchorElement
}

impl Layout {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(Layout {
            unlock: UnlockPage::new(document)?,
            query: QueryPage::new(document)?,
            profile: ProfilePage::new(document)?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) {
        self.unlock.hook(titso.clone());
        self.query.hook(titso.clone());
        self.profile.hook(titso);
    }
}

impl UnlockPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(UnlockPage {
            page: query_selector(document, "#unlock-page")?,
            password: query_selector(document, "#input-password")?,
            color: query_selector(document, "#color-password")?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) {
        let titso2 = titso.clone();

        EventListener::new_with_options(
            self.password.as_ref(),
            "keydown",
            EventListenerOptions::enable_prevent_default(),
            move |event| {
                event.prevent_default();
                let key = event.dyn_ref::<KeyboardEvent>()
                    .map(|ev| ev.key())
                    .map(|key| key.into_bytes());
                let titso = titso.clone();
                match key.as_deref() {
                    Some(b"Enter") => spawn_local(async move {
                        op::unlock_submit(&titso).await.unwrap_alert(&titso)
                    }),
                    Some(b"Backspace") => {
                        op::input_password(&titso, None).unwrap_alert(&titso)
                    },
                    Some(c) if c.len() == 1 && c[0].is_ascii() && !c[0].is_ascii_control() => {
                        op::input_password(&titso, Some(c[0])).unwrap_alert(&titso)
                    },
                    _ => ()
                }
            }
        ).forget();

        EventListener::new(
            self.color.as_ref(),
            "click",
            move |_event| {
                let titso = titso2.clone();

                spawn_local(async move {
                    op::unlock_submit(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();
    }
}

impl QueryPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(QueryPage {
            page: query_selector(document, "#query-page")?,
            input: query_selector(document, "#query-input")?,
            show: ShowPage::new(document)?,
            lock: query_selector(document, "#lock")?
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) {
        self.show.hook(titso.clone());
        let titso2 = titso.clone();

        EventListener::new(
            self.input.as_ref(),
            "keydown",
            move |event| {
                let key = event.dyn_ref::<KeyboardEvent>().map(|ev| ev.key());
                let titso = titso.clone();

                spawn_local(async move {
                    match key.as_deref() {
                        Some("Enter") => op::query_submit(&titso).await.unwrap_alert(&titso),
                        Some("Esc") | Some("Escape") => op::query_clear(&titso),
                        _ => ()
                    }
                })
            }
        ).forget();

        EventListener::new(
            self.lock.as_ref(),
            "click",
            move |_event| op::lock_page(&titso2)
        ).forget();
    }
}

impl RulePage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(RulePage {
            page: query_selector(document, "#rule-page")?,
            count: query_selector(document, "#rule-count")?,
            chars: query_selector(document, "#rule-chars")?,
            len: query_selector(document, "#rule-len")?,
        })
    }
}

impl ShowPage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(ShowPage {
            page: query_selector(document, "#show-page")?,
            fixed: query_selector(document, "#rule-fixed")?,
            rule: RulePage::new(document)?,
            password: query_selector(document, "#show-password")?,
            note: query_selector(document, "#show-note")?,
            submit: query_selector(document, "#submit-item")?,
            delete: query_selector(document, "#delete-item")?,
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) {
        let titso2 = titso.clone();
        let titso3 = titso.clone();
        let titso4 = titso.clone();

        EventListener::new(
            self.fixed.as_ref(),
            "click",
            move |_event| op::switch_fixed(&titso)
        ).forget();

        EventListener::new(
            self.submit.as_ref(),
            "click",
            move |_event| {
                let titso = titso2.clone();

                spawn_local(async move {
                    op::edit_item(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();

        EventListener::new(
            self.delete.as_ref(),
            "click",
            move |_event| {
                let titso = titso3.clone();

                spawn_local(async move {
                    op::delete_item(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();

        EventListener::new(
            self.password.as_ref(),
            "click",
            move |_event| op::show_password(&titso4)
        ).forget();
    }
}

impl ProfilePage {
    pub fn new(document: &Document) -> JsResult<Self> {
        Ok(ProfilePage {
            create: query_selector(document, "#create-secret")?,
            import_secret_file: query_selector(document, "#import-secret-file")?,
            import_secret: query_selector(document, "#import-secret")?,
            export_secret: query_selector(document, "#export-secret")?,
            import_store_file: query_selector(document, "#import-store-file")?,
            import_store: query_selector(document, "#import-store")?,
            export_store: query_selector(document, "#export-store")?,
        })
    }

    pub fn hook(&self, titso: Rc<Titso>) {
        let import_secret_file = self.import_secret_file.clone();
        let import_store_file = self.import_store_file.clone();
        let titso1 = titso.clone();
        let titso2 = titso.clone();
        let titso3 = titso.clone();
        let titso4 = titso.clone();

        EventListener::new(
            self.create.as_ref(),
            "click",
            move |_event| {
                let titso = titso.clone();

                spawn_local(async move {
                    op::create_new_profile(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();

        EventListener::new(
            self.import_secret.as_ref(),
            "click",
            move |_event| import_secret_file.click()
        ).forget();

        EventListener::new(
            self.import_store.as_ref(),
            "click",
            move |_event| import_store_file.click()
        ).forget();

        EventListener::new(
            self.import_secret_file.as_ref(),
            "change",
            move |_event| {
                let titso = titso1.clone();

                spawn_local(async move {
                    op::import_secret(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();

        EventListener::new(
            self.import_store_file.as_ref(),
            "change",
            move |_event| {
                let titso = titso2.clone();

                spawn_local(async move {
                    op::import_store(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();

        EventListener::new(
            self.export_secret.as_ref(),
            "click",
            move |_event| {
                let titso = titso3.clone();

                spawn_local(async move {
                    op::export_secret(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();

        EventListener::new(
            self.export_store.as_ref(),
            "click",
            move |_event| {
                let titso = titso4.clone();

                spawn_local(async move {
                    op::export_store(&titso).await.unwrap_alert(&titso)
                })
            }
        ).forget();
    }
}

fn query_selector<T: JsCast>(document: &Document, input: &str) -> JsResult<T> {
    document.query_selector(input)?
        .ok_or_else(|| format!("not found: {:?}", input))?
        .dyn_into::<T>()
        .map_err(cast_failed)
}
