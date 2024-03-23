mod dashboard;
mod user_data;

pub use dashboard::{SiteData, UserDashboard};
pub use user_data::UserData;

pub struct Public {
    pub in_: ft_sdk::In,
    conn: ft_sdk::PgConnection,
}

#[derive(serde::Serialize)]
pub struct PublicOutput {
    ud: ft2::UserData,
    pub page: serde_json::Value,
}

impl ft_sdk::Layout for Public {
    type Error = ft2::ActionError;

    fn from_in(in_: ft_sdk::In, _ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        Ok(Public {
            in_,
            conn: ft_sdk::default_pg()?,
        })
    }

    fn json(&mut self, page: serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::to_value(PublicOutput {
            page,
            ud: ft2::ud::get_optional_ud(&mut self.conn, &self.in_)
                .unwrap_or_default()
                .unwrap_or_default(),
        })?)
    }

    fn render_error(e: Self::Error) -> http::Response<bytes::Bytes> {
        ft2::route::error::action_error_to_html(e)
    }
}
