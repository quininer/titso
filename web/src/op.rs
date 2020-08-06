use web_sys::Event;
use kvdb_web::{ Database, KeyValueDB };
use seckey::TempKey;
use titso_core::Titso as Core;
use titso_core::packet::Tag;
use crate::error::JsResult;
use crate::Titso;


pub fn unlock_submit(titso: &Titso, _event: &Event) -> JsResult<()> {
    let mut secret = titso.db.get(0, b"secret")?
        .ok_or("not found secret")?;
    let secret = TempKey::new(secret.as_mut_slice());
    let password = titso.layout.unlock.password.value();

    if password.is_empty() {
        titso.window.alert_with_message("password is empty!")?;
        return Ok(());
    }

    let mut password = password.into_bytes();
    let password = TempKey::new(password.as_mut_slice());

    *titso.core.borrow_mut() = Some(Core::open(&password, &secret)?);

    titso.layout.unlock.page.set_hidden(true);
    titso.layout.query.page.set_hidden(false);

    Ok(())
}

pub fn query_submit(titso: &Titso, _event: &Event) -> JsResult<()> {
    let tags = titso.layout.query.input.value();
    let tags = tags
        .split_whitespace()
        .collect::<Vec<_>>();

    let mut core = titso.core.borrow_mut();
    let core = core
        .as_mut()
        .ok_or("titso core does not exist")?;

    core.execute(|core| -> JsResult<()> {
        let Tag(tag) = core.store_tag(&tags);
        titso.db.get(0, &tag)?;

        todo!();
    })?;

    todo!()
}
