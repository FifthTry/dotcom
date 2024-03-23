// Documentation in frontend/dev/actions/re-check-domain.ftd
pub struct GetContent {
    file_name: String,
}

impl ft_sdk::Action<ft2::route::Site, ft2::ActionError> for GetContent {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft2::ActionError>
    where
        Self: Sized,
    {
        pub use ft_sdk::JsonBodyExt;

        let file_name =
            ft2::route::utils::json_field(&c.in_.req.json_body()?, "file-name", "open")?;

        Ok(GetContent { file_name })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft2::ActionError>
    where
        Self: Sized,
    {
        let file = ft2::File::from_path(
            c.site_data.id,
            c.site_data.domain.as_str(),
            &c.site_data.updated_at,
            self.file_name.as_str(),
        )
        .map_err(|e| e.into_action_error())?;

        Ok(ft_sdk::ActionOutput::Data(std::collections::HashMap::from(
            [("content".to_string(), serde_json::to_value(file).unwrap())],
        )))
    }
}
