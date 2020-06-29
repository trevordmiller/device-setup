+++
title = "Vim reference"
weight = 4
+++

_My reference sheet for text editing with Vim._

## View documentation

```sh
:h some_thing
```

## Start

```sh
cd some_project_directory_root
vim
```

## Write

```sh
:w
```

## Write and quit

```sh
:wq
```

## Switch to a shell

```sh
:sh
# Run shell commands
<ctrl d>
```

## Explore directories

```sh
:e some_directory
```

Examples:

```sh
:e src/
:e .
```

## Open files

```sh
:e some_file
```

Examples:

```sh
:e **/*some_file<tab>
:e src/**/*some_file<tab>
:e **/*<tab>
```

## Search project for a pattern

```sh
:vim /some_pattern/ some_path
:cn/p # to jump between quickfix results
:copen # to show all quickfix results
```

Examples:

```sh
:vim /test/ **/*
:vim /fn/ **/*.rs
:vim /#/ **/.*
```

## Page

```sh
<ctrl f/b>
```

## Repeat

```sh
.
```

## Undo

```sh
u
```

## Redo

```sh
<ctrl r>
```

## Split windows

```sh
:vs
```

## Go to window

```sh
<ctrl w h/j/k/l>
```

## Go in/out (forward/back)

```sh
<ctrl i/o>
```

## Go to file

```sh
# Cursor over import
gf
```

## Complete word

```sh
<ctrl n>
<ctrl n/p> # to move through list items
```

## Complete line

```sh
<ctrl x><ctrl l>
<ctrl n/p> # to move through list items
```

## Substitute a pattern in a range

```sh
:some_range s/some_pattern/some_replacement/g
```

Examples:

```sh
:%s/foo/bar/gc
```

## Apply commands to each quickfix item

```sh
:cdo some_command
```

Examples:

```sh
# Populate the quickfix with something like :vim[grep]
:cdo normal d/foo
```

## Apply commands to each quickfix file

```sh
:cfdo some_command
```

Examples:

```sh
# Populate the quickfix with something like :vim[grep]
:cfdo %s/foo/bar/g
```

## Apply generic commands across lines in a file (macro)

```sh
qq
# Complete generic commands for a line
q
:some_range normal @q
```

## Apply generic commands across files (macro)

```sh
qq
# Complete generic commands for a file
q
# Populate the quickfix with something like :vim[grep]
:c[f]do normal @q | :w
```

## Apply ex commands to patterns in a file

```sh
:some_range g/some_pattern/some_command
```

Examples:

```sh
:g/delete_me/d
```

## Compose commands

Operators, motions, text objects etc. can be composed like a sentence.

Examples:

```sh
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

```sh
/some_pattern
n # to move to the next
```

## Change

```sh
c
```

## Delete

```sh
d
```

## Yank (copy)

```sh
y
```

## Put (paste)

```sh
p
```

## Inside

```sh
i
```

## Around

```sh
a
```

## Parenthesis

```sh
(
```

## Braces

```sh
{
```

## Brackets

```sh
[
```

## Double quotes

```sh
"
```

## Single quotes

```sh
'
```

## Backtick

```sh
`
```

## Tag

```sh
t
```

## Word

```sh
w
```

## Start of line

```sh
^
```

## End of line

```sh
$
```

## File top

```sh
gg
```

## File bottom

```sh
G
```

## Format

```sh
=
```

## Clipboard register

```sh
"+
```
