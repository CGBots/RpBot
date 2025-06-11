use crate::translation;

pub struct Data {
    #[allow(dead_code)]
    pub(crate) translations: translation::Translations,
}

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;