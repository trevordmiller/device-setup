# macOS reference

_My reference sheet for system work specific to macOS._

## View documentation

```sh
man some_thing
```

## Search for a package

```sh
brew search some_search
```

## View details about a package

```sh
brew info some_package
```

## Install packages

```sh
brew install some_package
```

## List installed packages

```sh
brew leaves
```

## Uninstall packages

```sh
brew uninstall some_package
```

## Install apps

```sh
brew cask install some_app
```

## List installed apps

```sh
brew cask list
```

## Uninstall apps

```sh
brew uninstall some_app
```

## Upgrade all package management

```sh
brew update && brew upgrade
```

## Copy to the clipboard

```sh
pbcopy
```

Examples:

```sh
echo "Some thing" | pbcopy
```

## Paste from the clipboard

```sh
pbpaste
```

Examples:

```sh
pbpaste > ~/Downloads/somefile.txt
```
