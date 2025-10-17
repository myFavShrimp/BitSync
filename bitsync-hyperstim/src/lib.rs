use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HyperStimPatchMode {
    Inner,
    Outer,
    Append,
    Prepend,
    Before,
    After,
}

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Serialize)]
#[serde(
    tag = "type",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum HyperStimCommand {
    HsPatchHtml {
        html: String,
        patch_target: String,
        patch_mode: HyperStimPatchMode,
    },
    // Should be array or object but this is sufficient.
    HsPatchSignals(serde_json::Value),
    HsExecute {
        code: String,
    },
}
