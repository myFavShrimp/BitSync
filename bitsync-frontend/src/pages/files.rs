pub mod directory_creation;
pub mod file_move;
pub mod file_operations;
pub mod file_share;

use bitsync_core::use_case::user_files::{
    create_directory::DirectoryCreationResult,
    delete_user_file::UserFileDeletionResult,
    move_user_file::UserFileMoveResult,
    read_user_file_item::{
        DirectoryBreadcrumbSegment, UserDirectoryContentsResult, UserFileItemResult,
        UserFilesHomeResult,
    },
    upload_user_file::UserFileResult,
};
use bitsync_routes::TypedPath;
use hypertext::prelude::*;

use crate::{
    Component,
    models::{StorageItemPresentation, StorageItemPresentationKind},
    pages::base::LoggedInDocument,
};

pub enum FilesHomePageElementId {
    FileUploadForm,
    DirectoryCreationDialog,
    FileMoveDialog,
    FileShareDialog,
}

impl FilesHomePageElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomePageElementId::FileUploadForm => "file-upload-form",
            FilesHomePageElementId::DirectoryCreationDialog => "directory-creation-dialog",
            FilesHomePageElementId::FileMoveDialog => "file-move-dialog",
            FilesHomePageElementId::FileShareDialog => "file-share-dialog",
        }
    }
}

pub struct DirectoryHeader {
    pub directory_name: String,
    pub download_zip_url: String,
    pub share_dialog_url: String,
    pub move_dialog_url: String,
    pub delete_url: String,
    pub actions_popover_id: String,
    pub is_root_directory: bool,
}

pub struct BreadcrumbLink {
    pub name: String,
    pub url: String,
}

pub enum BreadcrumbCrumb {
    Link(BreadcrumbLink),
    CollapsedGroup { hidden_links: Vec<BreadcrumbLink> },
}

const BREADCRUMB_MAX_VISIBLE: usize = 3;
const BREADCRUMB_COLLAPSED_POPOVER_ID: &str = "breadcrumb-collapsed-popover";

const _: () = assert!(BREADCRUMB_MAX_VISIBLE > 2);

fn to_breadcrumb_link(segment: DirectoryBreadcrumbSegment) -> BreadcrumbLink {
    let url = bitsync_routes::GetFilesHomePage
        .with_query_params(bitsync_routes::GetFilesHomePageQueryParameters { path: segment.path })
        .to_string();

    BreadcrumbLink {
        name: segment.name,
        url,
    }
}

fn build_breadcrumb(segments: Vec<DirectoryBreadcrumbSegment>) -> Vec<BreadcrumbCrumb> {
    let total = segments.len();

    if total <= BREADCRUMB_MAX_VISIBLE {
        return segments
            .into_iter()
            .map(to_breadcrumb_link)
            .map(BreadcrumbCrumb::Link)
            .collect();
    }

    let mut segments = segments;
    let tail_start = total - (BREADCRUMB_MAX_VISIBLE - 2);
    let tail: Vec<DirectoryBreadcrumbSegment> = segments.drain(tail_start..).collect();
    let middle: Vec<DirectoryBreadcrumbSegment> = segments.drain(1..).collect();
    let root = segments
        .into_iter()
        .next()
        .expect("root segment always present");

    let mut crumbs = vec![
        BreadcrumbCrumb::Link(to_breadcrumb_link(root)),
        BreadcrumbCrumb::CollapsedGroup {
            hidden_links: middle.into_iter().map(to_breadcrumb_link).collect(),
        },
    ];
    crumbs.extend(
        tail.into_iter()
            .map(to_breadcrumb_link)
            .map(BreadcrumbCrumb::Link),
    );

    crumbs
}

pub enum FilesHomePage {
    Directory(FilesHomeDirectoryPage),
    File(FilesHomeFilePage),
}

