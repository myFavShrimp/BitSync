use bitsync_core::use_case::user_settings::retrieve_totp_setup_data::TotpSetupData;

static BASE64_PNG_IMG_SRC_PREFIX: &str = "data:image/png;base64";

#[derive(askama::Template)]
#[template(path = "user_settings_page/totp_setup_page.html")]
pub struct TotpSetupPage {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
}

impl From<TotpSetupData> for TotpSetupPage {
    fn from(value: TotpSetupData) -> Self {
        Self {
            totp_secret_image_base64_img_src: format!(
                "{},{}",
                BASE64_PNG_IMG_SRC_PREFIX, value.secret_base64_qr_code
            ),
            totp_secret: value.secret_base32,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "user_settings_page/totp_setup_form.html")]
pub struct TotpSetupForm {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
}

impl From<TotpSetupData> for TotpSetupForm {
    fn from(value: TotpSetupData) -> Self {
        Self {
            totp_secret_image_base64_img_src: format!(
                "{},{}",
                BASE64_PNG_IMG_SRC_PREFIX, value.secret_base64_qr_code
            ),
            totp_secret: value.secret_base32,
        }
    }
}
