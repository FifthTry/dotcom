#[derive(Debug)]
pub enum MakePrimaryDomainError {
    DomainNotFound, // domain-is-present
    DomainNotVerified, // domain-is-verified
}

impl ft2::errors::FieldError for MakePrimaryDomainError {
    fn field_name(&self) -> &'static str {
        "domain"
    }
}

impl ft_common::TranslatedString for MakePrimaryDomainError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            MakePrimaryDomainError::DomainNotFound => {
                "There is no such domain. Maybe the domain for your site has been deleted."
                    .to_string()
            }
            MakePrimaryDomainError::DomainNotVerified => {
                "Domain is not verified yet can't make it primary. Please try again later."
                    .to_string()
            }
        }
    }
}

impl ft2::errors::ToActionError for MakePrimaryDomainError {
    fn to_action_error(&self) -> ft2::ActionError {
        use ft2::errors::FieldError;
        use ft_common::TranslatedString;

        ft2::ActionError::single_error(
            self.field_name(),
            &self.to_string(&ft_common::Language::default()),
        )
    }
}
