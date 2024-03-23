#[derive(Debug, diesel::Selectable, diesel::Queryable, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
#[diesel(table_name = ft_common::schema::ft_site_token)]
struct Token {
    id: i64,
    token: String,
    about: String,
    can_read: bool,
    can_write: bool,
    #[serde(serialize_with = "ft_common::serialize_datetime")]
    created_at: ft_common::DateTime,
    #[serde(serialize_with = "ft_common::serialize_o_datetime")]
    last_used_at: Option<ft_common::DateTime>,
}

#[derive(serde::Serialize, Debug)]
#[serde(transparent)]
pub struct Tokens(Vec<Token>);

impl ft_sdk::Page<ft2::route::Site, ft2::ActionError> for Tokens {
    fn page(i: &mut ft2::route::Site) -> Result<Self, ft2::ActionError> {
        use ft_common::{prelude::*, schema::ft_site_token};

        Ok(Tokens(
            ft_site_token::table
                .filter(ft_site_token::site_id.eq(i.site_data.id))
                .select(Token::as_select())
                .load(&mut i.conn)?,
        ))
    }
}
