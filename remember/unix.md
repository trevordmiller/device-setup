# Unix

My most used terminal commands for Unix-like operating systems.

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
killall some_process_name
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
mv some_directory another_directory
```

## Move file

```shell
mv some_directory another_directory
```

## Copy directory

```shell
cp -R some_directory another_directory
```

## Copy file

```shell
cp some_file another_file
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
some_command | another_command
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
some_command && another_command
```

## File contents

```shell
cat some_file
```

## Page contents

```shell
less some_file
```

## Search for a pattern

```shell
grep some_pattern
```

Examples:

```shell
ps -ef | grep systemd
```

## Search for a pattern while ignoring the casing

```shell
grep -i some_pattern
```

## Search for a pattern in a path

```shell
grep -R some_pattern some_path
```

## Substitute a pattern

```shell
sed 's/some_pattern/some_replacement/g' some_file
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
ssh some_remote_server
# Run other unix commands
exit
```

## Run a command on a remote server

```shell
ssh some_remote_server "some_command"
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
cd some_project_directory_root
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

### Explore directories

```shell
:e some_directory
```

Examples:

```shell
:e src/
:e .
```

### Edit files

```shell
:e some_file
```

Examples:

```shell
:e **/*some_file<tab>
:e src/**/*some_file<tab>
:e **/*<tab>
```

### Search project for a pattern

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

### Page

```shell
<ctrl f/b>
```

### Repeat

```shell
.
```

### Undo

```shell
u
```

### Redo

```shell
<ctrl r>
```

### Split windows

```shell
:vs
```

### Go to window

```shell
<ctrl w h/j/k/l>
```

### Go in/out (forward/back)

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

### Substitute a pattern

```shell
:some_range s/some_pattern/some_replacement/g
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
:cdo normal d/foo
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
:some_range g/some_pattern/some_command
```

Examples:

```shell
:g/delete_me/d
```

### Compose commands

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

#### Search file for a pattern

```shell
/some_pattern
n # to move to the next
```

#### Change

```shell
c
```

#### Delete

```shell
d
```

#### Yank (copy)

```shell
y
```

#### Put (paste)

```shell
p
```

#### Inside

```shell
i
```

#### Around

```shell
a
```

#### Parenthesis

```shell
(
```

#### Braces

```shell
{
```

#### Brackets

```shell
[
```

#### Double quotes

```shell
"
```

#### Single quotes

```shell
'
```

#### Backtick

```shell
`
```

#### Tag

```shell
t
```

#### Word

```shell
w
```

#### Start of line

```shell
^
```

#### End of line

```shell
$
```

#### File top

```shell
gg
```

#### File bottom

```shell
G
```

#### Format

```shell
=
```

#### Clipboard register

```shell
"+
```
