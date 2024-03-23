pub mod dashboard;

pub struct Org {
    pub in_: ft_sdk::In,
    pub ud: ft2::UserData,
    pub org_slug: String,
    pub org_id: i64,
    pub conn: ft_sdk::PgConnection,
}

#[derive(serde::Serialize)]
pub struct OrgOutput {
    ud: ft2::UserData,
    pub page: serde_json::Value,
}

impl ft_sdk::Layout for Org {
    type Error = ft2::ActionError;

    fn from_in(in_: ft_sdk::In, _ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        use ft_sdk::QueryExt;

        let mut conn = ft_sdk::default_pg()?;

        let ud = match ft2::route::ud(&mut conn, &in_) {
            Err(e) => {
                return Err(ft2::ActionError::Unauthorized(format!(
                    "user not logged in: {e:?}"
                )));
            }
            Ok(ud) => ud,
        };

        let org_slug = match in_.req.query().get("org-slug") {
            Some(v) => v.to_string(),
            None => {
                ft_sdk::println!("org-slug is missing");
                return Err(ft2::ActionError::UsageError {
                    message: "org-slug is missing".to_string(),
                });
            }
        };

        let org_id = match ft2::check::if_user_can_manage_org_using_slug(
            ud.user_id,
            org_slug.as_str(),
            &mut conn,
        ) {
            Ok(v) => v,
            Err(ft2::check::OrgManagementAccessError::AccessError(e)) => {
                return Err(ft2::ActionError::OrgManagementAccessError(format!(
                    "{e:?}"
                )));
            }
            Err(ft2::check::OrgManagementAccessError::Diesel(e)) => return Err(e.into()),
        };

        Ok(Org {
            ud,
            in_,
            org_id,
            org_slug,
            conn,
        })
    }

    fn json(&mut self, page: serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::to_value(OrgOutput {
            page,
            ud: self.ud.clone(),
        })?)
    }

    fn render_error(e: Self::Error) -> http::Response<bytes::Bytes> {
        ft2::route::error::action_error_to_html(e)
    }
}
