mod add_domain;
pub use add_domain::AddDomainError;

mod common;
pub use common::{ManageSiteError, OrgManagementAccessError};

pub trait FieldError: ft_common::TranslatedString + std::fmt::Debug {
    fn field_name(&self) -> &'static str;
}

pub trait ToActionError: ft_common::TranslatedString + ft2::errors::FieldError {
    fn to_action_error(&self) -> ft_common::ActionError;
}
