#[derive(Debug, serde::Serialize, PartialEq)]
pub struct FileText {
    lang: String,
    content: String,
    // TODO: include rendered content?
}

impl FileText {
    fn from_path(
        site_id: i64,
        path: &str,
        file_type: &FileType,
    ) -> Result<Option<FileText>, FileTextError> {
        match file_type {
            FileType::Ftd => Ok(Some(FileText {
                lang: "ftd".to_string(),
                content: FileText::get_content(site_id, path)?,
            })),
            FileType::Fastn => Ok(Some(FileText {
                lang: "ftd".to_string(),
                content: FileText::get_content(site_id, path)?,
            })),
            FileType::Text => Ok(Some(FileText {
                lang: "txt".to_string(),
                content: FileText::get_content(site_id, path)?,
            })),
            FileType::Markdown => Ok(Some(FileText {
                lang: "md".to_string(),
                content: FileText::get_content(site_id, path)?,
            })),
            FileType::Source(Some(ext)) => Ok(Some(FileText {
                lang: ext.to_string(),
                content: FileText::get_content(site_id, path)?,
            })),
            FileType::Dir | FileType::Image | FileType::Video | FileType::Source(None) => Ok(None),
        }
    }

    fn get_content(site_id: i64, path: &str) -> Result<String, FileTextError> {
        #[derive(serde::Serialize)]
        struct GetContentInputData {
            site_id: i64,
            path: String,
        }

        let (ptr, len) = ft_sys::memory::json_ptr(GetContentInputData {
            site_id,
            path: path.to_string(),
        });
        let ptr = unsafe { hostn_get_content(ptr, len) };
        let value = ft_sys::memory::json_from_ptr::<Option<Vec<u8>>>(ptr)
            .map(|v| String::from_utf8(v).ok())
            .flatten();
        value.ok_or(FileTextError::CantReadFile)
    }
}

extern "C" {
    fn hostn_get_content(ptr: i32, len: i32) -> i32;
}

#[derive(thiserror::Error, Debug)]
pub enum FileTextError {
    // #[error("Read document content error {0}")]
    // ReadDocumentContentError(#[from] ft_common::ReadDocumentContentError),
    #[error("Cant read file")]
    CantReadFile,
}

#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("diesel error {0}")]
    DieselError(#[from] diesel::result::Error),
}

impl FileError {
    pub(crate) fn into_action_error(self) -> ft_common::ActionError {
        match self {
            FileError::DieselError(d) => ft_common::ActionError::Diesel(d),
        }
    }
}

#[derive(Debug, serde::Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct File {
    base_name: String,
    full_name: String,
    url: String,
    filetype: FileType,
    text: Option<FileText>,
    error: Option<String>,
    updated_at: String,
}

impl File {
    pub(crate) fn from_path(
        site_id: i64,
        domain: &str,
        updated_at: &chrono::DateTime<chrono::Utc>,
        path: &str,
    ) -> Result<File, FileError> {
        let base_name = path.split('/').last().unwrap();
        let full_name = path.to_string();
        let url = get_url(domain, path);
        let file_type = file_type(base_name);
        let (text, error) =
            get_text_and_error_string(FileText::from_path(site_id, path, &file_type));

        Ok(File {
            base_name: base_name.to_string(),
            full_name,
            url,
            text,
            filetype: file_type,
            error,
            updated_at: updated_at.to_string(),
        })
    }
}

// folder, FASTN, FTD, Image, Video, Text, Markdown, Source
#[derive(Debug, PartialEq)]
pub enum FileType {
    Dir,
    Ftd,
    Fastn,
    Image,
    Video,
    Text,
    Markdown,
    Source(Option<String>),
}

impl serde::Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl FileType {
    pub fn to_str(&self) -> &'static str {
        match self {
            FileType::Dir => "dir",
            FileType::Ftd => "ftd",
            FileType::Fastn => "fastn",
            FileType::Image => "image",
            FileType::Video => "video",
            FileType::Text => "text",
            FileType::Markdown => "markdown",
            FileType::Source(_) => "source",
        }
    }
}

pub fn file_type(base_name: &str) -> FileType {
    let base_name = base_name.split('/').last().unwrap();
    match base_name.rsplit_once('.') {
        Some(("FASTN", "ftd")) => FileType::Fastn,
        Some((_, "ftd")) => FileType::Ftd,
        Some((_, "jpg" | "jpeg" | "png" | "gif" | "svg")) => FileType::Image,
        Some((_, "mp4" | "webm" | "ogg")) => FileType::Video,
        Some((_, "txt")) => FileType::Text,
        Some((_, "md")) => FileType::Markdown,
        Some((_, t)) => FileType::Source(Some(t.to_string())),
        None => FileType::Source(None),
    }
}

fn get_url(domain: &str, path: &str) -> String {
    let path = path.trim_matches('/');
    if let Some(remaining_path) = path.strip_prefix(".packages/") {
        return format!("//{domain}/-/{remaining_path}");
    }
    format!("//{domain}/{path}")
}

fn get_text_and_error_string(
    value: Result<Option<FileText>, FileTextError>,
) -> (Option<FileText>, Option<String>) {
    match value {
        Ok(text) => (text, None),
        Err(FileTextError::CantReadFile) => (None, Some("Can't read file content".to_string())),
    }
}
