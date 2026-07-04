
function runInConsole(command)

    vim.cmd('wa')
    vim.cmd('split')
    vim.cmd('term ' .. command)
    vim.cmd('setlocal nospell')
    vim.api.nvim_input('G')
end


vim.keymap.set('n', '<C-b>', function()

    runInConsole('cargo build')
end)


vim.keymap.set('n', '<F5>', function()

    runInConsole('cargo run')
end)


vim.keymap.set('n', '<C-c>', function()

    runInConsole('cargo clippy -- ' ..
                 '-Wclippy::all ' ..
                 '-Wclippy::pedantic ' ..
                 '-Wclippy::unwrap_used ' ..
                 '-Aclippy::module_inception')
end)


vim.api.nvim_create_user_command("WebDemo", function()

    local demoName = 'text_rendering'

    runInConsole('cargo build --example ' .. demoName .. ' --release --target wasm32-unknown-unknown && ' ..
                 'wasm-bindgen target/wasm32-unknown-unknown/release/examples/' .. demoName .. '.wasm ' ..
                 '--out-dir pkg --target web && ' ..
                 'node test.js')
end, {})
