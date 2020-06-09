# My most used Unix commands

## View help

```shell
man some-thing
```

## Interrupt a running process

```shell
<ctrl c>
```

## Run an executable

```shell
./some-path<enter>
```

## Print working directory

```shell
pwd
```

## Display own currently running processes

```shell
ps
```

## Display all currently running processes

```shell
ps -e
```

## Display processes using the most resources

```shell
top
```

## Stop a process and allow it to finish (SIGTERM)

```shell
kill -15 some-pid
```

## Force a process to quit with potential data loss or corruption (SIGKILL)

```shell
kill -9 some-pid
```

## Quit all spawned processes with a specific name

```shell
killall some-process-name
```

## List directory contents

```shell
ls
```

## List all directory contents

```shell
ls -a
```

## Change directory

```shell
cd some-path
```

## Change to last directory

```shell
cd -
```

## Create directory

```shell
mkdir some-directory
```

## Create file

```shell
touch some-file
```

## Remove directory

```shell
rm -rf some-directory
```

## Remove file

```shell
rm some-file
```

## Move directory

```shell
mv some-directory another-directory
```

## Move file

```shell
mv some-directory another-directory
```

## Copy directory

```shell
cp -R some-directory another-directory
```

## Copy file

```shell
cp some-file another-file
```

## Re-run commands

```shell
<ctrl r>
Some search
# <up/down> to move through list of items
# <enter> to run
```

## See where an executable is installed

```shell
which some-executable
```

## Pipe output from one command as input to another

```shell
some-command | another-command
```

## Redirect output from one command to a file

```shell
some-command > some-file
```

## Use file contents in a command

```shell
some-file < some-command
```

## Run a command if another is successful

```shell
some-command && another-command
```

## File contents

```shell
cat some-file
```

## Page contents

```shell
less some-file
```

## Search

```shell
grep some-regex
```

Examples:

```shell
ps -ef | grep systemd
```

## Search ignoring the casing

```shell
grep -i some-regex
```

## Search a path

```shell
grep -R some-regex some-path
```

## Substitute

```shell
sed 's/some-regex/some-replacement/g' some-file
```

## Send HTTP requests

```shell
curl some-url
```

## Run executable files

```shell
./some_executable
```

## Create a shell script

```shell
touch some_executable
chmod +x some_executable
# '#!/bin/sh' at top of file and shell commands inside
```

## Work on a remote server

```shell
ssh some-remote-server
# Run other unix commands
exit
```

## Run a command on a remote server

```shell
ssh some-remote-server "some-command"
```

## Show who is logged in

```shell
who
```

## Use text

```shell
echo "Some text"
```

Examples:

```shell
echo "Some text to append to file" >> some-file
```

## Check environment variables

```shell
echo $SOME_VARIABLE
```

## Edit files

```shell
vim
```

### View help

```shell
:h some-thing
```

### Start

```shell
cd some-project-directory-root
vim
```

### Switch to a shell

```shell
:sh
# Run shell commands
<ctrl d>
```

### Search

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

### Edit

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

### Write

```shell
:w
```

### Write and quit

```shell
:wq
```

### Page

```shell
<ctrl f/b>
```

### Search

```shell
/some-regex
n # to move to the next
```

### Block

```shell
{ / }
```

### Top

```shell
gg
```

### Bottom

```shell
G
```

### Format

```shell
=
```

### Change

```shell
c
```

### Delete

```shell
d
```

### Yank

```shell
y
```

### Put

```shell
p
```

### Inside

```shell
i
```

### Around

```shell
a
```

### Braces

```shell
{
```

### Parenthesis

```shell
(
```

### Tag

```shell
t
```

### Undo

```shell
u
```

### Redo

```shell
<ctrl r>
```

### Put from the clipboard

```shell
"+p
```

### Delete to the clipboard

```shell
"+d
```

### Yank to the clipboard

```shell
"+y
```

### Split windows

```shell
:vs
```

### Go to window

```shell
<ctrl w h/j/k/l>
```

### Go in/out

```shell
<ctrl i/o>
```

### Go to file

```shell
# Cursor over import
gf
```

### Complete word

```shell
<ctrl n>
<ctrl n/p> # to move through list items
```

### Complete line

```shell
<ctrl x><ctrl l>
<ctrl n/p> # to move through list items
```

### Substitute

```shell
:some-range s/some-regex/some-replacement/g
```

Examples:

```shell
:%s/foo/bar/gc
```

### Apply commands to each quickfix item

```shell
:cdo {command}
```

Examples:

```shell
# Populate the quickfix with something like :vim[grep]
:cdo normal dd
```

### Apply commands to each quickfix file

```shell
:cfdo {command}
```

Examples:

```shell
# Populate the quickfix with something like :vim[grep]
:cfdo %s/foo/bar/g
```

### Apply generic commands across lines in a file (macro)

```shell
qq
# Complete generic commands for a line
q
:some-range normal @q
```

### Apply generic commands across files (macro)

```shell
qq
# Complete generic commands for a file
q
# Populate the quickfix with something like :vim[grep]
:c[f]do normal @q | :w
```

### Apply ex commands to patterns in a file

```shell
:some-range g/some-regex/some-command
```

Examples:

```shell
:g/deleteMe/d
```
