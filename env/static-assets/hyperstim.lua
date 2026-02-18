local helpers = require("helpers")

local hyperstim_tag = "build-696a247b30db247e71f5495fbb17496b39ea43e8"

local destination = helpers.paths.static_external .. "hyperstim.js"

tasks["fetch_hyperstim"] = {
    handler = function()
        host:directory(helpers.paths.static_external):create()

        local url = "https://github.com/myFavShrimp/HyperStim/releases/download/" .. hyperstim_tag .. "/hyperstim.min.js"

        local result = host:run_command("curl -sL -o " .. destination .. " " .. url)
        if result.exit_code ~= 0 then
            error("Failed to fetch HyperStim: " .. result.stderr)
        end

        log.info("Fetched HyperStim (" .. hyperstim_tag .. ")")
    end,
}
