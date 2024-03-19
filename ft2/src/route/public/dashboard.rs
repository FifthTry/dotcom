impl ft_sdk::Page<ft2::route::Public, ft_common::ActionError> for UserDashboard {
    fn page(i: &mut ft2::route::Public) -> Result<Self, ft_common::ActionError> {
        let ud = match i.in_.ud {
            Some(ref ud) => ud,
            None => return Ok(UserDashboard::default()),
        };
        tracing::info!("username: {}", ud.username);

        let user_dashboard_data = UserDashboard {
            sites: get_all_sites(ud.id, ud.username.as_str())?,
            create_site_url: ft_common::urls::create_user_site_url(),
        };

        tracing::info!("dashboard_view_response: {user_dashboard_data:?}");

        Ok(user_dashboard_data)
    }
}

// Database read operations
pub fn get_all_sites(
    user_id: i64,
    username: &str,
) -> Result<Vec<ft_common::site::SiteCommonData>, ft_common::ActionError> {
    use ft_common::{prelude::*, schema::ft_site};

    let mut conn = ft_sdk::default_pg()?;

    // Fetching all sites owned by this user
    let sites_data = ft_site::table
        .select(SiteData::as_select())
        .filter(ft_site::org_id.is_null())
        .filter(ft_site::created_by.eq(user_id))
        .load::<SiteData>(&mut conn)?;

    let mut sites = vec![];
    for s in sites_data {
        sites.push(ft_common::site::SiteCommonData::new(
            username,
            s.slug.as_str(),
            s.name.as_str(),
            s.domain.as_str(),
            s.updated_at,
        ));
    }

    Ok(sites)
}

// This struct is equivalent to the record we are expecting
// inside sites/dashboard.ftd
#[derive(Default, Debug, serde::Serialize)]
pub struct UserDashboard {
    pub sites: Vec<ft_common::site::SiteCommonData>,
    #[serde(rename = "create-site-url")]
    pub create_site_url: String,
}

// Struct for data extracted from db table
#[derive(Debug, diesel::Selectable, diesel::Queryable)]
#[diesel(table_name = ft_common::schema::ft_site)]
pub struct SiteData {
    pub slug: String,
    pub name: String,
    pub domain: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
