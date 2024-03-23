// Documentation in frontend/dev/actions/create-file.ftd
pub struct CreateFile {
    file_name: String,
    content: String,
}

impl ft_sdk::Action<ft2::route::Site, ft2::ActionError> for CreateFile {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft2::ActionError>
    where
        Self: Sized,
    {
        pub use ft_sdk::JsonBodyExt;

        let json_body = c.in_.req.json_body()?;
        let file_name = ft2::route::utils::json_field(&json_body, "file-name", "create")?;
        let content = ft2::route::utils::json_field(&json_body, "content", "create")?;

        Ok(CreateFile {
            file_name,
            content,
        })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft2::ActionError>
    where
        Self: Sized,
    {
        if !c.site_data.is_editable {
            return Err(ft2::ActionError::single_error(
                "create",
                "This site cannot be updated using editor. Help: Use clift to update.",
            ));
        }

        match create_file(c.site_data.id, self.file_name.as_str(), self.content.as_str()) {
            Ok(()) => Ok(ft_sdk::ActionOutput::Reload),
            Err(e) => Err(e.get_action_error()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateFileError {
    #[error("{0}")]
    CreateWasmError(String),
}

impl CreateFileError {
    fn get_action_error(&self) -> ft2::ActionError {
        let error = match self {
            CreateFileError::CreateWasmError(error) => error.to_owned(),
        };

        ft2::ActionError::single_error("create", error.as_str())
    }
}

fn create_file(
    site_id: i64,
    path: &str,
    content: &str,
) -> Result<(), CreateFileError> {
    #[derive(serde::Serialize)]
    struct InputData {
        site_id: i64,
        path: String,
        content: String,
    }

    let (ptr, len) = ft_sys::memory::json_ptr(InputData {
        site_id,
        path: path.to_string(),
        content: content.to_string(),
    });
    let ptr = unsafe { hostn_create_file(ptr, len) };
    let value = ft_sys::memory::json_from_ptr::<Result<(), String>>(ptr);
    value.map_err(|e| CreateFileError::CreateWasmError(e))
}

extern "C" {
    fn hostn_create_file(ptr: i32, len: i32) -> i32;
}
