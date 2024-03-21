#[derive(Debug)]
pub enum DeleteDomainError {
    DomainNotFound, // domain-is-present
    CannotDeletePrimaryDomain, // cannot-delete-primary-domain
    CannotDeleteFifthTryDomain, // cannot-delete-fifthtry-domain
}

impl ft2::errors::FieldError for DeleteDomainError {
    fn field_name(&self) -> &'static str {
        "domain"
    }
}

impl ft_common::TranslatedString for DeleteDomainError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            DeleteDomainError::DomainNotFound => {
                "There is no such domain. Maybe the domain for your site has been deleted."
                    .to_string()
            }
            DeleteDomainError::CannotDeletePrimaryDomain => {
                "You can't delete the primary domain of a site.".to_string()
            }
            DeleteDomainError::CannotDeleteFifthTryDomain => {
                "You can't delete the FifthTry domain of a site.".to_string()
            }
        }
    }
}

impl ft2::errors::ToActionError for DeleteDomainError {
    fn to_action_error(&self) -> ft_common::ActionError {
        use ft2::errors::FieldError;
        use ft_common::TranslatedString;

        ft_common::ActionError::single_error(
            self.field_name(),
            &self.to_string(&ft_common::Language::default()),
        )
    }
}
