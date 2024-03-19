impl ft_sdk::Page<ft2::route::Org, ft_common::ActionError> for ft2::route::public::UserDashboard {
    fn page(i: &mut ft2::route::Org) -> Result<Self, ft_common::ActionError> {
        ft_sdk::println!("hello wasm");
        let dashboard_data = Self {
            sites: get_all_sites(&mut i.conn, i.org_id, i.org_slug.as_str())?,
            create_site_url: ft_common::urls::create_org_site_url(i.org_slug.as_str()),
        };

        tracing::info!("dashboard_view_response: {dashboard_data:?}");

        Ok(dashboard_data)
    }
}

// Database read operations
pub fn get_all_sites(
    conn: &mut ft_sdk::PgConnection,
    org_id: i64,
    org_slug: &str,
) -> Result<Vec<ft_common::site::SiteCommonData>, ft_common::ActionError> {
    use ft_common::{prelude::*, schema::ft_site};

    // Fetching all sites belonging to the given org
    // using org-id
    let sites_data = ft_site::table
        .select(ft2::route::public::SiteData::as_select())
        .filter(ft_site::org_id.eq(org_id))
        .load::<ft2::route::public::SiteData>(conn)?;

    let mut sites = vec![];
    for s in sites_data {
        sites.push(ft_common::site::SiteCommonData::new(
            org_slug,
            s.slug.as_str(),
            s.name.as_str(),
            s.domain.as_str(),
            s.updated_at,
        ));
    }

    Ok(sites)
}
