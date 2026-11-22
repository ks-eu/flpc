-- With 'exrc' disabled, run :source .nvim.lua then :lsp restart rust_analyzer.
vim.lsp.config('rust_analyzer', {
  settings = {
    ['rust-analyzer'] = { cargo = { features = { 'tokio' } } },
  },
})
