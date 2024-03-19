#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Editor {
    files: Vec<FileInfo>,
    current_file: ft2::File,
    site_base_url: String,
    site_relative_url: String,
}

impl ft_sdk::Page<ft2::route::Site, ft_common::ActionError> for Editor {
    fn page(i: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        use ft_common::{prelude::*, schema::ft_document};
        use ft_sdk::QueryExt;

        let file = i
            .in_
            .req
            .query()
            .get("file")
            .unwrap_or("index.ftd")
            .to_string();
        tracing::info!("Fetching site editor for site: {}", i.site_data.slug);

        let documents = ft_document::table
            .select(ft_document::path)
            .filter(ft_document::site_id.eq(i.site_data.id))
            .load::<String>(&mut i.conn)?;

        let files = documents
            .iter()
            .map(|path| {
                let base_name = path.split('/').last().unwrap().to_string();
                FileInfo {
                    full_name: path.to_string(),
                    is_active: path == &file,
                    filetype: ft2::file_type(base_name.as_str()),
                    base_name,
                    children: vec![],
                }
            })
            .collect();
        let files = make_tree(files);

        let current_file = ft2::File::from_path(
            i.site_data.id,
            i.site_data.domain.as_str(),
            &i.site_data.updated_at,
            file.as_str(),
        )
        .map_err(|e| e.into_action_error())?;

        // if there is no current file we open index.ftd
        Ok(Editor {
            files,
            current_file,
            site_base_url: format!("//{}", i.site_data.domain),
            site_relative_url: "/".to_string(),
        })
    }
}

#[derive(Debug, serde::Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct FileInfo {
    base_name: String,
    full_name: String,
    is_active: bool,
    filetype: ft2::FileType,
    children: Vec<FileInfo>,
}

fn make_tree(files: Vec<FileInfo>) -> Vec<FileInfo> {
    let mut tree: Vec<FileInfo> = vec![];

    for file in files {
        let mut path = file.full_name.split('/').collect::<Vec<&str>>();
        path.pop();

        let mut current_level = &mut tree;

        for dir in path {
            let mut found = false;

            for (index, child) in current_level.iter_mut().enumerate() {
                if child.base_name == dir {
                    current_level = &mut current_level[index].children;
                    found = true;
                    break;
                }
            }

            if !found {
                let new_dir = FileInfo {
                    base_name: dir.to_string(),
                    full_name: dir.to_string(),
                    is_active: false,
                    filetype: ft2::FileType::Dir,
                    children: vec![],
                };
                current_level.push(new_dir);
                let last_index = current_level.len() - 1;
                current_level = &mut current_level[last_index].children;
            }
        }

        current_level.push(file);
    }

    tree
}

#[cfg(test)]
mod test {
    #[test]
    fn test_make_tree() {
        use super::*;
        let files = vec![
            FileInfo {
                base_name: "index.ftd".to_string(),
                full_name: "index.ftd".to_string(),
                is_active: true,
                filetype: ft2::FileType::Ftd,
                children: vec![],
            },
            FileInfo {
                base_name: "FASTN.ftd".to_string(),
                full_name: "FASTN.ftd".to_string(),
                is_active: false,
                filetype: ft2::FileType::Fastn,
                children: vec![],
            },
            FileInfo {
                base_name: "main.rs".to_string(),
                full_name: "src/main.rs".to_string(),
                is_active: false,
                filetype: ft2::FileType::Source(Some("rs".to_string())),
                children: vec![],
            },
            FileInfo {
                base_name: "lib.rs".to_string(),
                full_name: "src/lib.rs".to_string(),
                is_active: false,
                filetype: ft2::FileType::Source(Some("rs".to_string())),
                children: vec![],
            },
            FileInfo {
                base_name: "ftd.rs".to_string(),
                full_name: "src/ftd.rs".to_string(),
                is_active: false,
                filetype: ft2::FileType::Source(Some("rs".to_string())),
                children: vec![],
            },
        ];
        let tree = make_tree(files);
        let files = vec![
            FileInfo {
                base_name: "index.ftd".to_string(),
                full_name: "index.ftd".to_string(),
                is_active: true,
                filetype: ft2::FileType::Ftd,
                children: vec![],
            },
            FileInfo {
                base_name: "FASTN.ftd".to_string(),
                full_name: "FASTN.ftd".to_string(),
                is_active: false,
                filetype: ft2::FileType::Fastn,
                children: vec![],
            },
            FileInfo {
                base_name: "src".to_string(),
                full_name: "src".to_string(),
                is_active: false,
                filetype: ft2::FileType::Dir,
                children: vec![
                    FileInfo {
                        base_name: "main.rs".to_string(),
                        full_name: "src/main.rs".to_string(),
                        is_active: false,
                        filetype: ft2::FileType::Source(Some("rs".to_string())),
                        children: vec![],
                    },
                    FileInfo {
                        base_name: "lib.rs".to_string(),
                        full_name: "src/lib.rs".to_string(),
                        is_active: false,
                        filetype: ft2::FileType::Source(Some("rs".to_string())),
                        children: vec![],
                    },
                    FileInfo {
                        base_name: "ftd.rs".to_string(),
                        full_name: "src/ftd.rs".to_string(),
                        is_active: false,
                        filetype: ft2::FileType::Source(Some("rs".to_string())),
                        children: vec![],
                    },
                ],
            },
        ];
        assert_eq!(tree, files);

        // Test with a deeper tree structure
        let files_deeper = vec![
            FileInfo {
                base_name: "a".to_string(),
                full_name: "a".to_string(),
                is_active: true,
                filetype: ft2::FileType::Dir,
                children: vec![],
            },
            FileInfo {
                base_name: "b".to_string(),
                full_name: "a/b".to_string(),
                is_active: true,
                filetype: ft2::FileType::Dir,
                children: vec![],
            },
            FileInfo {
                base_name: "c".to_string(),
                full_name: "a/b/c".to_string(),
                is_active: true,
                filetype: ft2::FileType::Dir,
                children: vec![],
            },
            FileInfo {
                base_name: "file.txt".to_string(),
                full_name: "a/b/c/file.txt".to_string(),
                is_active: false,
                filetype: ft2::FileType::Text,
                children: vec![],
            },
        ];

        let expected_tree_deeper = vec![FileInfo {
            base_name: "a".to_string(),
            full_name: "a".to_string(),
            is_active: true,
            filetype: ft2::FileType::Dir,
            children: vec![FileInfo {
                base_name: "b".to_string(),
                full_name: "a/b".to_string(),
                is_active: true,
                filetype: ft2::FileType::Dir,
                children: vec![FileInfo {
                    base_name: "c".to_string(),
                    full_name: "a/b/c".to_string(),
                    is_active: true,
                    filetype: ft2::FileType::Dir,
                    children: vec![FileInfo {
                        base_name: "file.txt".to_string(),
                        full_name: "a/b/c/file.txt".to_string(),
                        is_active: false,
                        filetype: ft2::FileType::Text,
                        children: vec![],
                    }],
                }],
            }],
        }];

        assert_eq!(make_tree(files_deeper), expected_tree_deeper);
    }
}
