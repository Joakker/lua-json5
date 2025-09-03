# Json5 parser for luajit

This crate provides json5 deserialization for luajit.

Inspired and adapted from [json5-rs](https://github.com/callum-oakley/json5-rs)

Also, if you haven't already, add ';?.dylib' to your `package.cpath` so it will
be recognized by the interpreter.

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
    -- if you're on windows
    -- run = 'powershell ./install.ps1'
    run = './install.sh'
}
```

Using [lazy.nvim](https://github.com/folke/lazy.nvim.git)

```lua
{ 
    'Joakker/lua-json5',
    build = './install.sh',
}
```

## Lua 5.4

You can also build the library for lua 5.4 using the following command:

```sh
cargo build --no-default-features --features lua54 --release
```

## Performance

Tested on neovim using the following script:

```lua
local data = [[ {"hello":"world"} ]]
local json5 = require('json5').parse
local json_decode = vim.fn.json_decode

local time_json5, time_json_decode = 0, 0

local aux

for _ = 1, 1000 do
    aux = os.clock()
    json5(data)
    time_json5 = time_json5 + (os.clock() - aux)
end

for _ = 1, 1000 do
    aux = os.clock()
    json_decode(data)
    time_json_decode = time_json_decode + (os.clock() - aux)
end

print(('json5:        %.3fms'):format(time_json5))
print(('json_decode:  %.3fms'):format(time_json_decode))
```

On average:
```
json5:        0.023ms
json_decode:  0.010ms
```

## So, why should I use this instead of the builtin `json_decode`?

If performance is your concern, I think you're better off using the builtin
function `json_decode`. The advantage this package has over regular json,
however, is that you get json5 features, such as comments, trailing commas and
more flexible string literals.
