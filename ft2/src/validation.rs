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
        return Err(ft2::errors::AddDomainError::MalformedDomain.to_action_error());
    }

    Ok(domain)
}
