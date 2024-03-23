mod editor;
pub use editor::Editor;

mod info;
pub use info::Info;

mod get_content;
pub use get_content::GetContent;

pub mod domain;

mod delete_file;
pub use delete_file::DeleteFile;

mod create_file;
pub use create_file::CreateFile;

mod save_file;
pub use save_file::SaveFile;

pub mod setting;
pub mod github;
pub mod token;


/// Site category is for views that only site members can access.
pub struct Site {
    pub conn: ft_sdk::PgConnection,
    pub in_: ft_sdk::In,
    pub ud: ft2::UserData,
    pub site_data: ft2::site::SiteQueryData,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SiteOutput {
    site: ft2::site::SiteCommonData,
    dashboard_url: String,
    ud: ft2::UserData,
    pub page: serde_json::Value,
}

impl ft_sdk::Layout for Site {
    type Error = ft_common::ActionError;

    fn from_in(in_: ft_sdk::In, ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        use ft_common::{prelude::*, schema::ft_site};
        use ft_sdk::QueryExt;

        ft_sdk::println!("from_in 1");
        let mut conn = ft_sdk::default_pg()?;

        let ud = ft2::route::ud(&mut conn, &in_)?;
        ft_sdk::println!("from_in 2");
        let site_slug = match ty {
            ft_sdk::RequestType::Page => in_.req.query().get("site-slug").map(|v| v.to_string()),
            ft_sdk::RequestType::Action => {
                use ft_sdk::JsonBodyExt;
                in_.req.json_body()?.field::<String>("site-slug")?
            }
        };
        let site_slug = match site_slug {
            Some(v) => v.to_string(),
            None => {
                ft_sdk::println!("site-slug is missing");
                return Err(ft_common::ActionError::UsageError {
                    message: "site-slug is missing".to_string(),
                });
            }
        };
        ft_sdk::println!("from_in 3");

        ft_sdk::println!("from_in 4 site_slug: {}", site_slug);

        let site_data = match ft_site::table
            .filter(ft_site::slug.eq(site_slug))
            .select(ft2::site::SiteQueryData::as_select())
            .first::<ft2::site::SiteQueryData>(&mut conn)
        {
            Ok(v) => v,
            Err(diesel::NotFound) => {
                // Validation returns msg: unknown-site
                ft_sdk::println!("site not found");
                return Err(ft_common::ActionError::NotFound {
                    message: "There is no such site. Maybe the slug of the site has changed, or site has been deleted.".to_string()
                });
            }
            Err(e) => return Err(e.into()),
        };

        ft_sdk::println!("from_in 5");

        match site_data.org_id {
            Some(org_id) => {
                match ft2::check::if_user_can_view_org_contents(ud.user_id, org_id, &mut conn) {
                    Ok(v) => v,
                    Err(ft2::check::OrgManagementAccessError::AccessError(e)) => {
                        return Err(ft_common::ActionError::OrgManagementAccessError(format!(
                            "{e:?}"
                        )));
                    }
                    Err(ft2::check::OrgManagementAccessError::Diesel(e)) => return Err(e.into()),
                };
            }
            None => {
                if site_data.created_by != ud.user_id {
                    return Err(ft_common::ActionError::Unauthorized(
                        "You do not have access to this site".to_string(),
                    ));
                }
            }
        }
        ft_sdk::println!("from_in 6");

        Ok(Site {
            conn: ft_sdk::default_pg()?,
            in_,
            ud,
            site_data,
        })
    }

    fn json(&mut self, page: serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::to_value(SiteOutput {
            page,
            site: ft2::site::SiteCommonData::new(
                &self.ud.username,
                &self.site_data.slug,
                &self.site_data.name,
                &self.site_data.domain,
                self.site_data.updated_at,
            ),
            dashboard_url: ft_common::urls::user_dashboard_url(&self.ud.username),
            ud: self.ud.clone(),
        })?)
    }

    fn render_error(e: Self::Error) -> http::Response<bytes::Bytes> {
        ft_sdk::println!("rendering error: {e:?}");
        ft2::route::error::action_error_to_html(e)
    }
}
