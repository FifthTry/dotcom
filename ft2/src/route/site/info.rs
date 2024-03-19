#[derive(serde::Serialize)]
pub struct Info {}

impl ft_sdk::Page<ft2::route::Site, ft_common::ActionError> for Info {
    fn page(_i: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        ft_sdk::println!("hello wasm");
        Ok(Info {})
    }
}
