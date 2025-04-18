use std::sync::LazyLock;

pub static CONFIG: LazyLock<String> = LazyLock::new(|| {
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
});
