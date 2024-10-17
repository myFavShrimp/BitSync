let output_path = "./static/external/"

let static_assets = [
    ["name" "url" "destination"];

    ["Meyer CSS reset" "https://unpkg.com/reset-css/reset.css" "css/reset.css"]

    ["HTMX" "https://unpkg.com/htmx.org@2.0.1/dist/htmx.min.js" "htmx.js"]

    ["Noto Latin Ext" "https://fonts.gstatic.com/s/notosans/v36/o-0IIpQlx3QUlC5A4PNr6zRAW_0.woff2" fonts/noto-sans/latin-ext.woff2]
    ["Noto Latin" "https://fonts.gstatic.com/s/notosans/v36/o-0IIpQlx3QUlC5A4PNr5TRA.woff2" fonts/noto-sans/latin.woff2]
]

def main [
    --all
] {
    rm -rf $output_path

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
