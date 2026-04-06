use serde::{Deserialize, Serialize};

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
#[typed_path("/user-file/create_directory")]
pub struct PostUserFileDirectoryCreation;
#[derive(Deserialize, Serialize, Debug)]
pub struct PostUserFileDirectoryCreationQueryParameters {
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
#[typed_path("/user-settings")]
pub struct GetUserSettingsPage;

#[derive(TypedPath, Deserialize)]
#[typed_path("/user-settings/change-password")]
pub struct PostUserSettingsChangePassword;
