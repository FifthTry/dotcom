// Documentation in frontend/dev/actions/delete-file.ftd
pub struct DeleteFile {
    file_name: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for DeleteFile {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError>
    where
        Self: Sized,
    {
        pub use ft_sdk::JsonBodyExt;

        let json_body = c.in_.req.json_body()?;
        let file_name = ft2::route::utils::json_field(&json_body, "file-name", "delete")?;
        let updated_at = ft2::route::utils::json_field(&json_body, "updated-at", "delete")?;

        Ok(DeleteFile {
            file_name,
            updated_at,
        })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError>
    where
        Self: Sized,
    {
        if !c.site_data.is_editable {
            return Err(ft_common::ActionError::single_error(
                "delete",
                "This site cannot be updated using editor. Help: Use clift to update.",
            ));
        }
        // if self.updated_at.lt(&c.site_data.updated_at) {
        //     return Err(ft_common::ActionError::single_error(
        //         "delete",
        //         "Looks like content has been updated.",
        //     ));
        // }

        match delete_file(c.site_data.id, self.file_name.as_str(), self.updated_at) {
            Ok(()) => Ok(ft_sdk::ActionOutput::Reload),
            Err(e) => Err(e.get_action_error()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteFileError {
    #[error("{0}")]
    DeleteWasmError(String),
}

impl DeleteFileError {
    fn get_action_error(&self) -> ft_common::ActionError {
        let error = match self {
            DeleteFileError::DeleteWasmError(error) => error.to_owned(),
        };

        ft_common::ActionError::single_error(
            "delete",
            error.as_str(),
        )
    }
}

fn delete_file(site_id: i64, path: &str, updated_at: chrono::DateTime<chrono::Utc>) -> Result<(), DeleteFileError> {
    #[derive(serde::Serialize)]
    struct InputData {
        site_id: i64,
        path: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    }

    let (ptr, len) = ft_sys::memory::json_ptr(InputData {
        site_id,
        path: path.to_string(),
        updated_at,
    });
    let ptr = unsafe { hostn_delete_file(ptr, len) };
    let value = ft_sys::memory::json_from_ptr::<Result<(), String>>(ptr);
    value.map_err(|e| DeleteFileError::DeleteWasmError(e))
}

extern "C" {
    fn hostn_delete_file(ptr: i32, len: i32) -> i32;
}
