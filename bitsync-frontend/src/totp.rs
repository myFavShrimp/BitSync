static BASE64_PNG_IMG_SRC_PREFIX: &str = "data:image/png;base64";

pub fn totp_qr_src(base64_image: &str) -> String {
    format!("{},{}", BASE64_PNG_IMG_SRC_PREFIX, base64_image)
}
