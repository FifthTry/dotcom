// Documentation in frontend/dev/actions/delete-file.ftd
pub struct DeleteFile {
    file_name: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for DeleteFile {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> where Self: Sized {
        pub use ft_sdk::JsonBodyExt;

        let json_body = c.in_.req.json_body()?;
        let file_name = ft2::route::utils::json_field(&json_body, "file-name", "delete")?;
        let updated_at = ft2::route::utils::json_field(&json_body, "updated-at", "delete")?;

        Ok(DeleteFile { file_name, updated_at })
    }

    fn action(&self, c: &mut ft2::route::Site) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> where Self: Sized {
        if !c.site_data.is_editable {
            return Err(ft_common::ActionError::single_error(
                "delete",
               "This site cannot be updated using editor. Help: Use clift to update.",
            ));
        }
        if self.updated_at.lt(&c.site_data.updated_at) {
            return Err(ft_common::ActionError::single_error(
                "delete",
                "Looks like content has been updated.",
            ));
        }

        match delete_file(c.site_data.id, self.file_name.as_str()) {
            Ok(()) => Ok(ft_sdk::ActionOutput::Reload),
            Err(e) => Err(e.get_action_error())
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteFileError {
    #[error("Cant delete file")]
    CantDeleteFile,
}

impl DeleteFileError {
    fn get_action_error(&self) -> ft_common::ActionError {
        match self {
            DeleteFileError::CantDeleteFile => ft_common::ActionError::single_error(
                "delete",
                "Something went wrong. Can't Delete File. Try again later.",
            )
        }
    }
}


fn delete_file(site_id: i64, path: &str) -> Result<(), DeleteFileError> {
    #[derive(serde::Serialize)]
    struct GetContentInputData {
        site_id: i64,
        path: String,
    }

    let (ptr, len) = ft_sys::memory::json_ptr(GetContentInputData {
        site_id,
        path: path.to_string(),
    });
    let ptr = unsafe { hostn_delete_file(ptr, len) };
    let value = ft_sys::memory::json_from_ptr::<Option<()>>(ptr);
    value.ok_or(DeleteFileError::CantDeleteFile)
}

extern "C" {
    fn hostn_delete_file(ptr: i32, len: i32) -> i32;
}