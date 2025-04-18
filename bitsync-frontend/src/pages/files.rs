use bitsync_core::use_case::user_files::{
    create_directory::DirectoryCreationResult, delete_user_file::UserFileDeletionResult,
    move_user_file::UserFileMoveResult, read_user_directory_contents::UserDirectoryContentsResult,
    upload_user_file::UserFileResult,
};
use bitsync_routes::TypedPath;
use maud::Render;

use crate::models::{ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind};

pub enum FilesHomePageElementId {
    FileUploadForm,
    DirectoryCreationDialog,
    FileStorageTableWrapper,
}

impl FilesHomePageElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomePageElementId::FileUploadForm => "file_upload_form",
            FilesHomePageElementId::FileStorageTableWrapper => "file_storage_table_wrapper",
            FilesHomePageElementId::DirectoryCreationDialog => "directory_creation_dialog",
        }
    }
}

pub struct FilesHomePage {
    dir_content: Vec<StorageItemPresentation>,
    parent_directory_url: Option<ParentDirectoryLink>,
    file_upload_url: String,
    directory_creation_url: String,
    directory_creation_popover_id: String,
}

impl From<UserDirectoryContentsResult> for FilesHomePage {
    fn from(value: UserDirectoryContentsResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        let file_upload_url = bitsync_routes::PostUserFileUpload
            .with_query_params(bitsync_routes::PostUserFileUploadQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let directory_creation_url = bitsync_routes::PostUserFileDirectoryCreation
            .with_query_params(
                bitsync_routes::PostUserFileDirectoryCreationQueryParameters {
                    path: value.path.path(),
                },
            )
            .to_string();

        FilesHomePage {
            dir_content: displayable_dir_content,
            parent_directory_url: ParentDirectoryLink::from_child(value.path.scoped_path),
            file_upload_url,
            directory_creation_url,
            directory_creation_popover_id: FilesHomePageElementId::DirectoryCreationDialog
                .to_str()
                .to_owned(),
        }
    }
}

impl Render for FilesHomePage {
    fn render(&self) -> maud::Markup {
        super::base::LoggedInDocument(maud::html! {
            style { (crate::styles::files_home_page::STYLE_SHEET) }
            main {
                @match &self.parent_directory_url {
                    Some(link_data) => {
                        a href=(link_data.parent_directory_url) {
                            i."small" {
                                "chevron_left"
                            }
                            (link_data.current_directory_name)
                        }
                    }
                    None => {}
                }

                (FileUploadForm { file_upload_url: self.file_upload_url.clone() })

                button onclick=(format_args!("openPopoverById('{}')", &self.directory_creation_popover_id)) {
                    "add directory"
                }
                dialog class=(crate::styles::files_home_page::ClassName::ACTIONS_POPOVER) popover id=(self.directory_creation_popover_id) {
                    form hx-post=(self.directory_creation_url) hx-target="this" {
                        input type="text" name="directory_name";
                        button {
                            "Create"
                        }
                        button type="button" onclick="closeClosestPopover(this)" {
                            "Cancel"
                        }
                    }
                }
                div #(FilesHomePageElementId::FileStorageTableWrapper.to_str()) {
                    ((FileStorageTable { dir_content: self.dir_content.clone() }))
                }
            }
        }).render()
    }
}

struct FileUploadForm {
    file_upload_url: String,
}

impl Render for FileUploadForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form #(FilesHomePageElementId::FileUploadForm.to_str()) hx-post=(self.file_upload_url) enctype="multipart/form-data" {
                input type="file" name="upload";
                input type="submit" value="Upload";
            }
        }
    }
}

struct FileStorageTable {
    dir_content: Vec<StorageItemPresentation>,
}

impl Render for FileStorageTable {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @if self.dir_content.is_empty() { } @else {
                table {
                    thead {
                        tr {
                            th."" {}
                            th {
                                "Name"
                            }
                            th."" {
                                "Size"
                            }
                            th."" {}
                        }
                    }
                    tbody {
                        @for dir_item in &self.dir_content {
                            tr hx-target="this" {
                                td {
                                    i."" {
                                        (dir_item.kind.icon())
                                    }
                                }
                                td {
                                    @match &dir_item.kind {
                                        StorageItemPresentationKind::Directory { url } => {
                                            a href=(url) {
                                                (dir_item.name)
                                            }
                                        }
                                        StorageItemPresentationKind::File => {
                                            (dir_item.name)
                                        }
                                    }
                                }
                                td class=(crate::styles::files_home_page::ClassName::FILE_SIZE_COLUMN) {
                                    (dir_item.size)
                                }
                                td {
                                    button onclick=(format_args!("openPopoverById('{}')", dir_item.actions_popover_id)) {
                                        "..."
                                    }
                                    dialog class=(crate::styles::files_home_page::ClassName::ACTIONS_POPOVER) popover id=(dir_item.actions_popover_id) {
                                        h1 {
                                            (dir_item.name)
                                        }
                                        hr;
                                        button onclick=(format_args!("openPopoverById('{}')", dir_item.actions_move_popover_id)) {
                                            "Move"
                                        }
                                        dialog class=(crate::styles::files_home_page::ClassName::ACTIONS_POPOVER) popover id=(dir_item.actions_move_popover_id) {
                                            form hx-post=(dir_item.move_url) hx-target="this" {
                                                (dir_item.path)
                                                input type="text" value=(dir_item.path) name="destination_path";
                                                button {
                                                    "Move"
                                                }
                                                button type="button" onclick="closeClosestPopover(this)" {
                                                    "Cancel"
                                                }
                                            }
                                        }
                                        a href=(dir_item.download_url) onclick="closeClosestDialog(this)" {
                                            "Download"
                                        }
                                        button hx-get=(dir_item.delete_url) onclick="closeClosestDialog(this)" {
                                            "Delete"
                                        }
                                        button onclick="closeClosestPopover(this)" {
                                            "close"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct FilesHomePageChangeResult {
    dir_content: Vec<StorageItemPresentation>,
}

impl From<UserFileResult> for FilesHomePageChangeResult {
    fn from(value: UserFileResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl From<UserFileDeletionResult> for FilesHomePageChangeResult {
    fn from(value: UserFileDeletionResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl From<UserFileMoveResult> for FilesHomePageChangeResult {
    fn from(value: UserFileMoveResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl From<DirectoryCreationResult> for FilesHomePageChangeResult {
    fn from(value: DirectoryCreationResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl Render for FilesHomePageChangeResult {
    fn render(&self) -> maud::Markup {
        maud::html! {
            template {
                div #(FilesHomePageElementId::FileStorageTableWrapper.to_str()) hx-swap-oob=(format_args!("outerHTML:#{}", FilesHomePageElementId::FileStorageTableWrapper.to_str())) {
                    ((FileStorageTable { dir_content: self.dir_content.clone() }))
                }
            }
        }
    }
}
