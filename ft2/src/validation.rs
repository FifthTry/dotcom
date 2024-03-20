// Validation returns msg: domain-regex
pub fn validate_domain(domain: &str) -> Result<String, ft_common::ActionError> {
    use ft_common::errors::ToActionError;

    let domain = domain
        .trim()
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .to_string()
        .to_lowercase();

    // Validation returns msg: domain-is-empty
    if domain.is_empty() {
        return Err(ft_common::errors::AddDomainError::IsEmpty.to_action_error());
    }

    // Understanding regex
    // Where:
    // ^ represents the starting of the string.
    // ( represents the starting of the group.
    // (?!-) represents the string should not start with a hyphen (-).
    // [A-Za-z0–9-]{1, 63} represents the domain name should be a-z or A-Z or 0–9 and hyphen (-) between 1 and 63 characters long.
    // (?<!-) represents the string should not end with a hyphen (-).
    // \\. represents the string followed by a dot.
    // )+ represents the ending of the group, this group must appear at least 1 time, but allowed multiple times for subdomain.
    // [A-Za-z]{2, 6} represents the TLD must be A-Z or a-z between 2 and 6 characters long.
    // $ represents the ending of the string.
    // This covers most of the cases. We have omit the capital case intentionally
    // Todo: if there's any pattern needs to add
    static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(r"^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*\.[a-z]{2,6}$").unwrap()
    });

    // Validation returns msg: domain-regex
    if !RE.is_match(domain.as_str()) {
        return Err(ft_common::errors::AddDomainError::MalformedDomain.to_action_error());
    }

    Ok(domain)
}

pub fn validate_site_slug(site_slug: &str) -> Result<String, ft_common::ActionError> {
    let site_slug = site_slug.trim().to_string();

    // Validation returns msg: site-slug-is-lowercase
    if site_slug != site_slug.to_lowercase() {
        // we explicitly warn user instead of converting it to lower case, because
        // user might be relying on case, and change in case breaks the meaning they
        // are going for, e.g. MissIng and missing are different things, maybe they would
        // prefer miss-ing.fifthtry.site and not missing.fifthtry.site
        return Err(ft_common::ActionError::single_error(
            "site",
            "site-slug should be in lowercase",
        ));
    }

    // Validation returns msg: site-slug-is-not-empty
    if site_slug.is_empty() {
        return Err(ft_common::ActionError::single_error(
            "site",
            "site-slug should not be empty",
        ));
    }

    static RE: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

    // Validation returns msg: site-slug-regex
    if !RE.is_match(site_slug.as_str()) {
        return Err(ft_common::ActionError::single_error(
            "site",
            "site-slug is not valid",
        ));
    }

    // NOTE: We do not check if slug is unique or not here, because we are doing it in
    // the transaction lower down, which is a more reliable check, no need to do the query
    // twice. This is same reason as mentioned in the note in the verify method.

    Ok(site_slug)
}


// Validation returns msg: github-field-is-not-empty
pub fn validate_github_org(
    value: &str,
) -> Result<String, ft_common::errors::GithubRepoFieldError> {
    if value.is_empty() {
        return Err(ft_common::errors::GithubRepoFieldError::IsEmpty(
            ft_common::errors::GithubRepoField::Organization,
        ));
    }
    Ok(value.to_string())
}

// Validation returns msg: github-field-is-not-empty
pub fn validate_github_repo_name(
    value: &str,
) -> Result<String, ft_common::errors::GithubRepoFieldError> {
    if value.is_empty() {
        return Err(ft_common::errors::GithubRepoFieldError::IsEmpty(
            ft_common::errors::GithubRepoField::RepoName,
        ));
    }
    Ok(value.to_string())
}

// Validation returns msg: github-field-is-not-empty
pub fn validate_github_repo_branch(
    value: &str,
) -> Result<String, ft_common::errors::GithubRepoFieldError> {
    if value.is_empty() {
        return Err(ft_common::errors::GithubRepoFieldError::IsEmpty(
            ft_common::errors::GithubRepoField::RepoBranch,
        ));
    }
    Ok(value.to_string())
}

pub fn validate_token_about(
    about: &str,
) -> Result<String, ft_common::errors::CreateTokenError> {
    let about = about.trim().to_string();

    if about.is_empty() {
        return Err(ft_common::errors::CreateTokenError::IsEmpty);
    }

    Ok(about)
}
