#[derive(Debug)]
pub enum GithubRepoField {
    Organization,
    RepoName,
    RepoBranch,
}

impl GithubRepoField {
    pub fn as_str(&self) -> &'static str {
        match self {
            GithubRepoField::Organization => "github-org-name",
            GithubRepoField::RepoName => "github-repo-name",
            GithubRepoField::RepoBranch => "github-repo-branch",
        }
    }
}

#[derive(Debug)]
pub enum GithubRepoFieldError {
    IsEmpty(GithubRepoField), // field-is-empty
}

impl ft2::errors::FieldError for GithubRepoFieldError {
    fn field_name(&self) -> &'static str {
        match self {
            GithubRepoFieldError::IsEmpty(field) => field.as_str(),
        }
    }
}

impl ft_common::TranslatedString for GithubRepoFieldError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            GithubRepoFieldError::IsEmpty(_) => "This field is required".to_string(),
        }
    }
}

impl ft2::errors::ToActionError for GithubRepoFieldError {
    fn to_action_error(&self) -> ft2::ActionError {
        use ft2::errors::FieldError;
        use ft_common::TranslatedString;

        ft2::ActionError::single_error(
            self.field_name(),
            &self.to_string(&ft_common::Language::default()),
        )
    }
}
