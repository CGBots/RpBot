use crate::translation;

/// Represents a data structure holding translation information.
///
/// This struct contains a field `translations` which is managed internally within the crate.
///
/// # Fields
///
/// * `translations` - Contains translation-related data, sourced from the `translation::Translations` module.
///   This field is marked with `#[allow(dead_code)]` to suppress warnings about unused code, as it may
///   serve internal purposes within the crate.
///
/// # Visibility
///
/// The `translations` field is restricted to crate-level access (`pub(crate)`),
/// making it inaccessible from outside the crate to ensure encapsulation and control of its usage.
pub struct Data {
    #[allow(dead_code)]
    pub(crate) translations: translation::Translations,
}

/// A type alias for a boxed error that implements the `std::error::Error`
/// trait, as well as the `Send` and `Sync` traits.
///
/// This alias is used to simplify the representation of errors that can be
/// sent across threads and accessed from multiple threads safely. It is
/// commonly used as a generic error type in functions or libraries.
///
/// ## Traits Implemented:
/// - `std::error::Error`: Indicates that this type represents an error.
/// - `Send`: Ensures the error can be transferred across thread boundaries.
/// - `Sync`: Ensures the error can be shared between threads.
///
/// ## Visibility:
/// This type is restricted to the current crate (`pub(crate)`).
///
/// ## Example:
/// ```rust
/// pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
///
/// fn example_function() -> Result<(), Error> {
///     Err("An example error occurred".into())
/// }
/// ```
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

/// A type alias for the context used in the application.
///
/// This alias simplifies referencing the `poise::Context` type with the specific
/// `Data` and `Error` types used in the application.
///
/// # Type Parameters:
/// - `'a`: The lifetime of the context.
///
/// # Underlying Type:
/// `poise::Context<'a, Data, Error>`
///
/// # Visibility:
/// - Restricted to the current crate (`pub(crate)`).
///
/// # Usage:
/// Use this type alias to avoid repeatedly specifying the full `poise::Context`
/// type with the application's custom `Data` and `Error` implementations.
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;