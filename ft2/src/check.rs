// pub mod misc;
// pub mod myself;
// pub mod site;
//
#[derive(thiserror::Error, Debug)]
pub enum OrgManagementAccessError {
    #[error("Org management access error {0:?}")]
    AccessError(ft2::errors::OrgManagementAccessError),
    #[error("Diesel error {0}")]
    Diesel(#[from] diesel::result::Error),
}

pub fn if_org_exists_from_slug(
    org_slug: &str,
    conn: &mut ft_sdk::PgConnection,
) -> Result<i64, OrgManagementAccessError> {
    use ft_common::prelude::*;
    use ft_common::schema::ft_org;

    match ft_org::table
        .filter(ft_org::slug.eq(org_slug))
        .select(ft_org::id)
        .first::<i64>(conn)
    {
        Ok(v) => Ok(v),
        Err(diesel::result::Error::NotFound) => {
            // Validation: no-such-org
            Err(OrgManagementAccessError::AccessError(
                ft2::errors::OrgManagementAccessError::NoSuchOrg,
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub fn if_org_exists_from_id(
    org_id: i64,
    conn: &mut ft_sdk::PgConnection,
) -> Result<(String, String), OrgManagementAccessError> {
    use ft_common::prelude::*;
    use ft_common::schema::ft_org;

    match ft_org::table
        .filter(ft_org::id.eq(org_id))
        .select((ft_org::slug, ft_org::name))
        .first::<(String, String)>(conn)
    {
        Ok(v) => Ok(v),
        Err(diesel::result::Error::NotFound) => {
            // Validation: no-such-org
            Err(OrgManagementAccessError::AccessError(
                ft2::errors::OrgManagementAccessError::NoSuchOrg,
            ))
        }
        Err(e) => Err(e.into()),
    }
}

// returns org member role (if org member exists)
pub fn if_user_is_org_member(
    user_id: i64,
    org_id: i64,
    conn: &mut ft_sdk::PgConnection,
) -> Result<String, OrgManagementAccessError> {
    use ft_common::prelude::*;
    use ft_common::schema::ft_org_member;

    match ft_org_member::table
        .filter(
            ft_org_member::user_id
                .eq(user_id)
                .and(ft_org_member::org_id.eq(org_id)),
        )
        .select(ft_org_member::role)
        .first::<String>(conn)
    {
        Ok(m) => Ok(m),
        Err(diesel::result::Error::NotFound) => {
            // Validation returns msg: not-org-member
            Err(OrgManagementAccessError::AccessError(
                ft2::errors::OrgManagementAccessError::NotOrgMember,
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub fn if_user_is_org_member_and_has_manage_permissions(
    user_id: i64,
    org_id: i64,
    conn: &mut ft_sdk::PgConnection,
) -> Result<(), OrgManagementAccessError> {
    let role_s = match if_user_is_org_member(user_id, org_id, conn) {
        Ok(role) => role,
        Err(e) => return Err(e),
    };

    println!("Found org member with role: {role_s}");
    let role: ft2::MemberRole = role_s.as_str().into();

    if !role.has_manage_permissions() {
        // Validation returns msg: unauthorized-role
        return Err(OrgManagementAccessError::AccessError(
            ft2::errors::OrgManagementAccessError::UnauthorizedRole,
        ));
    }
    Ok(())
}

//
// // Validation: no-such-org, not-org-member, unauthorized-role
// pub async fn check_if_user_can_manage_site(
//     current_user_id: i64,
//     site_created_by: i64,
//     org_id: Option<i64>,
//     conn: &mut ft_common::Conn,
// ) -> Result<Option<String>, OrgManagementAccessError> {
//     let org_id = match org_id {
//         Some(org_id) => org_id,
//         None => {
//             // If org_id is None, then the site is a personal site, and it has to be owned by the
//             // created_by person.
//             if current_user_id != site_created_by {
//                 // Validation returns msg: unauthorized-site
//                 return Err(OrgManagementAccessError::AccessError(
//                     ft_common::errors::OrgManagementAccessError::UnauthorizedRole,
//                 ));
//             }
//             return Ok(None);
//         }
//     };
//
//     let (org_slug, _) = check_if_org_exists_from_id(org_id, conn).await?;
//     check_if_user_is_org_member_and_has_manage_permissions(current_user_id, org_id, conn).await?;
//
//     Ok(Some(org_slug))
// }
//

// Validation: no-such-org, not-org-member, unauthorized-role
pub fn if_user_can_manage_org_using_slug(
    user_id: i64,
    org_slug: &str,
    conn: &mut ft_sdk::PgConnection,
) -> Result<i64, OrgManagementAccessError> {
    let org_id = if_org_exists_from_slug(org_slug, conn)?;
    if_user_is_org_member_and_has_manage_permissions(user_id, org_id, conn)?;

    Ok(org_id)
}

//
// Validation: no-such-org, not-org-member
pub fn if_user_can_view_org_contents(
    user_id: i64,
    org_id: i64,
    conn: &mut ft_sdk::PgConnection,
) -> Result<(i64, String, String), OrgManagementAccessError> {
    let (org_slug, org_name) = if_org_exists_from_id(org_id, conn)?;
    if_user_is_org_member(user_id, org_id, conn)?;

    Ok((org_id, org_slug, org_name))
}
//
// // Validation: no-such-org, not-org-member
// pub async fn check_if_user_can_view_org_contents_using_slug(
//     user_id: i64,
//     org_slug: &str,
//     conn: &mut ft_common::Conn,
// ) -> Result<i64, OrgManagementAccessError> {
//     let org_id = check_if_org_exists_from_slug(org_slug, conn).await?;
//     check_if_user_is_org_member(user_id, org_id, conn).await?;
//
//     Ok(org_id)
// }
//
// pub(crate) fn a2r(action_response: &ft_common::ActionResponse) -> fastn_core::http::Response {
//     match action_response {
//         ft_common::ActionResponse::Reload => fastn_core::http::frontend_reload(),
//         ft_common::ActionResponse::Redirect(redirect) => {
//             fastn_core::http::frontend_redirect(redirect)
//         }
//         ft_common::ActionResponse::Data(data) => fastn_core::http::frontend_data(data),
//     }
// }
//
// #[cfg(test)]
// mod test {
//     // use super::*;
//
//     /*#[test]
//     fn test_validate_domain() {
//         assert_eq!(validate_domain_("fastn.com"), Ok("fastn.com".to_string()));
//
//         let (_domain, error) = validate_domain_("john.fifthtry.site").unwrap();
//         assert_eq!(error, None);
//
//         let (domain, error) = validate_domain_("http://john.fifthtry.site").unwrap();
//         assert_eq!(error, None);
//         assert_eq!(domain, "john.fifthtry.site".to_string());
//
//         let (domain, error) = validate_domain_("https://john-doe.fifthtry.site");
//         assert_eq!(error, None);
//         assert_eq!(domain, "john-doe.fifthtry.site".to_string());
//
//         let (_domain, error) = validate_domain("-john.fifthtry.site");
//         assert_eq!(error, Some(DOMAIN_REG_INVALID_MSG.to_string()));
//
//         let (_domain, error) = validate_domain("éxàmplê.com");
//         assert_eq!(error, Some(DOMAIN_REG_INVALID_MSG.to_string()));
//
//         let (_domain, error) = validate_domain("john.in-");
//         assert_eq!(error, Some(DOMAIN_REG_INVALID_MSG.to_string()));
//
//         let (_domain, error) = validate_domain("john.in-.com");
//         assert_eq!(error, Some(DOMAIN_REG_INVALID_MSG.to_string()));
//
//         let (_domain, error) = validate_domain("john-.in.com");
//         assert_eq!(error, Some(DOMAIN_REG_INVALID_MSG.to_string()));
//     }*/
// }
