local paths = {}

paths.project_root = arc.project_root_path .. "/.."
paths.static_external = paths.project_root .. "/static/external/"
paths.static_external_css = paths.static_external .. "css/"

return paths
