#[allow(dead_code)]
pub fn action_error_to_html(e: ft2::ActionError) -> http::Response<bytes::Bytes> {
    match e {
        ft2::ActionError::FormError(errors) => {
            tracing::info!("form error: {errors:?}");
            ft_sdk::json_response(serde_json::json!({"errors": errors}))
        }
        ft2::ActionError::OrgRateLimitError(message) => {
            ft_sdk::not_found!("org rate limit error: {message:?}")
        }
        ft2::ActionError::Diesel(e) => {
            ft_sdk::server_error!("diesel error: {e:?}")
        }
        ft2::ActionError::CantDeserializeInput(message) => {
            ft_sdk::server_error!("serde error: {message:?}")
        }
        ft2::ActionError::Unauthorized(message) => {
            ft_sdk::not_found!("unauthorized error: {message}")
        }
        ft2::ActionError::Sdk(e) => {
            ft_sdk::server_error!("sdk error: {e:?}")
        }
        ft2::ActionError::ServerError { message } => {
            ft_sdk::server_error!("{message}")
        }
        ft2::ActionError::NotFound { message } => {
            ft_sdk::not_found!("not found error: {message}")
        }
        ft2::ActionError::UsageError { message } => {
            ft_sdk::not_found!("usage error: {:?}", message)
        }
        ft2::ActionError::OrgManagementAccessError(message) => {
            ft_sdk::not_found!("org management access error: {message}")
        }
    }
}