impl From<UserFilesHomeResult> for FilesHomePage {
    fn from(value: UserFilesHomeResult) -> Self {
        match value {
            UserFilesHomeResult::Directory(directory_result) => {
                FilesHomePage::Directory(directory_result.into())
            }
            UserFilesHomeResult::File(file_result) => FilesHomePage::File(file_result.into()),
        }
    }
}

impl Renderable for FilesHomePage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        match self {
            FilesHomePage::Directory(page) => page.render_to(buffer),
            FilesHomePage::File(page) => page.render_to(buffer),
        }
    }
}

pub struct FilesHomeDirectoryPage {
    current_path: String,
    dir_content: Vec<StorageItemPresentation>,
    breadcrumb: Vec<BreadcrumbCrumb>,
    directory_header: DirectoryHeader,
    file_upload_url: String,
    directory_creation_dialog_url: String,
}

impl From<UserDirectoryContentsResult> for FilesHomeDirectoryPage {
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

        let directory_creation_dialog_url = bitsync_routes::GetUserFileDirectoryCreationDialog
            .with_query_params(
                bitsync_routes::GetUserFileDirectoryCreationDialogQueryParameters {
                    path: value.path.path(),
                },
            )
            .to_string();

        let directory_header = DirectoryHeader {
            directory_name: value.directory_name,
            download_zip_url: bitsync_routes::GetUserFileDownload
                .with_query_params(bitsync_routes::GetUserFileDownloadQueryParameters {
                    path: value.path.path(),
                })
                .to_string(),
            share_dialog_url: bitsync_routes::GetUserFileShareDialog
                .with_query_params(bitsync_routes::GetUserFileShareDialogQueryParameters {
                    path: value.path.path(),
                })
                .to_string(),
            move_dialog_url: bitsync_routes::GetUserFileMoveDialog
                .with_query_params(bitsync_routes::GetUserFileMoveDialogQueryParameters {
                    path: value.path.path(),
                })
                .to_string(),
            delete_url: bitsync_routes::GetUserFileDelete
                .with_query_params(bitsync_routes::GetUserFileDeleteQueryParameters {
                    path: value.path.path(),
                })
                .to_string(),
            actions_popover_id: "directory-header-actions-popover".to_owned(),
            is_root_directory: value.is_root_directory,
        };

        let breadcrumb = build_breadcrumb(value.breadcrumb_segments);

        FilesHomeDirectoryPage {
            current_path: value.path.path(),
            dir_content: displayable_dir_content,
            breadcrumb,
            directory_header,
            file_upload_url,
            directory_creation_dialog_url,
        }
    }
}

