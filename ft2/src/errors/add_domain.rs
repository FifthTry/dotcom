#[derive(Debug)]
pub enum AddDomainError {
    IsEmpty,                     // domain-is-empty
    MalformedDomain,             // domain-regex
    DomainAlreadyExists(String), // unique-site-slug // unique-domain
}

impl crate::errors::FieldError for AddDomainError {
    fn field_name(&self) -> &'static str {
        "domain"
    }
}

impl ft_common::TranslatedString for AddDomainError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            AddDomainError::MalformedDomain => "Domain must be lowercase and can only contain letters (a-z), numbers (0-9), and hyphens (-). Internationalized Domain Names (IDN) are not supported at this time.".to_string(),
            AddDomainError::IsEmpty => "Domain is required.".to_string(),
            AddDomainError::DomainAlreadyExists(domain) => {
                format!("Site with this domain {domain} already exists.")
            }
        }
    }
}

impl crate::errors::ToActionError for AddDomainError {
    fn to_action_error(&self) -> ft_common::ActionError {
        use crate::errors::FieldError;
        use ft_common::TranslatedString;

        ft_common::ActionError::single_error(
            self.field_name(),
            &self.to_string(&ft_common::Language::default()),
        )
    }
}
