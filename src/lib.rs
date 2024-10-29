use fluent_templates::{ArcLoader, Loader, fluent_bundle::FluentValue};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

const ERROR_PARSING: &str = "Parsing failed...";
const ERROR_BUILDING: &str = "Failed to build fluent loader...";

lazy_static! {
    pub static ref I18N_ID: String = {
        match std::env::var("I18N_ID") {
            Ok(path) => path,
            _ => "en-US".to_string()
        }
    };

    pub static ref I18N_DIR: String = {
        match std::env::var("I18N_DIR") {
            Ok(path) => path,
            _ => "./".to_string()
        }
    };
}

struct I18n {
    pub loader: ArcLoader,
    pub locales: LanguageIdentifier
}

static I18N: Lazy<I18n> = Lazy::new(|| {
    let locales: LanguageIdentifier = I18N_ID
        .to_string()
        .parse()
        .expect(ERROR_PARSING);

    let loader = ArcLoader::builder(&I18N_DIR.clone(), locales.clone())
        .customize(|b| b.set_use_isolating(false))
        .build()
        .expect(ERROR_BUILDING);

    I18n { locales, loader }
});

pub fn get<T>(key: T) -> String where T: ToString {
    let id = I18N.locales.clone();
    let key = key.to_string();


    I18N.loader.lookup(&id, &key)
}

pub fn args<T, U, V>(key: T, args: &[(U, V)]) -> String
where T: ToString,
      U: ToString,
      V: ToString
{
    let id = I18N.locales.clone();
    let key = key.to_string();

    let array: HashMap<String, FluentValue> = args
        .iter()
        .map(|(k, v)| (
            k.to_string(),
            FluentValue::from(v.to_string())
        ))
        .collect();

    I18N.loader.lookup_with_args(&id, &key, &array)
}