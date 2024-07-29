let output_path = "./static/external/"

let static_assets = [
    ["name" "url" "destination"];
    ["MDUI CSS" "https://unpkg.com/mdui@2/mdui.css" "mdui/mdui.css"]
    ["MDUI JS" "https://unpkg.com/mdui@2/mdui.global.js" "mdui/mdui.js"]
    ["Material Symbols Outlined" "https://github.com/google/material-design-icons/raw/master/variablefont/MaterialSymbolsOutlined%5BFILL,GRAD,opsz,wght%5D.woff2" "fonts/material-symbols/outlined.woff2"]
    ["Material Symbols Rounded" "https://github.com/google/material-design-icons/raw/master/variablefont/MaterialSymbolsRounded%5BFILL,GRAD,opsz,wght%5D.woff2" "fonts/material-symbols/rounded.woff2"]
    ["Material Symbols Sharp" "https://github.com/google/material-design-icons/raw/master/variablefont/MaterialSymbolsSharp%5BFILL,GRAD,opsz,wght%5D.woff2" "fonts/material-symbols/sharp.woff2"]
    ["Material Symbols LICENSE" "https://raw.githubusercontent.com/google/material-design-icons/master/LICENSE" "fonts/material-symbols/LICENSE"]
    ["Meyer CSS reset" "https://unpkg.com/reset-css/reset.css" "css/reset.css"]
    ["HTMX" "https://unpkg.com/htmx.org@2.0.1/dist/htmx.min.js" "htmx.js"]
    ["Noto Latin Ext" "https://fonts.gstatic.com/s/notosans/v36/o-0bIpQlx3QUlC5A4PNB6Ryti20_6n1iPHjc5aDdu2ui.woff2" fonts/noto-sans/latin-ext.woff2]
    ["Noto Latin" "https://fonts.gstatic.com/s/notosans/v36/o-0bIpQlx3QUlC5A4PNB6Ryti20_6n1iPHjc5a7duw.woff2" fonts/noto-sans/latin.woff2]
]

def main [
    --all
] {
    let selected_assets = if $all {
        $static_assets
    } else {
        $static_assets
        | input list --multi --display name "Choose assets to update"
    }
    | each {|assets|
        $assets
        | update destination ($output_path + $in.destination)
    }

    $selected_assets
    | each {|asset|
        $asset.destination
        | path dirname
        | mkdir $in
    }

    $selected_assets
    | par-each {|asset|
        http get --raw $asset.url
        | save --force $asset.destination
    }

    {"updated assets": $selected_assets}
}
