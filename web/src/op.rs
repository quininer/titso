use std::mem;
use log::debug;
use kvdb_web::KeyValueDB;
use seckey::{ TempKey, zero };
use titso_core::Titso as Core;
use titso_core::primitive::rng::HashRng;
use titso_core::packet::{ Tag, Item, Rule, Type };
use crate::error::JsResult;
use crate::Titso;


pub fn unlock_submit(titso: &Titso) -> JsResult<()> {
    debug!("unlock start");

    let mut secret = titso.db.get(0, b"secret")?
        .ok_or("not found secret")?;
    let secret = TempKey::new(secret.as_mut_slice());
    let mut password = titso.layout.unlock.password
        .value()
        .into_bytes();

    if password.is_empty() {
        titso.window.alert_with_message("password is empty")?;
        return Ok(());
    }

    let password = TempKey::new(password.as_mut_slice());

    *titso.core.borrow_mut() = Some(Core::open(&password, &secret)?);

    titso.layout.unlock.password.set_value("");
    titso.layout.unlock.page.set_hidden(true);
    titso.layout.query.page.set_hidden(false);
    titso.layout.query.input.focus()?;

    debug!("unlock ok");
    Ok(())
}

pub fn query_clear(titso: &Titso) {
    titso.layout.query.input.set_value("");
}

pub fn query_submit(titso: &Titso) -> JsResult<()> {
    enum QueryState {
        Render {
            password: String,
            rule: Option<Rule>,
            note: String
        },
        Nothing,
        New
    }

    impl Drop for QueryState {
        fn drop(&mut self) {
            if let QueryState::Render { password, note, .. } = self {
                let mut pass = mem::take(password).into_bytes();
                let mut note = mem::take(note).into_bytes();

                zero(&mut pass);
                zero(&mut note);
            }
        }
    }

    debug!("query start");

    let tags = titso.layout.query.input.value();
    let tags = take_tags(&tags);

    if tags.is_empty() {
        titso.window.alert_with_message("tags is empty")?;
        return Ok(())
    }

    let mut core = titso.core.borrow_mut();
    let core = core
        .as_mut()
        .ok_or("titso core does not exist")?;

    let state = core.execute(|core| -> JsResult<QueryState> {
        let Tag(tag) = core.store_tag(&tags);

        Ok(if let Some(val) = titso.db.get(0, &tag)? {
            match core.get(&tags, &val) {
                Ok(item) => match item.password {
                    Type::Derive(rule) => QueryState::Render {
                        password: core.derive(&tags, &rule),
                        rule: Some(rule),
                        note: item.note
                    },
                    Type::Fixed(pass) => QueryState::Render {
                        password: pass,
                        rule: None,
                        note: item.note
                    }
                },
                Err(err) => {
                    let msg = format!("query failed: {:?}", err);
                    titso.window.alert_with_message(&msg)?;
                    QueryState::Nothing
                }
            }
        } else {
            QueryState::New
        })
    })?;

    match &state {
        QueryState::Render { password, rule, note } => {
            if let Some(rule) = rule {
                let chars: String = rule.chars.iter().copied().collect();

                titso.layout.query.show.fixed.set_checked(false);
                titso.layout.query.show.rule.count.set_value_as_number(rule.count as _);
                titso.layout.query.show.rule.chars.set_value(&chars);
                titso.layout.query.show.rule.len.set_value_as_number(rule.length as _);
                titso.layout.query.show.rule.page.set_hidden(false);
            } else {
                titso.layout.query.show.fixed.set_checked(true);
                titso.layout.query.show.rule.page.set_hidden(true);
            }

            titso.layout.query.show.password.set_value(&password);
            titso.layout.query.show.note.set_value(&note);
            titso.layout.query.show.page.set_hidden(false);
        },
        QueryState::Nothing => (),
        QueryState::New => {
            debug!("not found");

            let rule = Rule::default();
            let chars = rule.chars.into_iter().collect::<String>();

            titso.layout.query.show.fixed.set_checked(false);
            titso.layout.query.show.rule.count.set_value_as_number(rule.count as _);
            titso.layout.query.show.rule.chars.set_value(&chars);
            titso.layout.query.show.rule.len.set_value_as_number(rule.length as _);
            titso.layout.query.show.rule.page.set_hidden(false);
            titso.layout.query.show.password.set_value("");
            titso.layout.query.show.note.set_value("");
            titso.layout.query.show.page.set_hidden(false);
        }
    }

    debug!("query ok");
    Ok(())
}

