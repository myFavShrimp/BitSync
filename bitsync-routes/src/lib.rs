use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};

fn build_default_files_query_parameter_path() -> String {
    "/".to_owned()
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/static/*file_path")]
pub struct GetStaticFile {
    pub file_path: String,
}

// auth

#[derive(TypedPath, Deserialize)]
#[typed_path("/login")]
pub struct GetLoginPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/login")]
pub struct PostLoginAction;

#[derive(TypedPath, Deserialize)]
#[typed_path("/logout")]
pub struct GetLogoutAction;

#[derive(TypedPath, Deserialize)]
#[typed_path("/register")]
pub struct GetRegisterPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/register")]
pub struct PostRegisterAction;

#[derive(TypedPath, Deserialize)]
#[typed_path("/totp-auth")]
pub struct GetTotpAuthPage;

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
#[typed_path("/user-file/create_directory")]
pub struct PostUserFileDirectoryCreation;
#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileDirectoryCreationQueryParameters {
    pub path: String,
}

// account

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings")]
pub struct GetUserSettingsPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/change-password")]
pub struct PostUserSettingsChangePassword;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/totp-setup")]
pub struct GetTotpSetupPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/totp-setup")]
pub struct PostTotpSetup;
