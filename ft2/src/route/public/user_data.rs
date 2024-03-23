#[derive(serde::Serialize)]
pub struct UserData {}

impl ft_sdk::Page<ft2::route::Public, ft2::ActionError> for UserData {
    fn page(_i: &mut ft2::route::Public) -> Result<Self, ft2::ActionError> {
        Ok(UserData {})
    }
}