pub fn switch_fixed(titso: &Titso) {
    let use_fixed = titso.layout.query.show.fixed.checked();
    titso.layout.query.show.rule.page.set_hidden(use_fixed);
}

pub fn change_password(titso: &Titso) -> JsResult<()> {
    struct TempItem(Item);

    impl Drop for TempItem {
        fn drop(&mut self) {
            if let Type::Fixed(pass) = &mut self.0.password {
                let mut pass = mem::take(pass).into_bytes();
                zero(&mut pass);
            }

            let mut note = mem::take(&mut self.0.note).into_bytes();
            zero(&mut note);
        }
    }

    debug!("change start");

    let tags = titso.layout.query.input.value();
    let tags = take_tags(&tags);

    if tags.is_empty() {
        titso.window.alert_with_message("tags is empty")?;
        return Ok(())
    }

    let mut core = titso.core.borrow_mut();
    let core = core
        .as_mut()
        .ok_or("titso core does not exist")?;

    let item = TempItem(if titso.layout.query.show.fixed.checked() {
        let password = titso.layout.query.show.password.value();
        let note = titso.layout.query.show.note.value();

        Item {
            password: Type::Fixed(password),
            padding: Vec::new(),
            note,
        }
    } else {
        let count = titso.layout.query.show.rule.count.value_as_number() as u64;
        let chars = titso.layout.query.show.rule.chars
            .value()
            .chars()
            .collect();
        let len = titso.layout.query.show.rule.len.value_as_number() as u16;
        let note = titso.layout.query.show.note.value();

        Item {
            password: Type::Derive(Rule { count, chars, length: len }),
            padding: Vec::new(),
            note
        }
    });

    let (tag, val) = core.execute(|core| -> JsResult<([u8; 16], Vec<u8>)> {
        let Tag(tag) = core.store_tag(&tags);
        let val = core.put(&tags, &item.0)?;
        Ok((tag, val))
    })?;

    let mut t = titso.db.transaction();
    t.put(0, &tag[..], &val);
    titso.db.write(t)?;

    debug!("change ok");
    Ok(())
}

pub fn show_password(titso: &Titso) {
    let ty = titso.layout.query.show.password.type_();
    match ty.as_str() {
        "password" => {
            titso.layout.query.show.password.set_type("text");
            titso.layout.query.show.password.select();
        },
        _ => titso.layout.query.show.password.set_type("password")
    }
}

pub fn lock_page(titso: &Titso) {
    titso.core.borrow_mut().take();

    titso.layout.query.page.set_hidden(true);
    titso.layout.query.show.page.set_hidden(true);
    titso.layout.unlock.page.set_hidden(false);
    titso.layout.query.input.set_value("");
    titso.layout.query.show.password.set_value("");
    titso.layout.query.show.note.set_value("");
}

pub fn create_new_profile(titso: &Titso) -> JsResult<()> {
    debug!("create start");

    if titso.db.get(0, b"secret")?.is_some() {
        titso.window.alert_with_message("The profile already exists!")?;
        return Ok(())
    }

    let mut password = titso.layout.unlock.password
        .value()
        .into_bytes();

    if password.is_empty() {
        titso.window.alert_with_message("password is empty")?;
        return Ok(());
    }

    let password = TempKey::new(password.as_mut_slice());

    let mut rng = HashRng::random()?;
    let (core, mut secret) = Core::init(&mut rng, &password)?;
    let secret = TempKey::new(&mut secret);

    let mut t = titso.db.transaction(); // FIXME oh leak secret
    t.put(0, b"secret", &secret);
    titso.db.write(t)?;

    *titso.core.borrow_mut() = Some(core);

    titso.layout.unlock.password.set_value("");
    titso.layout.unlock.page.set_hidden(true);
    titso.layout.query.page.set_hidden(false);

    debug!("create ok");
    Ok(())
}

fn take_tags(tags: &str) -> Vec<&str> {
    let mut tags = tags
        .split_whitespace()
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    tags
}
