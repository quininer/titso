use log::debug;
use wasm_bindgen_futures::JsFuture;
use js_sys::{ Array, ArrayBuffer, Uint8Array };
use web_sys::{ Url, File };
use serde_bytes::{ Bytes, ByteBuf };
use titso_core::Titso as Core;
use titso_core::primitive::rng::HashRng;
use titso_core::packet::{ Tag, Item, Rule, Type };
use crate::error::JsResult;
use crate::common::{ take_tags, padding };
use crate::Titso;


pub fn input_password(titso: &Titso, key: Option<u8>) {
    let mut password = titso.password.borrow_mut();

    if let Some(c) = key {
        let _ = password.push(c);
    } else {
        password.backspace();
    }
}

pub async fn unlock_submit(titso: &Titso) -> JsResult<()> {
    debug!("unlock start");

    let _guard = titso.defense.acquire()?;

    let secret = titso.db.get(b"secret").await?;
    let secret = if secret.length() > 0 {
        secret.to_vec()
    } else {
        titso.window.alert_with_message("not found secret")?;
        return Ok(());
    };

    let mut password = titso.password.borrow_mut();
    let password = password.take();

    if password.is_empty() {
        titso.window.alert_with_message("password is empty")?;
        return Ok(());
    }

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

pub async fn query_submit(titso: &Titso) -> JsResult<()> {
    enum QueryState {
        Render {
            password: String,
            rule: Option<Rule>,
            note: String
        },
        Nothing,
        New
    }

    debug!("query start");

    let _guard = titso.defense.acquire()?;

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

    let Tag(tag) = {
        let core = core.ready();
        core.store_tag(&tags)
    };

    let core = core.ready();

    let val = titso.db.get(&tag).await?;
    let state = if val.length() > 0 {
        let val = val.to_vec();
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
    };

    drop(core);

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
    titso.layout.query.show.password.set_read_only(!use_fixed);
}

pub async fn edit_item(titso: &Titso) -> JsResult<()> {
    edit_item2(titso).await?;
    query_submit(titso).await?;
    Ok(())
}

async fn edit_item2(titso: &Titso) -> JsResult<()> {
    debug!("edit start");

    let _guard = titso.defense.acquire()?;

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

    let item = if titso.layout.query.show.fixed.checked() {
        let password = titso.layout.query.show.password.value();
        let note = titso.layout.query.show.note.value();

        Item {
            password: Type::Fixed(password),
            padding: padding(),
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
            padding: padding(),
            note
        }
    };

    let core = core.ready();

    let Tag(tag) = core.store_tag(&tags);
    let val = core.put(&tags, &item)?;

    drop(core);

    titso.db.put(&tag[..], Uint8Array::from(&val[..])).await?;

    debug!("edit ok");
    Ok(())
}

pub async fn delete_item(titso: &Titso) -> JsResult<()> {
    debug!("delete start");

    let _guard = titso.defense.acquire()?;

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

    let Tag(tag) = {
        let core = core.ready();
        core.store_tag(&tags)
    };

    titso.db.del(&tag[..]).await?;

    titso.layout.query.show.password.set_value("");
    titso.layout.query.show.note.set_value("");

    debug!("delete ok");
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

pub async fn create_new_profile(titso: &Titso) -> JsResult<()> {
    debug!("create start");

    let _guard = titso.defense.acquire()?;

    if titso.db.get(b"secret").await?.length() > 0 {
        titso.window.alert_with_message("The profile already exists!")?;
        return Ok(())
    }

    let mut password = titso.password.borrow_mut();
    let password = password.take();

    if password.is_empty() {
        titso.window.alert_with_message("password is empty")?;
        return Ok(());
    }

    let mut rng = HashRng::random()?;
    let (core, secret) = Core::init(&mut rng, &password)?;

    titso.db.put(b"secret", Uint8Array::from(&secret[..])).await?;

    *titso.core.borrow_mut() = Some(core);

    titso.layout.unlock.password.set_value("");
    titso.layout.unlock.page.set_hidden(true);
    titso.layout.query.page.set_hidden(false);

    debug!("create ok");
    Ok(())
}

pub async fn import_secret(titso: &Titso) -> JsResult<()> {
    debug!("import secret start");

    let _guard = titso.defense.acquire()?;

    if titso.db.get(b"secret").await?.length() > 0 {
        titso.window.alert_with_message("The profile already exists!")?;
        return Ok(())
    }

    let fd = titso.layout.profile.import_secret_file
        .files()
        .and_then(|files| files.get(0))
        .ok_or("not found secret file")?;

    let buf = JsFuture::from(fd.array_buffer()).await?;
    let buf: ArrayBuffer = buf.into();
    let buf = Uint8Array::new(&buf);

    titso.db.put(b"secret", buf).await?;

    debug!("import secret ok");
    Ok(())
}

pub async fn export_secret(titso: &Titso) -> JsResult<()> {
    debug!("export secret start");
    let _guard = titso.defense.acquire()?;

    let buf = titso.db.get(b"secret").await?;

    if buf.length() == 0 {
        titso.window.alert_with_message("The profile no exists!")?;
        return Ok(())
    }

    let arr = Array::new();
    arr.push(buf.as_ref());
    let fd = File::new_with_u8_array_sequence(
        arr.as_ref(),
        "titso-secret.bin"
    )?;
    let url = Url::create_object_url_with_blob(fd.as_ref())?;

    titso.window.open_with_url(&url)?;

    debug!("export secret ok");
    Ok(())
}

pub async fn import_store(titso: &Titso) -> JsResult<()> {
    type StoreList<'a> = Vec<(&'a Bytes, &'a Bytes)>;

    debug!("import store start");
    let _guard = titso.defense.acquire()?;

    let fd = titso.layout.profile.import_store_file
        .files()
        .and_then(|files| files.get(0))
        .ok_or("not found secret file")?;

    let buf = JsFuture::from(fd.array_buffer()).await?;
    let buf: ArrayBuffer = buf.into();
    let buf = Uint8Array::new(&buf);
    let buf = buf.to_vec();

    let storelist: StoreList = serde_cbor::from_slice(&buf)?;

    for (k, v) in storelist {
        let v = Uint8Array::from(v.as_ref());
        titso.db.put(k.as_ref(), v).await?;
    }

    debug!("import store ok");
    Ok(())
}

pub async fn export_store(titso: &Titso) -> JsResult<()> {
    type StoreList = Vec<(ByteBuf, ByteBuf)>;

    debug!("export store start");
    let _guard = titso.defense.acquire()?;

    let mut iter = titso.db.find(b"").await?;
    let mut storelist: StoreList = Vec::with_capacity(8);

    while let Some((k, v)) = iter.next().await? {
        if k == b"secret" {
            continue
        }

        let k = ByteBuf::from(k);
        let v = ByteBuf::from(v.to_vec());
        storelist.push((k, v));
    }

    if storelist.is_empty() {
        titso.window.alert_with_message("The storelist is empty!")?;
        return Ok(())
    }

    let buf = serde_cbor::to_vec(&storelist)?;
    let buf = Uint8Array::from(buf.as_slice());
    let arr = Array::new();
    arr.push(buf.as_ref());
    let fd = File::new_with_u8_array_sequence(
        arr.as_ref(),
        "titso-storelist.bin"
    )?;
    let url = Url::create_object_url_with_blob(fd.as_ref())?;

    titso.window.open_with_url(&url)?;

    debug!("export store start");
    Ok(())
}
