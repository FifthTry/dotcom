pub fn login_url() -> String {
    "/-/auth/login/".to_string()
}

pub fn github_login_url() -> String {
    "/-/auth/github/".to_string()
}

pub fn signup_url() -> String {
    "/-/auth/create-account/".to_string()
}

pub fn logout_url() -> String {
    "/-/auth/logout/".to_string()
}

pub fn org_dashboard_url(org_slug: &str) -> String {
    format!("/o/{org_slug}/")
}

pub fn user_dashboard_url(_username: &str) -> String {
    // we currently do not have public user profile page, so only place you can see
    // a user's site is on their own dashboard.
    "/".to_string()
}

pub fn site_info_url(site_slug: &str) -> String {
    format!("/s/{site_slug}/")
}

pub fn site_editor_url(site_slug: &str) -> String {
    format!("/{site_slug}/edit/")
}

pub fn site_tokens_url(site_slug: &str) -> String {
    format!("/{site_slug}/tokens/")
}

pub fn site_domains_url(site_slug: &str) -> String {
    format!("/{site_slug}/domains/")
}

pub fn site_cr_list_url(site_slug: &str) -> String {
    format!("/{site_slug}/cr-list/")
}

pub fn github_configure_form_url(site_slug: &str) -> String {
    format!("/{site_slug}/configure-github/")
}

pub fn github_url(site_slug: &str) -> String {
    format!("/{site_slug}/github/")
}

pub fn create_org_site_url(org_slug: &str) -> String {
    format!("/o/{org_slug}/create-site/")
}

pub fn create_user_site_url() -> String {
    // even when we have public user pages, the url to create a person site will always be the same
    "/create-site/".to_string()
}

pub fn site_slug_url(site_slug: &str) -> String {
    format!("{site_slug}.fifthtry.site")
}
