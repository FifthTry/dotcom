extern crate self as ft2;

pub mod check;
pub mod errors;
mod file;
mod route;
mod ud;
pub mod validation;
pub mod uuid;
mod member_role;
pub mod site;
pub mod urls;
pub mod insertable;

use ft_common::{FormError, FrontendData};
pub use insertable::*;
pub use file::{file_type, File, FileText, FileTextError, FileType};
pub use route::route;
pub use ud::{GetUDError, UserData};
pub use member_role::MemberRole;

#[no_mangle]
pub extern "C" fn main_ft() {
    let req = ft_sdk::http::current_request();
    let resp = ft2::route(req);
    ft_sdk::http::send_response(resp);
}


#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("cant deserialize input: {0}")]
    CantDeserializeInput(#[from] serde_json::Error),
    #[error("Org Rate Limit error: {0}")]
    OrgRateLimitError(String),
    #[error("not authorised: {0}")]
    Unauthorized(String),
    #[error("form error: {0:?}")]
    FormError(FormError),
    #[error("Not Found: {message}")]
    NotFound { message: String },
    #[error("Usage Error: {message}")] // client
    UsageError { message: String },
    #[error("Org Management Access error: {0}")]
    OrgManagementAccessError(String),
    #[error("server error: {message}")]
    ServerError { message: String },
    #[error("sdk error: {0}")]
    Sdk(#[from] ft_sdk::Error),
}

impl ActionError {
    pub fn single_error(field: &str, error: &str) -> Self {
        ActionError::FormError(std::collections::HashMap::from([(
            field.to_string(),
            error.to_string(),
        )]))
    }
}

#[derive(Debug)]
pub enum ActionResponse {
    Reload,
    Redirect(String),
    Data(FrontendData),
}

pub type ActionResult = Result<ActionResponse, ActionError>;