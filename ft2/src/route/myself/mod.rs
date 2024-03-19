mod create_site;

pub use create_site::CreateSite;

pub struct MySelf {
    pub in_: ft_sdk::In,
    pub ud: ft2::UserData,
    pub conn: ft_sdk::PgConnection,
}

#[derive(serde::Serialize)]
pub struct MyselfOutput {
    ud: ft2::UserData,
    pub page: serde_json::Value,
}

impl ft_sdk::Layout for MySelf {
    type Error = ft_common::ActionError;

    fn from_in(in_: ft_sdk::In, _ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        let mut conn = ft_sdk::default_pg()?;
        let ud = match ft2::route::ud(&mut conn, &in_) {
            Err(e) => {
                return Err(ft_common::ActionError::Unauthorized(format!(
                    "user not logged in: {e:?}"
                )));
            }
            Ok(ud) => ud,
        };
        Ok(MySelf { ud, in_, conn })
    }

    fn json(&mut self, page: serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::to_value(MyselfOutput {
            page,
            ud: self.ud.clone(),
        })?)
    }

    fn render_error(e: Self::Error) -> http::Response<bytes::Bytes> {
        ft2::route::error::action_error_to_html(e)
    }
}
