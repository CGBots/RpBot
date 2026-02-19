use poise::{async_trait};
#[allow(unused_imports)]
use serenity::all::{ChannelType, CreateChannel, Context, Guild, Ready};
use poise::serenity_prelude::{EventHandler};
#[cfg(test)] use crate::discord::connect_bot::TEST_PASSED;

#[allow(unused_imports)]
#[cfg(not(test))] use std::ops::Add;
#[allow(unused_imports)]
#[cfg(not(test))] use serenity::all::ActivityData;
#[allow(unused_imports)]
use crate::translation::{apply_translations, tr};

/// The `Handler` struct serves as a placeholder or marker in this context.
///
/// This struct may be used to define behavior, facilitate functionality, or act as
/// a component in a larger system. Currently, it doesn't hold any data or
/// implement any methods but can be extended to include specific functionality
/// as required.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let handler = Handler;
/// // Additional logic or functionality can be added here
/// ```
///
/// This structure can be customized and expanded as necessary to meet the needs
/// of the application.
pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    ///  Handles the `ready` event in an asynchronous context for testing purposes.
    ///
    ///  This function is executed when the bot successfully connects to Discord
    ///  during tests. It logs a message indicating the bot's connection status
    ///  and modifies a shared `TEST_PASSED` Mutex to reflect that the event
    ///  handler has been executed.
    ///
    ///  # Arguments
    ///
    ///  * `self` - The instance of the struct this function is a part of.
    ///  * `_ctx` - The context of the event, which contains data and utilities
    ///             required for the event handling. It is not used in this function.
    ///  * `ready` - The `Ready` struct, which contains information about the
    ///              bot's connection, such as the bot user's details.
    ///
    ///  # Behavior
    ///
    ///  - Prints a message to the console confirming the bot's connection
    ///    and the associated bot user's name.
    ///  - Attempts to acquire a lock on the `TEST_PASSED` Mutex:
    ///       - If successful, it pushes `true` to the front of the linked list
    ///         inside the Mutex.
    ///       - If an error occurs while acquiring the lock, prints the error.
    ///
    ///  # Notes
    ///
    ///  - This function is conditionally compiled and will only be available
    ///    when the `test` configuration is enabled (e.g., during unit/integration tests).
    ///  - Ensure that the `TEST_PASSED` Mutex is properly initialized before use to
    ///    avoid runtime issues.
    ///  - Any errors encountered when locking the Mutex will only be logged to the
    ///    console; they are not propagated further.
    #[cfg(test)]
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        match TEST_PASSED.lock(){
            Ok(mut mutex) => {mutex.push_front(true)}
            Err(e) => {println!("{:?}", e)}
        }
    }
}