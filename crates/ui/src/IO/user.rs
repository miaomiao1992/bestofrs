use dioxus::prelude::*;

use crate::impls::session::preference;
use crate::impls::session::AppSession;

#[get("/api/user/locale", session: AppSession, seed: preference::PreferenceSeed)]
pub async fn get_locale() -> ServerFnResult<String> {
    Ok(preference::resolve_locale(&session, &seed))
}

#[post("/api/user/locale/:locale", session: AppSession)]
pub async fn set_locale(locale: String) -> ServerFnResult<()> {
    preference::update_locale(&session, &locale);
    Ok(())
}
