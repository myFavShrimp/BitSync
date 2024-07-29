let css_files_path = "./static/external/css/"

let material_fonts = [
    ["name" "location"];
    ["Material Icons Outlined" "./static/external/fonts/material-symbols/outlined.woff2"]
    ["Material Icons Rounded" "./static/external/fonts/material-symbols/rounded.woff2"]
    ["Material Icons Sharp" "./static/external/fonts/material-symbols/sharp.woff2"]
]

let noto_fonts = [
    ["name" "location" "unicode_range"];
    ["Latin Ext" "./static/external/fonts/noto-sans/latin-ext.woff2" "U+0100-02AF, U+0304, U+0308, U+0329, U+1E00-1E9F, U+1EF2-1EFF, U+2020, U+20A0-20AB, U+20AD-20C0, U+2113, U+2C60-2C7F, U+A720-A7FF"]
    ["Latin" "./static/external/fonts/noto-sans/latin.woff2" "U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD"]
]

let noto_fonts_name = "Noto Sans"

def main [
] {
    mkdir $css_files_path

    $material_fonts
    | each {|asset|
        build_material_font_style_sheet $asset.name $asset.location
        | save -f (build_style_sheet_path $asset.name)
    }

    $noto_fonts
    | each {|asset|
        build_noto_font_style_sheet_part $asset.location
    } | str join "\n"
    | save -f (build_style_sheet_path $noto_fonts_name)

    $material_fonts | get name | each {|asset| build_style_sheet_path $asset}
    | append $"(build_style_sheet_path $noto_fonts_name) ($noto_fonts | get name)"

}

def build_material_font_style_sheet [
    asset_name: string
    asset_location: path
] {
$"@font-face {
  font-family: '($asset_name)';
  font-style: normal;
  src: url\(($asset_location | str trim --left -c '.')) format\('woff');
}"
}

def build_noto_font_style_sheet_part [
    asset_location: path
] {
$"@font-face {
  font-family: 'Noto Sans';
  font-style: normal;
  font-weight: 100 900;
  font-stretch: 100%;
  font-display: swap;
  src: url\(($asset_location | str trim --left -c '.')) format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}"
}

def build_style_sheet_path [
    asset_name: string
] {
    [$css_files_path ($asset_name + ".css")] | path join
}
