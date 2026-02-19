use crate::database::server::Server;
use crate::discord::poise_structs::{Context, Error};
use crate::setup_command::complementary_setup::complementary_setup;
use crate::setup_command::partial_setup::partial_setup;

/// Performs a complete asynchronous setup of the server by sequentially running the
/// `partial_setup` and `complementary_setup` functions. If both steps succeed, it returns
/// a confirmation string indicating success.
///
/// # Arguments
/// * `ctx` - A reference to the operational context providing utilities and shared state.
/// * `server` - A mutable reference to the server being configured.
/// * `snapshot` - A snapshot of the server's state used during the setup process.
///
/// # Returns
/// * `Ok(&'static str)` - A string identifier confirming the successful setup.
/// * `Err(Error)` - An error type indicating that one of the setup steps failed.
///
/// # Errors
/// This function forwards any errors returned by `partial_setup` or `complementary_setup`.
///
/// # Examples
/// ```rust
/// // Assuming `ctx`, `server`, and `snapshot` are already initialized:
/// let result = full_setup(&ctx, &mut server, snapshot).await;
/// match result {
///     Ok(success_message) => println!("{}", success_message),
///     Err(e) => eprintln!("Setup failed: {}", e),
/// }
/// ```
pub async fn full_setup<'a>(ctx: &Context<'_>, server: &'a mut Server, snapshot: Server) -> Result<&'static str, Error> {
    partial_setup(ctx, server, snapshot).await?;
    complementary_setup(ctx, server, snapshot).await?;
    Ok("setup__full_setup_success")
}