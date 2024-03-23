#[derive(Debug)]
pub enum DeleteTokenError {
    NotFound,
}

impl ft2::errors::FieldError for DeleteTokenError {
    fn field_name(&self) -> &'static str {
        "token"
    }
}

impl ft_common::TranslatedString for DeleteTokenError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            Self::NotFound => "Token not found to delete!".to_string(),
        }
    }
}

impl ft2::errors::ToActionError for DeleteTokenError {
    fn to_action_error(&self) -> ft2::ActionError {
        use ft2::errors::FieldError;
        use ft_common::TranslatedString;

        ft2::ActionError::single_error(
            self.field_name(),
            &self.to_string(&ft_common::Language::default()),
        )
    }
}
