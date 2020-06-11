# Standard Vim: how to use Vim without customizations

The purpose of this article is to show useful things that can be done in standard Vim without customizations like plugins, configurations, forks, or GUIs. However, this is not meant to shame using customizations! Customizing can be useful. But I do see Vim users that add customizations based on misunderstandings or popularity rather than to solve an actual problem. I would suggest instead that we try to understand the vanilla options first. Then add customizations for specific cases when we know what we want and what tradeoffs we are making. The advantage of using native Vim functionality is that it is portable; it works on most Unix-based machines by default without any extra setup. Although to be fair, a few of these items do rely on newer versions of Vim than you might find on some machines. I'd recommend using a package manager to install the latest version of Vim.

## View help

```shell
:h some-thing
```

## Start

```shell
cd some-project-directory-root
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

## Search

```shell
:vim /some-regex/ some-path
:cn/p # to jump between quickfix results
:copen # to show all quickfix results
```

Examples:

```shell
:vim /test/ **/*
:vim /fn/ **/*.rs
:vim /#/ **/.*
```

## Edit

```shell
:e some-path
```

Examples:

```shell
:e **/*some-file<tab>
:e src/**/*some-file<tab>
:e **/*<tab>
:e src/
:e .
```

## Page

```shell
<ctrl f/b>
```

## Search

```shell
/some-regex
n # to move to the next
```

## Block

```shell
{ / }
```

## Top

```shell
gg
```

## Bottom

```shell
G
```

## Format

```shell
=
```

## Change

```shell
c
```

## Delete

```shell
d
```

## Yank

```shell
y
```

## Put

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

## Braces

```shell
{
```

## Parenthesis

```shell
(
```

## Tag

```shell
t
```

## Undo

```shell
u
```

## Redo

```shell
<ctrl r>
```

## Put from the clipboard

```shell
"+p
```

## Delete to the clipboard

```shell
"+d
```

## Yank to the clipboard

```shell
"+y
```

## Split windows

```shell
:vs
```

## Go to window

```shell
<ctrl w h/j/k/l>
```

## Go in/out

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

## Substitute

```shell
:some-range s/some-regex/some-replacement/g
```

Examples:

```shell
:%s/foo/bar/gc
```

## Apply commands to each quickfix item

```shell
:cdo {command}
```

Examples:

```shell
# Populate the quickfix with something like :vim[grep]
:cdo normal dd
```

## Apply commands to each quickfix file

```shell
:cfdo {command}
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
:some-range normal @q
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
:some-range g/some-regex/some-command
```

Examples:

```shell
:g/deleteMe/d
```
