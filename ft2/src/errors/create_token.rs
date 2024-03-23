#[derive(Debug)]
pub enum CreateTokenError {
    IsEmpty,
}

impl ft2::errors::FieldError for CreateTokenError {
    fn field_name(&self) -> &'static str {
        "about"
    }
}

impl ft_common::TranslatedString for CreateTokenError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            Self::IsEmpty => "This field is required.".to_string(),
        }
    }
}

impl ft2::errors::ToActionError for CreateTokenError {
    fn to_action_error(&self) -> ft2::ActionError {
        use ft2::errors::FieldError;
        use ft_common::TranslatedString;

        ft2::ActionError::single_error(
            self.field_name(),
            &self.to_string(&ft_common::Language::default()),
        )
    }
}
