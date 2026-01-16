use client::Error;
use client::core::Status;

use super::*;

/// This helper is useful when you call .delete() and want to ignore "not found" errors
/// the NOT_FOUND status will be converted into an Ok variant with a Status object
///
/// Example:
/// ```no_run
/// # use kube_client_ext::ignore_not_found;
/// # use kube_client_ext::KubeClientExt;
/// # use kube_client::Client;
///
/// async fn delete_and_ignore_not_found(client: &Client) -> bool {
///     client
///         .namespaces()
///         .delete("nonexistent", &client.delete_params())
///         .await
///         .or_else(ignore_not_found)
///         .is_ok()
/// }
/// ```
pub fn not_found_ok<K>(err: Error) -> client::Result<either::Either<K, Status>> {
    match err {
        Error::Api(status) if status.is_not_found() => Ok(either::Right(*status)),
        other => Err(other),
    }
}

#[deprecated(since = "3.0.1", note = "use `not_found_ok` instead")]
pub fn ignore_not_found<K>(err: Error) -> client::Result<either::Either<K, Status>> {
    not_found_ok(err)
}
