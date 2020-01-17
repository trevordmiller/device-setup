# Most used commands in Vim

## View documentation

- `:h {verb}`

## Start

- `cd {project root}`
- `vim`

## Switch to a shell

- `:sh`
- Run shell commands
- `<ctrl d>`

## Search

- `:vim /{regex}/ {path}`
- `:cn/p` to jump between quickfix results
- `:copen` to show all quickfix results

Examples:

 ```vim
:vim /test/ **/*
:vim /fn/ **/*.rs
:vim /#/ **/.*
```

## Edit

- `:e {path}`

Examples:

```vim
:e **/*{file}<tab>
:e src/**/*{file}<tab>
:e **/*<tab>
:e src/
:e .
```

## Write

- `:w`

## Write and quit

- `:wq`

## Page

- `<ctrl f/b>`

## Search

- `/{regex}`
- `n` to move to the next

## Block

- `{` / `}`

## Top

- `gg`

## Bottom

- `G`

## Format

- `=`

## Change

- `c`

## Delete

- `d`

## Yank

- `y`

## Put

- `p`

## Inside

- `i`

## Around

- `a`

## Braces

- `{`

## Parenthesis

- `(`

## Tag

- `t`

## Undo

- `u`

## Redo

- `<ctrl r>`

## Put from the clipboard

- `"+p`

## Delete to the clipboard

- `"+d`

## Yank to the clipboard

- `"+y`

## Substitute

- `:{range}s/{regex}/{substitution}/g`

Examples:

```vim
:%s/foo/bar/gc
```

## Apply ex commands to patterns

- `:{range}g/{regex}/{command}`

Examples:

```vim
:g/deleteMe/d
```

## Macro

- `qq`
- Complete generic commands
- `q`
- Populate the quickfix with something like `:vim[grep]`
- `:c[f]do normal @q | :w`

## Split windows

- `:vs`

## Go to window

- `<ctrl w h/j/k/l>`

## Go in/out

- `<ctrl i/o>`

## Go to file

- Cursor over import
- `gf`

## Complete word

- `<ctrl n>`
- `<ctrl n/p>` to move through list items

## Complete line

- `<ctrl x><ctrl l>`
- `<ctrl n/p>` to move through list items

