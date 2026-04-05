use bitsync_core::use_case::user_files::{
    create_directory::DirectoryCreationResult, delete_user_file::UserFileDeletionResult,
    move_user_file::UserFileMoveResult, read_user_directory_contents::UserDirectoryContentsResult,
    upload_user_file::UserFileResult,
};
use bitsync_routes::TypedPath;
use hypertext::prelude::*;

use crate::{
    Component,
    models::{ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind},
    pages::base::LoggedInDocument,
};

pub enum FilesHomePageElementId {
    FileUploadForm,
    DirectoryCreationDialog,
    FileMoveDialog,
}

impl FilesHomePageElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomePageElementId::FileUploadForm => "file-upload-form",
            FilesHomePageElementId::DirectoryCreationDialog => "directory-creation-dialog",
            FilesHomePageElementId::FileMoveDialog => "file-move-dialog",
        }
    }
}

pub struct FilesHomePage {
    current_path: String,
    dir_content: Vec<StorageItemPresentation>,
    parent_directory_url: Option<ParentDirectoryLink>,
    file_upload_url: String,
    directory_creation_url: String,
    directory_creation_dialog_id: String,
    file_move_dialog_id: String,
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
            current_path: value.path.path(),
            dir_content: displayable_dir_content,
            parent_directory_url: ParentDirectoryLink::from_child(value.path.scoped_path),
            file_upload_url,
            directory_creation_url,
            directory_creation_dialog_id: FilesHomePageElementId::DirectoryCreationDialog
                .to_str()
                .to_owned(),
            file_move_dialog_id: FilesHomePageElementId::FileMoveDialog.to_str().to_owned(),
        }
    }
}

impl Renderable for FilesHomePage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            LoggedInDocument current_path=(Some(self.current_path.clone())) {
                style { (crate::styles::files_home_page::STYLE_SHEET) }
                main
                    data-init=(format!(
                        "initDropUpload(this, document.getElementById('{form_id}'), '{active_class}')",
                        form_id = FilesHomePageElementId::FileUploadForm.to_str(),
                        active_class = crate::styles::files_home_page::ClassName::DROP_ZONE_ACTIVE,
                    ))
                {
                    @match &self.parent_directory_url {
                        Some(link_data) => {
                            a class=(crate::styles::files_home_page::ClassName::BREADCRUMB) href=(link_data.parent_directory_url) {
                                (crate::icons::chevron_left::ChevronLeft)
                                (link_data.current_directory_name)
                            }
                        }
                        None => {}
                    }

                    div class=(crate::styles::files_home_page::ClassName::ACTIONS) {
                        FileUploadForm file_upload_url=(self.file_upload_url.clone());

                        button
                            class=(crate::styles::files_home_page::ClassName::ACTION_BUTTON)
                            title="New Folder"
                            onclick=(format_args!("openDialogModalById('{}')", &self.directory_creation_dialog_id))
                        {
                            (crate::icons::folder_plus::FolderPlus)
                        }
                    }

                    dialog
                        class=(crate::styles::modal::ClassName::MODAL)
                        id=(self.directory_creation_dialog_id)
                        onclick="if (event.target === this) this.close()"
                    {
                        div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                            h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { "Create New Folder" }
                            button class=(crate::styles::modal::ClassName::MODAL_CLOSE) onclick="closeClosestDialog(this)" { "×" }
                        }
                        form
                            data-hijack
                            action=(self.directory_creation_url)
                            method="POST"
                        {
                            div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                                label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                                    "Folder Name"
                                    input class=(crate::styles::base::ClassName::FORM_CONTROL) type="text" name="directory_name" placeholder="Enter folder name";
                                }
                            }
                            div class=(crate::styles::modal::ClassName::MODAL_ACTIONS) {
                                button type="button" class=(crate::styles::modal::ClassName::MODAL_BUTTON) onclick="closeClosestDialog(this)" { "Cancel" }
                                button type="submit" class=(format!("{} {}", crate::styles::modal::ClassName::MODAL_BUTTON, crate::styles::modal::ClassName::MODAL_BUTTON_PRIMARY)) { "Create" }
                            }
                        }
                    }

                    dialog
                        class=(crate::styles::modal::ClassName::MODAL)
                        id=(self.file_move_dialog_id)
                        onclick="if (event.target === this) this.close()"
                    {
                        div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                            h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { "Move Item" }
                            button class=(crate::styles::modal::ClassName::MODAL_CLOSE) onclick="closeClosestDialog(this)" { "×" }
                        }
                        form
                            data-hijack
                            method="POST"
                        {
                            div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                                label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                                    "Destination Path"
                                    input class=(crate::styles::base::ClassName::FORM_CONTROL) type="text" name="destination_path" placeholder="Enter destination path";
                                }
                            }
                            div class=(crate::styles::modal::ClassName::MODAL_ACTIONS) {
                                button type="button" class=(crate::styles::modal::ClassName::MODAL_BUTTON) onclick="closeClosestDialog(this)" { "Cancel" }
                                button type="submit" class=(format!("{} {}", crate::styles::modal::ClassName::MODAL_BUTTON, crate::styles::modal::ClassName::MODAL_BUTTON_PRIMARY)) { "Move" }
                            }
                        }
                    }

                    ((FileStorageTable { dir_content: self.dir_content.clone() }))
                }
            }
        }.render_to(buffer);
    }
}

struct FileUploadForm {
    file_upload_url: String,
}

