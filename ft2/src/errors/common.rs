#[derive(Debug)]
pub enum ManageSiteError {
    UnknownSite, // unknown-site
    OrgManagementAccessError(ft2::errors::OrgManagementAccessError),
}

impl ft2::errors::FieldError for ManageSiteError {
    fn field_name(&self) -> &'static str {
        "manage-site"
    }
}

impl ft_common::TranslatedString for ManageSiteError {
    fn to_string(&self, lang: &ft_common::Language) -> String {
        match self {
            ManageSiteError::UnknownSite => "There is no such site. Maybe the slug of the site has changed, or site has been deleted.".to_string(),
            ManageSiteError::OrgManagementAccessError(e) => e.to_string(lang),
        }
    }
}

#[derive(Debug)]
pub enum OrgManagementAccessError {
    NoSuchOrg,        // no-such-org
    NotOrgMember,     // not-org-member
    UnauthorizedRole, // unauthorized-role
}

impl ft2::errors::FieldError for OrgManagementAccessError {
    fn field_name(&self) -> &'static str {
        "manage-org"
    }
}

impl ft_common::TranslatedString for OrgManagementAccessError {
    fn to_string(&self, _lang: &ft_common::Language) -> String {
        match self {
            OrgManagementAccessError::NoSuchOrg => "No such organisation. Maybe the slug of the organisation has changed, or organisation has been deleted.",
            OrgManagementAccessError::NotOrgMember => "Only organization members can perform this action. You may have been removed.",
            OrgManagementAccessError::UnauthorizedRole => "Only roles admin or manager can perform this action. Maybe you lost your role after loading the page.",
        }.to_string()
    }
}
