use crate::i18n::Locale;

const SESSION_STORAGE_KEY: &str = "dnsmasqweb_session";
const LOCALE_STORAGE_KEY: &str = "dnsmasqweb_locale";

pub(super) fn load_locale() -> Locale {
    let Some(window) = web_sys::window() else {
        return Locale::default();
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return Locale::default();
    };
    storage
        .get_item(LOCALE_STORAGE_KEY)
        .ok()
        .flatten()
        .as_deref()
        .map(Locale::from)
        .unwrap_or_default()
}

pub(super) fn save_locale(locale: Locale) {
    if let Some(window) = web_sys::window()
        && let Ok(Some(storage)) = window.local_storage()
    {
        let _ = storage.set_item(LOCALE_STORAGE_KEY, locale.code());
    }
}

pub(super) fn load_session_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(SESSION_STORAGE_KEY).ok().flatten()
}

pub(super) fn save_session_token(token: Option<&str>) {
    if let Some(window) = web_sys::window()
        && let Ok(Some(storage)) = window.local_storage()
    {
        match token {
            Some(token) => {
                let _ = storage.set_item(SESSION_STORAGE_KEY, token);
            }
            None => {
                let _ = storage.remove_item(SESSION_STORAGE_KEY);
            }
        }
    }
}
