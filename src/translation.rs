//! Provides a translation layer using Fluent for internationalization,
//! with automatic fallback and macro-based access for Poise commands.

use std::collections::HashMap;
use std::path::Path;
use crate::{Context, Data, Error};
use fluent::{FluentArgs, FluentValue};
use fluent::bundle::FluentBundle;
use fluent::FluentResource;
use intl_memoizer::concurrent::IntlLangMemoizer;
use regex::Regex;

/// Type alias for a Fluent bundle with concurrent memoization
type Bundle = FluentBundle<FluentResource, IntlLangMemoizer>;

/// Holds all loaded translation bundles, including the default and locale-specific ones.
pub struct Translations {
    pub main: Bundle,
    pub other: HashMap<String, Bundle>,
}

/// Macro to retrieve a translation string, optionally with arguments.
/// Usage:
/// - `tr!(ctx, "identifier")`
/// - `tr!(ctx, "identifier", arg1: VALUE1, arg2: VALUE2)`
#[macro_export]
macro_rules! tr {
    ( $ctx:expr, $id:expr $(, $argname:ident: $argvalue:expr )* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut args = fluent::FluentArgs::new();
        $( args.set(stringify!($argname), $argvalue); )*
        $crate::translation::smart_tr($ctx, $id, Some(&args)).unwrap()
    }};
    ( $ctx:expr, $id:expr ) => {{
        $crate::translation::smart_tr($ctx, $id, None).unwrap()
    }};
}
#[allow(unused_imports)]
pub(crate) use tr;

/// Formats a Fluent message with optional attribute and arguments.
pub fn format(
    bundle: &Bundle,
    id: &str,
    attr: Option<&str>,
    args: Option<&FluentArgs<'_>>,
) -> Option<String> {
    let message = bundle.get_message(id)?;
    let pattern = match attr {
        Some(attribute) => message.get_attribute(attribute)?.value(),
        None => message.value()?,
    };
    Some(bundle.format_pattern(pattern, args, &mut vec![]).into_owned())
}

/// Retrieves a translation string using the user's locale, falling back to the default bundle.
#[allow(unused)]
pub fn get(
    ctx: Context,
    id: &str,
    attr: Option<&str>,
    args: Option<&FluentArgs<'_>>,
) -> String {
    let translations = &ctx.data().translations;
    ctx.locale()
        .and_then(|locale| format(translations.other.get(locale)?, id, attr, args))
        .or_else(|| format(&translations.main, id, attr, args))
        .unwrap_or_else(|| {
            tracing::warn!("Unknown Fluent message identifier `{}`", id);
            id.to_string()
        })
}

/// Loads all `.ftl` files from the `translations/` folder into memory.
pub fn read_ftl() -> Result<Translations, Error> {
    fn read_single_ftl(path: &Path) -> Result<(String, Bundle), Error> {
        let locale = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid .ftl filename")?;

        let file_contents = std::fs::read_to_string(path)?;
        let resource = FluentResource::try_new(file_contents)
            .map_err(|(_, e)| format!("Failed to parse {:?}: {:?}", path, e))?;

        let mut bundle = Bundle::new_concurrent(vec![locale.parse()?]);
        bundle.add_resource(resource)
            .map_err(|e| format!("Failed to add resource to bundle: {:?}", e))?;

        Ok((locale.to_string(), bundle))
    }

    Ok(Translations {
        main: read_single_ftl("translations/en-US.ftl".as_ref())?.1,
        other: std::fs::read_dir("translations")?
            .map(|entry| read_single_ftl(&entry?.path()))
            .collect::<Result<_, _>>()?,
    })
}

/// Applies translations to Poise command definitions, including names, descriptions, and parameters.
pub fn apply_translations(
    translations: &Translations,
    commands: &mut [poise::Command<Data, Error>],
) {
    for command in commands {
        for (locale, bundle) in &translations.other {
            if let Some(name) = format(bundle, &command.name, None, None) {
                command.name_localizations.insert(locale.clone(), name);
                command.description_localizations.insert(
                    locale.clone(),
                    format(bundle, &command.name, Some("description"), None).unwrap(),
                );

                for param in &mut command.parameters {
                    param.name_localizations.insert(
                        locale.clone(),
                        format(bundle, &command.name, Some(&param.name), None).unwrap(),
                    );
                    param.description_localizations.insert(
                        locale.clone(),
                        format(bundle, &command.name, Some(&format!("{}-description", param.name)), None).unwrap(),
                    );
                    for choice in &mut param.choices {
                        choice.localizations.insert(
                            locale.clone(),
                            format(bundle, &choice.name, None, None).unwrap(),
                        );
                    }
                }
            }
        }

        // Fallback to main bundle
        let bundle = &translations.main;
        if let Some(name) = format(bundle, &command.name, None, None) {
            command.name = name;
            command.description = Some(format(bundle, &command.name, Some("description"), None).unwrap());

            for param in &mut command.parameters {
                param.name = format(bundle, &command.name, Some(&param.name), None).unwrap();
                param.description = Some(format(bundle, &command.name, Some(&format!("{}-description", param.name)), None).unwrap());

                for choice in &mut param.choices {
                    choice.name = format(bundle, &choice.name, None, None).unwrap();
                }
            }
        }
    }
}

/// Extracts all variable names like `{$var}` from a Fluent-formatted string.
fn extract_variables_from_pattern(pattern: &str) -> Vec<String> {
    Regex::new(r"\{\$(\w+)\}")
        .unwrap()
        .captures_iter(pattern)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

/// Retrieves a translation and automatically fills missing variables from other Fluent messages.
/// If a variable is not passed explicitly, it will attempt to resolve it using a fallback key
/// with the same name (e.g. `{$create_universe}` → `create_universe`).
pub fn smart_tr(
    ctx: Context,
    id: &str,
    explicit_args: Option<&FluentArgs>,
) -> Result<String, Error> {
    let translations = &ctx.data().translations;
    let bundle = ctx.locale()
        .and_then(|locale| translations.other.get(locale))
        .unwrap_or(&translations.main);

    let message = bundle.get_message(id)
        .or_else(|| translations.main.get_message(id))
        .ok_or_else(|| format!("Missing Fluent message for id `{}`", id))?;

    let pattern = message.value()
        .ok_or_else(|| format!("Message `{}` has no value", id))?;

    let raw_text = bundle.format_pattern(pattern, None, &mut vec![]).into_owned();
    let used_vars = extract_variables_from_pattern(&raw_text);

    let mut args = FluentArgs::new();
    if let Some(explicit) = explicit_args {
        for (k, v) in explicit.iter() {
            args.set(k, v.clone());
        }
    }

    for var in used_vars {
        if args.get(&var).is_none() {
            let fallback_id = var.clone();
            if let Some(value) = format(bundle, &fallback_id, None, None)
                .or_else(|| format(&translations.main, &fallback_id, None, None))
            {
                args.set(var.clone(), FluentValue::from(value));
            } else {
                return Err(format!(
                    "Missing variable `{var}` and fallback `{}` not found",
                    fallback_id
                ).into());
            }
        }
    }

    Ok(bundle.format_pattern(pattern, Some(&args), &mut vec![]).into_owned())
}