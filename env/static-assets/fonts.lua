local helpers = require("helpers")

local css_output_path = helpers.paths.static_external_css

local static_assets = {
    {
        name = "Meyer CSS reset",
        url = "https://unpkg.com/reset-css/reset.css",
        destination = "css/reset.css",
    },
    {
        name = "Noto Latin Ext",
        url = "https://fonts.gstatic.com/s/notosans/v36/o-0IIpQlx3QUlC5A4PNr6zRAW_0.woff2",
        destination = "fonts/noto-sans/latin-ext.woff2",
    },
    {
        name = "Noto Latin",
        url = "https://fonts.gstatic.com/s/notosans/v36/o-0IIpQlx3QUlC5A4PNr5TRA.woff2",
        destination = "fonts/noto-sans/latin.woff2",
    },
}

local material_fonts = {
    {
        name = "Material Icons Outlined",
        location = "/static/external/fonts/material-symbols/outlined.woff2",
    },
    {
        name = "Material Icons Rounded",
        location = "/static/external/fonts/material-symbols/rounded.woff2",
    },
    {
        name = "Material Icons Sharp",
        location = "/static/external/fonts/material-symbols/sharp.woff2",
    },
}

local noto_fonts = {
    {
        name = "Latin Ext",
        location = "/static/external/fonts/noto-sans/latin-ext.woff2",
    },
    {
        name = "Latin",
        location = "/static/external/fonts/noto-sans/latin.woff2",
    },
}

tasks["fetch_static_assets"] = {
    handler = function()
        local output_dir = helpers.paths.static_external

        for _, asset in ipairs(static_assets) do
            local dest = output_dir .. asset.destination
            local dir = dest:match("(.*/)")

            host:directory(dir):create()

            local result = host:run_command("curl -sL -o " .. dest .. " " .. asset.url)
            if result.exit_code ~= 0 then
                error("Failed to fetch " .. asset.name .. ": " .. result.stderr)
            end

            log.info("Fetched " .. asset.name)
        end
    end,
}

tasks["generate_font_css"] = {
    handler = function()
        host:directory(css_output_path):create()

        local material_template = host:file(arc.project_root_path .. "/templates/material_font.css").content

        for _, font in ipairs(material_fonts) do
            host:file(css_output_path .. font.name .. ".css").content = template.render(material_template, font)

            log.info("Generated " .. font.name .. ".css")
        end

        local noto_template = host:file(arc.project_root_path .. "/templates/noto_font.css").content

        local noto_parts = {}

        for _, font in ipairs(noto_fonts) do
            table.insert(noto_parts, template.render(noto_template, font))
        end

        host:file(css_output_path .. "Noto Sans.css").content = table.concat(noto_parts, "\n")

        log.info("Generated Noto Sans.css")
    end,
}

tasks["static_assets"] = {
    requires = { "fetch_static_assets", "generate_font_css", "fetch_hyperstim" },
    handler = function()
        log.info("All static assets ready")
    end,
}
