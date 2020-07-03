# Shell reference

_My reference sheet for system work with Unix-like shells._

## View documentation

```sh
man some_thing
```

## Interrupt a running process

```sh
<ctrl c>
```

## Run an executable

```sh
./some_path<enter>
```

## Print working directory

```sh
pwd
```

## Display own currently running processes

```sh
ps
```

## Display all currently running processes

```sh
ps -e
```

## Stop a process and allow it to finish (SIGTERM)

```sh
kill -15 some_pid
```

## Force a process to quit with potential data loss or corruption (SIGKILL)

```sh
kill -9 some_pid
```

## Quit all spawned processes with a specific name

```sh
killall some_process_name
```

## List directory contents

```sh
ls
```

## List all directory contents

```sh
ls -a
```

## Change directory

```sh
cd some_path
```

## Change to last directory

```sh
cd -
```

## Create directory

```sh
mkdir some_directory
```

## Create file

```sh
touch some_file
```

## Remove directory

```sh
rm -rf some_directory
```

## Remove file

```sh
rm some_file
```

## Move directory

```sh
mv some_directory another_directory
```

## Move file

```sh
mv some_directory another_directory
```

## Copy directory

```sh
cp -R some_directory another_directory
```

## Copy file

```sh
cp some_file another_file
```

## Re-run commands

```sh
<ctrl r>
Some search
# <up/down> to move through list of items
# <enter> to run
```

## See where an executable is installed

```sh
which some_executable
```

## Pipe output from one command as input to another

```sh
some_command | another_command
```

## Redirect output from one command to a file

```sh
some_command > some_file
```

## Use file contents in a command

```sh
some_file < some_command
```

## Run a command if another is successful

```sh
some_command && another_command
```

## File contents

```sh
cat some_file
```

## Page contents

```sh
less some_file
```

## Search for a pattern

```sh
grep some_pattern
```

Examples:

```sh
ps -ef | grep systemd
```

## Search for a pattern while ignoring the casing

```sh
grep -i some_pattern
```

## Search for a pattern in a path

```sh
grep -R some_pattern some_path
```

## Substitute a pattern in a path

```sh
sed 's/some_pattern/some_replacement/g' some_path
```

## Send HTTP requests

```sh
curl some_url
```

- Retrieve data with a `GET` request method
- Submit data with a `POST` request method
- Replace data with a `PUT` request method
- Remove data with a `DELETE` request method

HTTP response status codes:

- Informational responses (100–199)
- Successful responses (200–299)
- Redirects (300–399)
- Client errors (400–499)
- Server errors (500–599)

Common status codes include:

- `200` is a catchall for success
- `404` means the requested resource is not available
- `403` means the requester does not have permission to access the requested resource
- `500` is a catchall for server-side errors

Headers with `Name: value` like:

```http
Cache-Control: max-age=600
Authorization: Basic YWxhZGRpbjpvcGVuc2VzYW1l
Age: 24
```

## Run executable files

```sh
./some_executable
```

## Create a shell script

```sh
touch some_executable
chmod +x some_executable
# '#!/bin/sh' at top of file and shell commands inside
```

## Work on a remote server

```sh
ssh some_remote_server
# Run other unix commands
exit
```

## Run a command on a remote server

```sh
ssh some_remote_server "some_command"
```

## Show who is logged in

```sh
who
```

## Use text

```sh
echo "Some text"
```

Examples:

```sh
echo "Some text to append to file" >> some_file
```

## Check environment variables

```sh
echo $SOME_VARIABLE
```

## Display processes using the most resources

```sh
top
```

## Use package management

On Linux:

```sh
# Search for a package
snap find "some search"

# View details about a package
snap info some_package

# Install packages
sudo snap install some_package

# List installed packages
snap list

# Uninstall packages
sudo snap remove some_package
```

On macOS:

````sh
# Search for a package
brew search some_search

# View details about a package
brew info some_package

# Install packages
brew install some_package

# List installed packages
brew leaves

# Uninstall packages
brew uninstall some_package

# Install apps
brew cask install some_app

# List installed apps
brew cask list

# Uninstall apps
brew uninstall some_app

# Upgrade all package management
brew update && brew upgrade ```
````

## Use the clipboard

On macOS:

```sh
# Copy to the clipboard
pbcopy

# Paste from the clipboard
pbpaste
```

Examples:

```sh
echo "Some thing" | pbcopy
pbpaste > ~/Downloads/somefile.txt
```
