use std::sync::OnceLock;

static CONFIG_STRING: OnceLock<String> = OnceLock::new();

pub fn retrieve_config() -> &'static str {
    CONFIG_STRING.get_or_init(|| {
        serde_json::json!(
            {
                "defaultSwapStyle": "none",
                "responseHandling": [
                    {
                        "code": "[1234]..",
                        "swap": true,
                        "error": false,
                    },
                    {
                        "code": "5..",
                        "swap": false,
                        "error": true,
                    },
                ],
            }
        )
        .to_string()
    })
}
