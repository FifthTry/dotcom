pub struct CreateSite {
    pub site_slug: String,
}

impl ft_sdk::Action<ft2::route::MySelf, ft2::ActionError> for CreateSite {
    fn validate(c: &mut ft2::route::MySelf) -> Result<Self, ft2::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        let site_slug: String =
            ft2::route::utils::json_field(&c.in_.req.json_body()?, "site-slug", "site")?;
        let site_slug = validate_site_slug(site_slug.as_str())?;

        Ok(CreateSite { site_slug })

        // // Query the database to count number of existing sites under the given org
        // /*let sites_under_org = ft_site::table
        //     .filter(ft_site::org_id.eq(org_id))
        //     .select(count_star())
        //     .first::<i64>(conn)
        //     .await?;
        //
        // if sites_under_org >= 100 {
        //     // Validation returns msg: maximum-site-count-not-exceeded
        //     return Err(
        //         in_.single_error(ft_common::errors::CreateSiteError::MaximumAllowedSiteCountExceeded)
        //     );
        // }
        //
        // // Query the database to check number of sites created in last
        // // one minute under his org
        // let created_sites_under_org_in_last_minute = ft_site::table
        //     .filter(ft_site::org_id.eq(org_id))
        //     .filter(ft_site::created_at.gt(in_.now - chrono::Duration::minutes(1)))
        //     .select(count_star())
        //     .first::<i64>(conn)
        //     .await?;
        //
        // if created_sites_under_org_in_last_minute >= 5 {
        //     // Validation returns msg: create-site-org-rate-limit-reached
        //     return Err(
        //         in_.single_error(ft_common::errors::CreateSiteError::CreateSiteOrgRateLimitReached)
        //     );
        // }*/
    }

    fn action(
        &self,
        i: &mut ft2::route::MySelf,
    ) -> Result<ft_sdk::ActionOutput, ft2::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::{ft_document, ft_document_history, ft_site};
        // Inserting Site
        let site_default_domain = ft2::urls::site_slug_url(self.site_slug.as_str());

        // transaction
        i.conn.transaction(|conn| {
            let site_id: i64 = match diesel::insert_into(ft_site::table)
                .values(ft2::Site {
                    name: self.site_slug.to_string(),
                    slug: self.site_slug.to_string(),
                    is_static: true,
                    is_public: true,
                    is_editable: true,
                    domain: site_default_domain.clone(),
                    created_at: i.in_.now,
                    updated_at: i.in_.now,
                    org_id: None,
                    created_by: i.ud.user_id,
                    is_package: false
                })
                .returning(ft_site::id)
                .get_result::<i64>(conn)
            {
                Ok(id) => id,
                Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => {
                    // Validation returns msg: unique-site-slug
                    return Err(ft2::ActionError::single_error(
                        "site-slug",
                        "site-slug already exists",
                    ));
                }
                Err(e) => return Err(e.into()),
            };

            // we never insert <slug>.fifthtry.site in sites_domain, because the only reason to
            // consider adding it there is to do a redirect, but we can handle that in our redirect
            // code, so everything becomes slightly simpler, as otherwise we have to do a lot of
            // special casing to ignore <slug>.fifthtry.site all other code.

            diesel::insert_into(ft_document::table)
                .values(get_sample_documents(i.in_.now, site_id))
                .execute(conn)?;

            diesel::insert_into(ft_document_history::table)
                .values(get_sample_document_histories(
                    i.in_.now,
                    site_id,
                    i.ud.user_id,
                ))
                .execute(conn)?;

            Ok(())
        })?;

        let site_url = ft2::urls::site_info_url(self.site_slug.as_str());

        Ok(ft_sdk::ActionOutput::Redirect(site_url))
    }
}

fn get_sample_documents(
    now: chrono::DateTime<chrono::Utc>,
    site_id: i64,
) -> Vec<ft_common::Document> {
    ft_common::get_sample_file_names()
        .iter()
        .map(|path| ft_common::Document {
            path: path.to_string(),
            is_public: true,
            is_ftd: path.ends_with(".ftd"),
            created_at: now,
            updated_at: now,
            site_id,
            tejar_content_id: None,
        })
        .collect()
}

fn get_sample_document_histories(
    now: chrono::DateTime<chrono::Utc>,
    site_id: i64,
    editor_id: i64,
) -> Vec<ft_common::DocumentHistory> {
    ft_common::get_sample_file_names()
        .iter()
        .map(|path| ft_common::DocumentHistory {
            path: path.to_string(),
            diff: None,
            created_at: now,
            site_id,
            tejar_content_id: None,
            is_deleted: false,
            editor_id,
        })
        .collect()
}

fn validate_site_slug(site_slug: &str) -> Result<String, ft2::ActionError> {
    let site_slug = site_slug.trim().to_string();

    // Validation returns msg: site-slug-is-lowercase
    if site_slug != site_slug.to_lowercase() {
        // we explicitly warn user instead of converting it to lower case, because
        // user might be relying on case, and change in case breaks the meaning they
        // are going for, e.g. MissIng and missing are different things, maybe they would
        // prefer miss-ing.fifthtry.site and not missing.fifthtry.site
        return Err(ft2::ActionError::single_error(
            "site",
            "site-slug should be in lowercase",
        ));
    }

    // Validation returns msg: site-slug-is-not-empty
    if site_slug.is_empty() {
        return Err(ft2::ActionError::single_error(
            "site",
            "site-slug should not be empty",
        ));
    }

    static RE: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

    // Validation returns msg: site-slug-regex
    if !RE.is_match(site_slug.as_str()) {
        return Err(ft2::ActionError::single_error(
            "site",
            "site-slug is not valid",
        ));
    }

    // NOTE: We do not check if slug is unique or not here, because we are doing it in
    // the transaction lower down, which is a more reliable check, no need to do the query
    // twice. This is same reason as mentioned in the note in the verify method.

    Ok(site_slug)
}
