let destination_dir = "./templates"
let origin_dir = "./templates-src"

def main [] {
    rm -rf ($destination_dir + '/*')
    let original_files = (glob -D templates-src/**/*.html | each {|file| $file | str replace (pwd) '.'})

    $original_files | each {|file| destination_path $file}
}

def destination_path [original_path: string] {
    let destination_file = ($original_path | str replace $origin_dir '' | $destination_dir + $in)

    let parent_dir = $destination_file | path dirname
    mkdir $parent_dir

    let minified = minhtml --do-not-minify-doctype --ensure-spec-compliant-unquoted-attribute-values --keep-spaces-between-attributes --preserve-brace-template-syntax $original_path

    $minified | save -f $destination_file

    $destination_file
}
