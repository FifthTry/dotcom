#[allow(dead_code)]
pub fn action_error_to_html(e: ft_common::ActionError) -> http::Response<bytes::Bytes> {
    match e {
        ft_common::ActionError::FormError(errors) => {
            tracing::info!("form error: {errors:?}");
            ft_sdk::json_response(serde_json::json!({"errors": errors}))
        }
        ft_common::ActionError::OrgRateLimitError(message) => {
            ft_sdk::not_found!("org rate limit error: {message:?}")
        }
        ft_common::ActionError::Diesel(e) => {
            ft_sdk::server_error!("diesel error: {e:?}")
        }
        ft_common::ActionError::CantDeserializeInput(message) => {
            ft_sdk::server_error!("serde error: {message:?}")
        }
        ft_common::ActionError::Unauthorized(message) => {
            ft_sdk::not_found!("unauthorized error: {message}")
        }
        ft_common::ActionError::Sdk(e) => {
            ft_sdk::server_error!("sdk error: {e:?}")
        }
        ft_common::ActionError::ServerError { message } => {
            ft_sdk::server_error!("{message}")
        }
        ft_common::ActionError::NotFound { message } => {
            ft_sdk::not_found!("not found error: {message}")
        }
        ft_common::ActionError::UsageError { message } => {
            ft_sdk::not_found!("usage error: {:?}", message)
        }
        ft_common::ActionError::OrgManagementAccessError(message) => {
            ft_sdk::not_found!("org management access error: {message}")
        }
    }
}