impl Component for FileUploadForm {
    fn id(&self) -> String {
        FilesHomePageElementId::FileUploadForm.to_str().to_string()
    }
}

impl Renderable for FileUploadForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                data-hijack
                action=(self.file_upload_url)
                method="POST"
                enctype="multipart/form-data"
            {
                input type="file" name="upload" hidden onchange="this.form.requestSubmit()";
                button
                    type="button"
                    class=(
                        crate::styles::files_home_page::ClassName::ACTION_BUTTON, " ",
                        crate::styles::files_home_page::ClassName::ACTION_BUTTON_PRIMARY,
                    )
                    title="Upload Files"
                    onclick="this.previousElementSibling.click()"
                    data-effect=("this.disabled = this.form.hsFetch.state() === 'pending'")
                {
                    div class=(crate::styles::files_home_page::ClassName::SPINNER) {}
                    (crate::icons::upload::Upload)
                }
            }
        }
        .render_to(buffer);
    }
}

static FILE_STORAGE_TABLE_ID: &str = "file-storage-table";

struct FileStorageTable {
    dir_content: Vec<StorageItemPresentation>,
}

impl Component for FileStorageTable {
    fn id(&self) -> String {
        FILE_STORAGE_TABLE_ID.to_owned()
    }
}

impl Renderable for FileStorageTable {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            table id=(self.id()) class=(crate::styles::files_home_page::ClassName::FILE_BROWSER) {
                @if self.dir_content.is_empty() {
                    tbody {
                        tr {
                            td class=(crate::styles::files_home_page::ClassName::EMPTY_STATE) colspan="4" {
                                (crate::icons::cloudy::Cloudy)
                                p { "This folder is empty" }
                            }
                        }
                    }
                } @else {
                    thead {
                        tr class=(crate::styles::files_home_page::ClassName::FILE_HEADER) {
                            th {}
                            th { "Name" }
                            th { "Size" }
                            th {}
                        }
                    }
                    tbody {
                        @for dir_item in &self.dir_content {
                            tr class=(crate::styles::files_home_page::ClassName::FILE_ITEM) {
                                @match &dir_item.kind {
                                    StorageItemPresentationKind::Directory { url } => {
                                        td class=(crate::styles::files_home_page::ClassName::FILE_ICON) {
                                            (crate::icons::folder::Folder)
                                        }
                                        td class=(crate::styles::files_home_page::ClassName::FILE_NAME) {
                                            a href=(url) { (dir_item.name) }
                                        }
                                    }
                                    StorageItemPresentationKind::File => {
                                        td class=(format!("{} {}", crate::styles::files_home_page::ClassName::FILE_ICON, crate::styles::files_home_page::ClassName::FILE_ICON_SECONDARY)) {
                                            (crate::icons::file_text::FileText)
                                        }
                                        td class=(crate::styles::files_home_page::ClassName::FILE_NAME) {
                                            (dir_item.name)
                                        }
                                    }
                                }
                                td class=(crate::styles::files_home_page::ClassName::FILE_SIZE) {
                                    @match &dir_item.kind {
                                        StorageItemPresentationKind::Directory { .. } => { "\u{2014}" }
                                        StorageItemPresentationKind::File => { (dir_item.size) }
                                    }
                                }
                                td class=(crate::styles::files_home_page::ClassName::FILE_ACTIONS) {
                                    button
                                        title="More"
                                        class=(crate::styles::files_home_page::ClassName::FILE_ACTION_BUTTON)
                                        popovertarget=(dir_item.actions_popover_id)
                                    {
                                        (crate::icons::ellipsis_vertical::EllipsisVertical)
                                    }
                                    dialog class=(format!("{} {}", crate::styles::base::ClassName::CONTEXT_MENU, crate::styles::files_home_page::ClassName::FILE_CONTEXT_MENU)) popover id=(dir_item.actions_popover_id) {
                                        button
                                            class=(crate::styles::base::ClassName::CONTEXT_MENU_ITEM)
                                            onclick=(format_args!(
                                                "closeClosestPopover(this); openMoveModal('{}', '{}', '{}')",
                                                FilesHomePageElementId::FileMoveDialog.to_str(),
                                                dir_item.move_url,
                                                dir_item.path,
                                            ))
                                        {
                                            span { "Move" }
                                        }
                                        a class=(crate::styles::base::ClassName::CONTEXT_MENU_ITEM) href=(dir_item.download_url) onclick="closeClosestDialog(this)" {
                                            span { "Download" }
                                        }
                                        div class=(crate::styles::base::ClassName::CONTEXT_MENU_DIVIDER) {}
                                        button
                                            class=(format!("{} {}", crate::styles::base::ClassName::CONTEXT_MENU_ITEM, crate::styles::base::ClassName::CONTEXT_MENU_ITEM_DANGER))
                                            data-init=(format!("this.fetch = fetch('{}')", (dir_item.delete_url)))
                                            data-on-click="this.fetch.trigger(), closeClosestDialog(this)"
                                        {
                                            span { "Delete" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }.render_to(buffer);
    }
}

pub struct FilesHomePageChangeResult {
    dir_content: Vec<StorageItemPresentation>,
}

impl Component for FilesHomePageChangeResult {
    fn id(&self) -> String {
        FILE_STORAGE_TABLE_ID.to_owned()
    }
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

impl Renderable for FilesHomePageChangeResult {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            ((FileStorageTable { dir_content: self.dir_content.clone() }))
        }
        .render_to(buffer);
    }
}
