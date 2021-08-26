# Json5 parser for luajit

This crate provides json5 deserialization for luajit.

Inspired and adapted from [json5-rs](https://github.com/callum-oakley/json5-rs)

## Usage

You can simply require the module in your scripts and parse a string using the
`parse` method:

```lua
local parse = require'json5'.parse
local data = [[
{
    /* This is a comment */
    ecma_identifier: 'works like a charm',
    "string keys": [1,2,3], // trailing comma
}
]]
local parsed_data = parse(data)
```

## Use with neovim

You must have `cargo` installed and in your `$PATH`

Using [packer.nvim](https://github.com/wbthomason/packer.nvim):

```lua
use {
    'Joakker/lua-json5',
    run = './install.sh'
}
```
