use crate::i18n::{Locale, Msg, t};

pub(super) enum NoticeMessage {
    Localized(Msg),
    LocalizedDetail { msg: Msg, detail: String },
    Raw(String),
}

impl NoticeMessage {
    pub(super) fn render(&self, locale: Locale) -> String {
        match self {
            Self::Localized(msg) => t(locale, *msg).into(),
            Self::LocalizedDetail { msg, detail } if detail.is_empty() => t(locale, *msg).into(),
            Self::LocalizedDetail { msg, detail } => {
                format!("{}: {}", t(locale, *msg), detail)
            }
            Self::Raw(message) => message.clone(),
        }
    }
}
