use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use axum_extra::routing::TypedPath;

fn build_default_files_query_parameter_path() -> String {
    "/".to_owned()
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/static/{*file_path}")]
pub struct GetStaticFile {
    pub file_path: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/favicon.ico")]
pub struct GetFavicon;

// auth

#[derive(TypedPath, Deserialize)]
#[typed_path("/login")]
pub struct GetLoginPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/login")]
pub struct PostLoginAction;

#[derive(TypedPath, Deserialize)]
#[typed_path("/login/totp-auth")]
pub struct GetLoginTotpAuthPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/login/totp-auth")]
pub struct PostLoginTotpAuthAction;

#[derive(TypedPath, Deserialize)]
#[typed_path("/logout")]
pub struct GetLogoutAction;

#[derive(TypedPath, Deserialize)]
#[typed_path("/suspended")]
pub struct GetSuspendedPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/register")]
pub struct GetRegisterPage;
#[derive(Deserialize, Serialize, Debug)]
pub struct GetRegisterPageQueryParameters {
    pub token: Option<String>,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/register/redeem-token")]
pub struct PostRedeemInviteToken;

#[derive(TypedPath, Deserialize)]
#[typed_path("/register")]
pub struct PostRegisterAction;
#[derive(Deserialize, Serialize, Debug)]
pub struct PostRegisterActionQueryParameters {
    pub token: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/register/totp-setup")]
pub struct GetRegisterTotpSetupPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/register/totp-setup")]
pub struct PostRegisterTotpSetupAction;

// home

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct GetFilesHomePage;
#[derive(Deserialize, Serialize, Debug)]
pub struct GetFilesHomePageQueryParameters {
    #[serde(default = "build_default_files_query_parameter_path")]
    pub path: String,
}

// home actions

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/upload")]
pub struct PostUserFileUpload;
#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileUploadQueryParameters {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserFileDownloadQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/download")]
pub struct GetUserFileDownload;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserFileDeleteQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/delete")]
pub struct GetUserFileDelete;

#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileMoveQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/move")]
pub struct PostUserFileMove;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/create-directory")]
pub struct PostUserFileDirectoryCreation;
#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileDirectoryCreationQueryParameters {
    pub path: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/create-directory/dialog")]
pub struct GetUserFileDirectoryCreationDialog;
#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserFileDirectoryCreationDialogQueryParameters {
    pub path: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/move/dialog")]
pub struct GetUserFileMoveDialog;
#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserFileMoveDialogQueryParameters {
    pub path: String,
}

// search

#[derive(TypedPath, Deserialize)]
#[typed_path("/search")]
pub struct GetSearch;
#[derive(Deserialize, Serialize, Debug)]
pub struct GetSearchQueryParameters {
    pub query: String,
    pub path: Option<String>,
}

// account

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/dialog")]
pub struct GetUserSettingsDialog;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/password")]
pub struct GetUserSettingsPasswordTab;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/password/change")]
pub struct PostUserSettingsChangePassword;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/sessions")]
pub struct GetUserSettingsSessionsTab;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/sessions/terminate/{session_id}")]
pub struct PostTerminateSession {
    pub session_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/sessions/terminate-all-others")]
pub struct PostTerminateAllOtherSessions;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/totp")]
pub struct GetUserSettingsTotpTab;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/totp/initiate-reset")]
pub struct PostUserSettingsTotpInitiateReset;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/totp/reset")]
pub struct PostUserSettingsTotpReset;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users")]
pub struct GetUserSettingsUsersTab;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/make-admin/dialog")]
pub struct GetMakeAdminDialog {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/make-admin")]
pub struct PostUserSettingsMakeAdmin {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/remove-admin/dialog")]
pub struct GetRevokeAdminDialog {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/remove-admin")]
pub struct PostUserSettingsRemoveAdmin {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/reset-totp/dialog")]
pub struct GetResetUserTotpDialog {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/reset-totp")]
pub struct PostUserSettingsResetUserTotp {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/suspend/dialog")]
pub struct GetSuspendUserDialog {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/suspend")]
pub struct PostUserSettingsSuspendUser {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/unsuspend/dialog")]
pub struct GetUnsuspendUserDialog {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/unsuspend")]
pub struct PostUserSettingsUnsuspendUser {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/delete/dialog")]
pub struct GetDeleteUserDialog {
    pub user_id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/users/{user_id}/delete")]
pub struct PostUserSettingsDeleteUser {
    pub user_id: Uuid,
}

// shares

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserFileShareDialogQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/share/dialog")]
pub struct GetUserFileShareDialog;

#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileShareCreateQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/share/create")]
pub struct PostUserFileShareCreate;

#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileShareDeleteQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/share/delete/{user_share_id}")]
pub struct PostUserFileShareDelete {
    pub user_share_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileShareDeleteAllQueryParameters {
    pub path: String,
}
#[derive(TypedPath, Deserialize)]
#[typed_path("/user-file/share/delete-all")]
pub struct PostUserFileShareDeleteAll;

// account

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/shares")]
pub struct GetUserSettingsSharesTab;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/invites")]
pub struct GetUserSettingsInvitesTab;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/invites/create")]
pub struct PostUserSettingsInviteTokenCreate;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/invites/delete/{invite_token_id}")]
pub struct PostUserSettingsInviteTokenDelete {
    pub invite_token_id: Uuid,
}