impl Renderable for FilesHomeDirectoryPage {
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
                    div class=(crate::styles::files_home_page::ClassName::DIRECTORY_HEADER) {
                        button
                            title="More"
                            class=(
                                crate::styles::button::ClassName::BUTTON, " ",
                                crate::styles::files_home_page::ClassName::ACTION_BUTTON,
                            )
                            popovertarget=(self.directory_header.actions_popover_id)
                        {
                            div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                            (crate::icons::FolderOpen::default())
                        }

                        div style="display: flex; flex-direction: column; gap: 0.2em;" {
                            h1 class=(crate::styles::files_home_page::ClassName::DIRECTORY_HEADER_TITLE) {
                                (self.directory_header.directory_name)
                            }

                            nav class=(crate::styles::files_home_page::ClassName::BREADCRUMB) {
                                @for crumb in self.breadcrumb.iter() {
                                    @match crumb {
                                        BreadcrumbCrumb::Link(link) => {
                                            a
                                                class=(crate::styles::files_home_page::ClassName::BREADCRUMB_LINK)
                                                href=(link.url)
                                            {
                                                (link.name)
                                            }
                                        }
                                        BreadcrumbCrumb::CollapsedGroup { hidden_links } => {
                                            button
                                                class=(crate::styles::files_home_page::ClassName::BREADCRUMB_ELLIPSIS)
                                                popovertarget=(BREADCRUMB_COLLAPSED_POPOVER_ID)
                                                title="Show hidden folders"
                                            {
                                                "..."
                                            }
                                            div
                                                id=(BREADCRUMB_COLLAPSED_POPOVER_ID)
                                                class=(
                                                    crate::styles::context_menu::ClassName::CONTEXT_MENU, " ",
                                                    crate::styles::context_menu::ClassName::ANCHOR_TOP_LEFT,
                                                )
                                                popover
                                            {
                                                @for link in hidden_links {
                                                    a
                                                        class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                                        href=(link.url)
                                                        onclick="closeClosestPopover(this)"
                                                    {
                                                        span { (link.name) }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    span class=(crate::styles::files_home_page::ClassName::BREADCRUMB_SEPARATOR) {
                                        "/"
                                    }
                                }
                            }
                        }

                        dialog
                            id=(self.directory_header.actions_popover_id)
                            class=(
                                crate::styles::context_menu::ClassName::CONTEXT_MENU, " ",
                                crate::styles::context_menu::ClassName::ANCHOR_TOP_LEFT,
                            )
                            popover
                        {
                            button
                                class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                data-init=(format!(
                                    "this.uploadInput = document.getElementById('{form_id}').querySelector('input[type=file]')",
                                    form_id = FilesHomePageElementId::FileUploadForm.to_str(),
                                ))
                                data-on-click="closeClosestPopover(this), this.uploadInput.click()"
                            {
                                (crate::icons::Upload::default())
                                span { "Upload" }
                            }

                            button
                                class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", self.directory_creation_dialog_url))
                                data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                (crate::icons::FolderPlus::default())
                                span { "New Folder" }
                            }

                            div class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_DIVIDER) {}

                            @if !self.dir_content.is_empty() {
                                a
                                    class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                    href=(self.directory_header.download_zip_url)
                                {
                                    (crate::icons::Download::default())
                                    span { "Download" }
                                }
                            }

                            @if !self.directory_header.is_root_directory {
                                button
                                    class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                    data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", self.directory_header.move_dialog_url))
                                    data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                    data-effect=(format!(
                                        "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                        loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                    ))
                                {
                                    (crate::icons::Move::default())
                                    span { "Move" }
                                }
                            }

                            button
                                class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", self.directory_header.share_dialog_url))
                                data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                (crate::icons::Share2::default())
                                span { "Share" }
                            }

                            @if !self.directory_header.is_root_directory {
                                div class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_DIVIDER) {}

                                button
                                    class=(
                                        crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM, " ",
                                        crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM_DANGER,
                                    )
                                    data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", self.directory_header.delete_url))
                                    data-on-click="this.fetch.trigger(), closeClosestDialog(this)"
                                    data-effect=(format!(
                                        "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                        loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                    ))
                                {
                                    (crate::icons::Trash2::default())
                                    span { "Delete" }
                                }
                            }
                        }
                    }

                    FileUploadForm file_upload_url=(self.file_upload_url.clone());

                    (FileStorageTable { dir_content: self.dir_content.clone() })
                }
            }
        }.render_to(buffer);
    }
}

pub struct FilesHomeFilePage {
    current_path: String,
    file_name: String,
    download_url: String,
    share_dialog_url: String,
    move_dialog_url: String,
    delete_url: String,
    actions_popover_id: String,
    breadcrumb: Vec<BreadcrumbCrumb>,
}

impl From<UserFileItemResult> for FilesHomeFilePage {
    fn from(value: UserFileItemResult) -> Self {
        let download_url = bitsync_routes::GetUserFileDownload
            .with_query_params(bitsync_routes::GetUserFileDownloadQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let share_dialog_url = bitsync_routes::GetUserFileShareDialog
            .with_query_params(bitsync_routes::GetUserFileShareDialogQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let move_dialog_url = bitsync_routes::GetUserFileMoveDialog
            .with_query_params(bitsync_routes::GetUserFileMoveDialogQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let delete_url = bitsync_routes::GetUserFileDelete
            .with_query_params(bitsync_routes::GetUserFileDeleteQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let breadcrumb = build_breadcrumb(value.breadcrumb_segments);

        FilesHomeFilePage {
            current_path: value.path.path(),
            file_name: value.file_name,
            download_url,
            share_dialog_url,
            move_dialog_url,
            delete_url,
            actions_popover_id: "file-header-actions-popover".to_owned(),
            breadcrumb,
        }
    }
}

impl Renderable for FilesHomeFilePage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            LoggedInDocument current_path=(Some(self.current_path.clone())) {
                style { (crate::styles::files_home_page::STYLE_SHEET) }
                main {
                    div class=(crate::styles::files_home_page::ClassName::FILE_HEADER_BANNER) {
                        button
                            title="More"
                            class=(crate::styles::files_home_page::ClassName::FILE_HEADER_ICON)
                            popovertarget=(self.actions_popover_id)
                        {
                            div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                            (crate::icons::FileInput::default())
                        }

                        dialog
                            id=(self.actions_popover_id)
                            class=(
                                crate::styles::context_menu::ClassName::CONTEXT_MENU, " ",
                                crate::styles::context_menu::ClassName::ANCHOR_TOP_LEFT,
                            )
                            popover
                        {
                            button
                                class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", self.move_dialog_url))
                                data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                (crate::icons::Move::default())
                                span { "Move" }
                            }

                            div class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_DIVIDER) {}

                            button
                                class=(
                                    crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM, " ",
                                    crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM_DANGER,
                                )
                                data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", self.delete_url))
                                data-on-click="this.fetch.trigger(), closeClosestDialog(this)"
                                data-effect=(format!(
                                    "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                (crate::icons::Trash2::default())
                                span { "Delete" }
                            }
                        }

                        div class=(crate::styles::files_home_page::ClassName::FILE_HEADER_TEXT) {
                            h1 class=(crate::styles::files_home_page::ClassName::FILE_HEADER_TITLE) {
                                (self.file_name)
                            }

                            nav class=(crate::styles::files_home_page::ClassName::BREADCRUMB) {
                                @for crumb in self.breadcrumb.iter() {
                                    @match crumb {
                                        BreadcrumbCrumb::Link(link) => {
                                            a
                                                class=(crate::styles::files_home_page::ClassName::BREADCRUMB_LINK)
                                                href=(link.url)
                                            {
                                                (link.name)
                                            }
                                        }
                                        BreadcrumbCrumb::CollapsedGroup { hidden_links } => {
                                            button
                                                class=(crate::styles::files_home_page::ClassName::BREADCRUMB_ELLIPSIS)
                                                popovertarget=(BREADCRUMB_COLLAPSED_POPOVER_ID)
                                                title="Show hidden folders"
                                            {
                                                "..."
                                            }
                                            div
                                                id=(BREADCRUMB_COLLAPSED_POPOVER_ID)
                                                class=(
                                                    crate::styles::context_menu::ClassName::CONTEXT_MENU, " ",
                                                    crate::styles::context_menu::ClassName::ANCHOR_TOP_LEFT,
                                                )
                                                popover
                                            {
                                                @for link in hidden_links {
                                                    a
                                                        class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                                        href=(link.url)
                                                        onclick="closeClosestPopover(this)"
                                                    {
                                                        span { (link.name) }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    span class=(crate::styles::files_home_page::ClassName::BREADCRUMB_SEPARATOR) {
                                        "/"
                                    }
                                }
                            }
                        }

                        div class=(crate::styles::files_home_page::ClassName::FILE_HEADER_ACTIONS) {
                            button
                                title="Share"
                                class=(
                                    crate::styles::button::ClassName::BUTTON, " ",
                                    crate::styles::files_home_page::ClassName::FILE_HEADER_SHARE,
                                )
                                data-init=(format!("this.fetch = fetch('{}')", self.share_dialog_url))
                                data-on-click__throttle.1s="this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                                (crate::icons::Share2::default())
                                span { "Share" }
                            }

                            a
                                title="Download"
                                class=(
                                    crate::styles::button::ClassName::BUTTON, " ",
                                    crate::styles::button::ClassName::BUTTON_PRIMARY, " ",
                                    crate::styles::files_home_page::ClassName::FILE_HEADER_DOWNLOAD,
                                )
                                href=(self.download_url)
                            {
                                (crate::icons::Download::default())
                                span { "Download" }
                            }
                        }
                    }
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
                input
                    type="file"
                    name="upload"
                    hidden
                    onchange="this.form.requestSubmit()"
                ;
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
            table
                id=(self.id())
                class=(crate::styles::files_home_page::ClassName::FILE_BROWSER)
            {
                @if self.dir_content.is_empty() {
                    tbody {
                        tr {
                            td
                                class=(crate::styles::files_home_page::ClassName::EMPTY_STATE)
                                colspan="4"
                            {
                                (crate::icons::Cloudy::default())
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
                                            (crate::icons::Folder::default())
                                        }

                                        td class=(crate::styles::files_home_page::ClassName::FILE_NAME) {
                                            a href=(url) { (dir_item.name) }
                                        }
                                    }
                                    StorageItemPresentationKind::File { url } => {
                                        td
                                            class=(
                                                crate::styles::files_home_page::ClassName::FILE_ICON, " ",
                                                crate::styles::files_home_page::ClassName::FILE_ICON_SECONDARY,
                                            )
                                        {
                                            (crate::icons::FileText::default())
                                        }

                                        td class=(crate::styles::files_home_page::ClassName::FILE_NAME) {
                                            a href=(url) { (dir_item.name) }
                                        }
                                    }
                                }

                                td class=(crate::styles::files_home_page::ClassName::FILE_SIZE) {
                                    @match &dir_item.kind {
                                        StorageItemPresentationKind::Directory { .. } => { "\u{2014}" }
                                        StorageItemPresentationKind::File { .. } => { (dir_item.size) }
                                    }
                                }

                                td class=(crate::styles::files_home_page::ClassName::FILE_ACTIONS) {
                                    button
                                        title="More"
                                        class=(crate::styles::files_home_page::ClassName::FILE_ACTION_BUTTON)
                                        popovertarget=(dir_item.actions_popover_id)
                                    {
                                        div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                                        (crate::icons::EllipsisVertical::default())
                                    }

                                    dialog
                                        id=(dir_item.actions_popover_id)
                                        class=(
                                            crate::styles::context_menu::ClassName::CONTEXT_MENU, " ",
                                            crate::styles::context_menu::ClassName::ANCHOR_RIGHT_CENTERED,
                                        )
                                        popover
                                    {
                                        button
                                            class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                            data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", dir_item.share_dialog_url))
                                            data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                            data-effect=(format!(
                                                "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                                loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                            ))
                                        {
                                            (crate::icons::Share2::default())
                                            span { "Share" }
                                        }

                                        button
                                            class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                            data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", dir_item.move_dialog_url))
                                            data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                            data-effect=(format!(
                                                "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                                loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                            ))
                                        {
                                            (crate::icons::Move::default())
                                            span { "Move" }
                                        }

                                        a
                                            class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                            href=(dir_item.download_url)
                                            onclick="closeClosestDialog(this)"
                                        {
                                            (crate::icons::Download::default())
                                            span { "Download" }
                                        }

                                        div class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_DIVIDER) {}

                                        button
                                            class=(
                                                crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM, " ",
                                                crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM_DANGER,
                                            )
                                            data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", dir_item.delete_url))
                                            data-on-click="this.fetch.trigger(), closeClosestDialog(this)"
                                            data-effect=(format!(
                                                "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                                loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                            ))
                                        {
                                            (crate::icons::Trash2::default())
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
            (FileStorageTable { dir_content: self.dir_content.clone() })
        }
        .render_to(buffer);
    }
}
