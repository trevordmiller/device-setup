# Unix

My reference sheet for Unix.

## View documentation

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

## Substitute a pattern in a path

```shell
sed 's/some_pattern/some_replacement/g' some_path
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
