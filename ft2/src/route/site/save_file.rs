// Documentation in frontend/dev/actions/create-file.ftd
pub struct SaveFile {
    file_name: String,
    content: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}


impl ft_sdk::Action<ft2::route::Site, ft2::ActionError> for SaveFile {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft2::ActionError>
    where
        Self: Sized,
    {
        pub use ft_sdk::JsonBodyExt;

        let json_body = c.in_.req.json_body()?;
        let file_name = ft2::route::utils::json_field(&json_body, "file-name", "save")?;
        let content = ft2::route::utils::json_field(&json_body, "content", "save")?;
        let updated_at = ft2::route::utils::json_field(&json_body, "updated-at", "save")?;

        Ok(SaveFile {
            file_name,
            content,
            updated_at
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
                "save",
                "This site cannot be updated using editor. Help: Use clift to update.",
            ));
        }

        match save_file(c.site_data.id, self.file_name.as_str(), self.content.as_str(), self.updated_at) {
            Ok(()) => Ok(ft_sdk::ActionOutput::Reload),
            Err(e) => Err(e.get_action_error()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SaveFileError {
    #[error("{0}")]
    SaveWasmError(String),
}

impl SaveFileError {
    fn get_action_error(&self) -> ft2::ActionError {
        let error = match self {
            SaveFileError::SaveWasmError(error) => error.to_owned(),
        };

        ft2::ActionError::single_error("save", error.as_str())
    }
}

fn save_file(
    site_id: i64,
    path: &str,
    content: &str,
    updated_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), SaveFileError> {
    #[derive(serde::Serialize)]
    struct InputData {
        site_id: i64,
        path: String,
        content: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    }

    let (ptr, len) = ft_sys::memory::json_ptr(InputData {
        site_id,
        path: path.to_string(),
        content: content.to_string(),
        updated_at
    });
    let ptr = unsafe { hostn_save_file(ptr, len) };
    let value = ft_sys::memory::json_from_ptr::<Result<(), String>>(ptr);
    value.map_err(|e| SaveFileError::SaveWasmError(e))
}

extern "C" {
    fn hostn_save_file(ptr: i32, len: i32) -> i32;
}
