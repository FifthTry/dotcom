mod add_domain;
pub use add_domain::AddDomainError;

mod common;
pub use common::{ManageSiteError, OrgManagementAccessError};

mod delete_domain;
pub use delete_domain::DeleteDomainError;

mod make_primary_domain;
pub use make_primary_domain::MakePrimaryDomainError;

mod github;
pub use github::{GithubRepoFieldError, GithubRepoField};

mod create_token;
pub use create_token::CreateTokenError;

mod delete_token;
pub use delete_token::DeleteTokenError;

pub trait FieldError: ft_common::TranslatedString + std::fmt::Debug {
    fn field_name(&self) -> &'static str;
}

pub trait ToActionError: ft_common::TranslatedString + ft2::errors::FieldError {
    fn to_action_error(&self) -> ft_common::ActionError;
}
