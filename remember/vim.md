# Vim

My reference sheet for Vim.

## View documentation

```shell
:h some_thing
```

## Start

```shell
cd some_project_directory_root
vim
```

## Write

```shell
:w
```

## Write and quit

```shell
:wq
```

## Switch to a shell

```shell
:sh
# Run shell commands
<ctrl d>
```

## Explore directories

```shell
:e some_directory
```

Examples:

```shell
:e src/
:e .
```

## Open files

```shell
:e some_file
```

Examples:

```shell
:e **/*some_file<tab>
:e src/**/*some_file<tab>
:e **/*<tab>
```

## Search project for a pattern

```shell
:vim /some_pattern/ some_path
:cn/p # to jump between quickfix results
:copen # to show all quickfix results
```

Examples:

```shell
:vim /test/ **/*
:vim /fn/ **/*.rs
:vim /#/ **/.*
```

## Page

```shell
<ctrl f/b>
```

## Repeat

```shell
.
```

## Undo

```shell
u
```

## Redo

```shell
<ctrl r>
```

## Split windows

```shell
:vs
```

## Go to window

```shell
<ctrl w h/j/k/l>
```

## Go in/out (forward/back)

```shell
<ctrl i/o>
```

## Go to file

```shell
# Cursor over import
gf
```

## Complete word

```shell
<ctrl n>
<ctrl n/p> # to move through list items
```

## Complete line

```shell
<ctrl x><ctrl l>
<ctrl n/p> # to move through list items
```

## Substitute a pattern in a range

```shell
:some_range s/some_pattern/some_replacement/g
```

Examples:

```shell
:%s/foo/bar/gc
```

## Apply commands to each quickfix item

```shell
:cdo some_command
```

Examples:

```shell
# Populate the quickfix with something like :vim[grep]
:cdo normal d/foo
```

## Apply commands to each quickfix file

```shell
:cfdo some_command
```

Examples:

```shell
# Populate the quickfix with something like :vim[grep]
:cfdo %s/foo/bar/g
```

## Apply generic commands across lines in a file (macro)

```shell
qq
# Complete generic commands for a line
q
:some_range normal @q
```

## Apply generic commands across files (macro)

```shell
qq
# Complete generic commands for a file
q
# Populate the quickfix with something like :vim[grep]
:c[f]do normal @q | :w
```

## Apply ex commands to patterns in a file

```shell
:some_range g/some_pattern/some_command
```

Examples:

```shell
:g/delete_me/d
```

## Compose commands

Operators, motions, text objects etc. can be composed like a sentence.

Examples:

```shell
# search for 'foo'
/foo

# delete up to 'foo'
d/foo

# delete word
dw

# change to end of line
c$

# change inside braces
ci{

# yank around brackets
ya[

# clipboard register put
"+p

# format to end of file
=G

# etc.
```

## Search file for a pattern

```shell
/some_pattern
n # to move to the next
```

## Change

```shell
c
```

## Delete

```shell
d
```

## Yank (copy)

```shell
y
```

## Put (paste)

```shell
p
```

## Inside

```shell
i
```

## Around

```shell
a
```

## Parenthesis

```shell
(
```

## Braces

```shell
{
```

## Brackets

```shell
[
```

## Double quotes

```shell
"
```

## Single quotes

```shell
'
```

## Backtick

```shell
`
```

## Tag

```shell
t
```

## Word

```shell
w
```

## Start of line

```shell
^
```

## End of line

```shell
$
```

## File top

```shell
gg
```

## File bottom

```shell
G
```

## Format

```shell
=
```

## Clipboard register

```shell
"+
```
