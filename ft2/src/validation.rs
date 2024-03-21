// Validation returns msg: domain-regex
pub fn validate_domain(domain: &str) -> Result<String, ft_common::ActionError> {
    use ft2::errors::ToActionError;

    let domain = domain
        .trim()
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .to_string()
        .to_lowercase();

    // Validation returns msg: domain-is-empty
    if domain.is_empty() {
        return Err(ft2::errors::AddDomainError::IsEmpty.to_action_error());
    }

    // Domain Matching Regex
    // This covers most of the cases. We have omit the capital case intentionally
    static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(r"^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*\.[a-z]{2,6}$").unwrap()
    });

    // Validation returns msg: domain-regex
    if !RE.is_match(domain.as_str()) {
        return Err(ft2::errors::AddDomainError::MalformedDomain.to_action_error());
    }

    Ok(domain)
}




// Validation returns msg: github-field-is-not-empty
pub fn validate_github_org(
    value: &str,
) -> Result<String, ft2::errors::GithubRepoFieldError> {
    if value.is_empty() {
        return Err(ft2::errors::GithubRepoFieldError::IsEmpty(
            ft2::errors::GithubRepoField::Organization,
        ));
    }
    Ok(value.to_string())
}

// Validation returns msg: github-field-is-not-empty
pub fn validate_github_repo_name(
    value: &str,
) -> Result<String, ft2::errors::GithubRepoFieldError> {
    if value.is_empty() {
        return Err(ft2::errors::GithubRepoFieldError::IsEmpty(
            ft2::errors::GithubRepoField::RepoName,
        ));
    }
    Ok(value.to_string())
}

// Validation returns msg: github-field-is-not-empty
pub fn validate_github_repo_branch(
    value: &str,
) -> Result<String, ft2::errors::GithubRepoFieldError> {
    if value.is_empty() {
        return Err(ft2::errors::GithubRepoFieldError::IsEmpty(
            ft2::errors::GithubRepoField::RepoBranch,
        ));
    }
    Ok(value.to_string())
}

pub fn validate_token_about(
    about: &str,
) -> Result<String, ft2::errors::CreateTokenError> {
    let about = about.trim().to_string();

    if about.is_empty() {
        return Err(ft2::errors::CreateTokenError::IsEmpty);
    }

    Ok(about)
}

