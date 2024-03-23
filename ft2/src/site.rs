#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SiteCommonData {
    pub account_name: String,
    pub site_name: String,
    pub site_slug: String,
    pub primary_domain: String,

    pub info_url: String,
    pub editor_url: String,
    pub github_url: String,
    pub tokens_url: String,
    pub domains_url: String,
    pub setting_url: String,
    pub cr_list_url: String,
    pub environments_url: String,

    pub preview_image: ImageSrc,

    pub updated_on: String,
}

#[derive(serde::Serialize, Debug)]
pub struct ImageSrc {
    pub light: String,
    pub dark: String,
}

impl Default for ImageSrc {
    fn default() -> Self {
        ImageSrc {
            light: "/-/ui.fifthtry.com/assets/sites/placeholder-preview.svg".to_string(),
            dark: "/-/ui.fifthtry.com/assets/sites/placeholder-preview.svg".to_string(),
        }
    }
}

impl SiteCommonData {
    pub fn new(
        account_name: &str,
        site_slug: &str,
        site_name: &str,
        primary_domain: &str,
        updated_on: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        SiteCommonData {
            account_name: account_name.to_string(),
            site_name: site_name.to_string(),
            site_slug: site_slug.to_string(),
            primary_domain: primary_domain.to_string(),
            preview_image: ImageSrc::default(),

            info_url: ft2::urls::site_info_url(site_slug),
            editor_url: ft2::urls::site_editor_url(site_slug),
            github_url: ft2::urls::github_url(site_slug),
            tokens_url: ft2::urls::site_tokens_url(site_slug),
            domains_url: ft2::urls::site_domains_url(site_slug),
            setting_url: ft2::urls::site_domains_url(site_slug),
            cr_list_url: ft2::urls::site_cr_list_url(site_slug),
            environments_url: Default::default(),
            updated_on: ft_common::human_date(updated_on),
        }
    }
}

#[derive(Debug, diesel::Selectable, diesel::Queryable, Clone)]
#[diesel(table_name = ft_common::schema::ft_site)]
pub struct SiteQueryData {
    pub id: i64,
    pub org_id: Option<i64>,
    pub created_by: i64,
    pub name: String,
    pub slug: String,
    pub domain: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_editable: bool,
}

#[cfg(not(feature = "legacy"))]
impl SiteQueryData {
    pub fn update_updated_at(
        &self,
        conn: &mut ft_sdk::PgConnection,
        now: ft_common::DateTime,
    ) -> Result<(), ft2::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_site;

        diesel::update(ft_site::table)
            .filter(ft_site::id.eq(self.id))
            .set(ft_site::updated_at.eq(now))
            .execute(conn)
            .map_err(|e| e.into())
            .map(|_| ())
    }
}

#[cfg(feature = "legacy")]
impl SiteQueryData {
    pub async fn update_updated_at(
        &self,
        conn: &mut ft_common::Conn,
        now: ft_common::DateTime,
    ) -> Result<(), ft2::ActionError> {
        use diesel_async::RunQueryDsl;
        use ft_common::prelude::*;
        use ft_common::schema::ft_site;

        diesel::update(ft_site::table)
            .filter(ft_site::id.eq(self.id))
            .set(ft_site::updated_at.eq(now))
            .execute(conn)
            .await
            .map_err(|e| e.into())
            .map(|_| ())
    }
}
