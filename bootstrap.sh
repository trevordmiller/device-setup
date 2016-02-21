#!/bin/sh

cd ~/


# ==================================================================
# COMMAND LINE PACKAGES (WITH HOMEBREW)
# ==================================================================

# Install Homebrew, then use it to install OSX packages
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
packages="git vim node haskell-stack"
for package in $packages; do
  brew install $package
done


# ==================================================================
# GRAPHICAL APPS (WITH HOMEBREW CASK)
# ==================================================================

# Install Homebrew Cask, then use it to install OSX graphical apps
brew tap caskroom/cask
apps="anki dash google-chrome google-photos-backup iterm2 karabiner screenflow seil sketch skitch slack slate spotify"
for app in $apps; do
  brew cask install $app
done


# ==================================================================
# DOTFILES
# ==================================================================

# Create dotfile symlinks in home directory
dotfilesDirectoryPath=~/projects/settings/dotfiles
dotfiles=".bash_profile .git-prompt.sh .git-completion.sh .gitconfig .vimrc .npmrc .slate"
for file in $dotfiles; do
  ln -s $dotfilesDirectoryPath/$file ~/$file
done


# ==================================================================
# VIM PLUGINS (WITH VUNDLE)
# ==================================================================

# Install Vundle, then use it to install Vim plugins
git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim
vim +PluginInstall +qall


# ==================================================================
# OSX SYSTEM SETTINGS
# ==================================================================

# Show hidden files
defaults write com.apple.finder AppleShowAllFiles YES

# Set screenshot output folder
defaults write com.apple.screencapture location ~/Downloads/
