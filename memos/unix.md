# Unix

## View help

```shell
man some_thing
```

## Interrupt a running process

```shell
<ctrl c>
```

## Run an executable

```shell
./some_path<enter>
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
kill -15 some_pid
```

## Force a process to quit with potential data loss or corruption (SIGKILL)

```shell
kill -9 some_pid
```

## Quit all spawned processes with a specific name

```shell
killall some_process-name
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
cd some_path
```

## Change to last directory

```shell
cd -
```

## Create directory

```shell
mkdir some_directory
```

## Create file

```shell
touch some_file
```

## Remove directory

```shell
rm -rf some_directory
```

## Remove file

```shell
rm some_file
```

## Move directory

```shell
mv some_directory another-directory
```

## Move file

```shell
mv some_directory another-directory
```

## Copy directory

```shell
cp -R some_directory another-directory
```

## Copy file

```shell
cp some_file another-file
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
which some_executable
```

## Pipe output from one command as input to another

```shell
some_command | another-command
```

## Redirect output from one command to a file

```shell
some_command > some_file
```

## Use file contents in a command

```shell
some_file < some_command
```

## Run a command if another is successful

```shell
some_command && another-command
```

## File contents

```shell
cat some_file
```

## Page contents

```shell
less some_file
```

## Search

```shell
grep some_regex
```

Examples:

```shell
ps -ef | grep systemd
```

## Search ignoring the casing

```shell
grep -i some_regex
```

## Search a path

```shell
grep -R some_regex some_path
```

## Substitute

```shell
sed 's/some_regex/some_replacement/g' some_file
```

## Send HTTP requests

```shell
curl some_url
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
ssh some_remote-server
# Run other unix commands
exit
```

## Run a command on a remote server

```shell
ssh some_remote-server "some_command"
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
echo "Some text to append to file" >> some_file
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
:h some_thing
```

### Start

```shell
cd some_project-directory-root
vim
```

### Write

```shell
:w
```

### Write and quit

```shell
:wq
```

### Switch to a shell

```shell
:sh
# Run shell commands
<ctrl d>
```

### Search

```shell
:vim /some_regex/ some_path
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
:e some_path
```

Examples:

```shell
:e **/*some_file<tab>
:e src/**/*some_file<tab>
:e **/*<tab>
:e src/
:e .
```

### Page

```shell
<ctrl f/b>
```

### Search

```shell
/some_regex
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
:some_range s/some_regex/some_replacement/g
```

Examples:

```shell
:%s/foo/bar/gc
```

### Apply commands to each quickfix item

```shell
:cdo some_command
```

Examples:

```shell
# Populate the quickfix with something like :vim[grep]
:cdo normal dd
```

### Apply commands to each quickfix file

```shell
:cfdo some_command
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
:some_range normal @q
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
:some_range g/some_regex/some_command
```

Examples:

```shell
:g/delete_me/d
```
