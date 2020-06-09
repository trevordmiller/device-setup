# My most used Unix commands

## View documentation

- `man {verb}`

## Interrupt a running process

- `<ctrl c>`

## Run a binary executable

- `{path}<enter>`

## Print working directory

- `pwd`

## Display currently running processes

- `ps`

## List directory

- `ls -a`

## Change directory

- `cd {path}`

## Change to last directory

- `cd -`

## Create directory

- `mkdir {directory}`

## Create file

- `touch {file}`

## Remove directory 

- `rm -rf {directory}`

## Remove file

- `rm {file}`

## Move directory

- `mv {source} {destination}`

## Move file

- `mv {source} {destination}`

## Re-run commands

- `<ctrl r>`
- Search
- `<up/down>` to move through list of items
- `<enter>` to run

## See where a package is installed

- `which {package}`

## Pipe output from one command as input to another

- `{command} | {command}`

## Redirect output from one command to a file

- `{command} > {file}`

## Use file contents in a command

- `{file} < {command}`

## Run a command if another is successful

- `{command} && {command}`

## File contents

- `cat {file}`

## Search

- `grep -r {regex} {path}`

## Substitute

- `sed`

## HTTP requests

- `curl`

## Manage packages (via Homebrew on macOS, Linux, or Windows Subsystem for Linux)

### Search packages

- `brew search {package}`

### View info about packages

- `brew info {package}`

### Install packages

- `brew install {package}`

### Install apps

- `brew cask install {app}`

### List installed packages

- `brew leaves`

### Uninstall packages

- `brew uninstall {package}`

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

## See if in a subshell

- `ps`, if `vim` is listed

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
