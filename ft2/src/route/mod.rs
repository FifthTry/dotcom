pub mod error;
pub mod myself;
mod org;
pub mod public;
pub mod site;
mod urls;
mod utils;

pub use myself::MySelf;
pub use org::Org;
pub use public::Public;
pub use site::Site;
pub use urls::route;

pub fn ud(
    conn: &mut ft_sdk::PgConnection,
    in_: &ft_sdk::In,
) -> Result<ft2::UserData, ft_common::ActionError> {
    match ft2::ud::get_optional_ud(conn, in_) {
        Ok(Some(ud)) => Ok(ud),
        Ok(None) => {
            tracing::error!("user not logged in");
            Err(ft_common::ActionError::Unauthorized(
                "user not logged in".to_string(),
            ))
        }
        Err(e) => {
            tracing::error!("failed to get user data: {:?}", e);
            Err(ft_common::ActionError::ServerError {
                message: "failed to get user data".to_string(),
            })
        }
    }
}
