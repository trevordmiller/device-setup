#!/bin/sh


# ==================================================================
# COMMAND LINE PACKAGES 
# ==================================================================

# HOMEBREW
# Install Homebrew, then use it to install packages
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
packages="git vim node ghc haskell-stack"
for package in $packages; do
  brew install $package
done

# NPM
npm install -g eslint_d

# STACK
cd ~/
stack setup
stack install hdevtools


# ==================================================================
# GRAPHICAL APPS
# ==================================================================

# CASK
# Open Brew tap for Cask, then use Cask to install graphical apps
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
dotfiles=".bash_profile .vimrc .git-prompt.sh .git-completion.sh .gitconfig .npmrc .ghci .slate"
for file in $dotfiles; do
  ln -s $dotfilesDirectoryPath/$file ~/$file
done


# ==================================================================
# VIM PLUGINS
# ==================================================================

# Install Vundle, then use it to install Vim plugins
git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim
vim +PluginInstall +qall


# ==================================================================
# SYSTEM SETTINGS
# ==================================================================

# Show hidden files
defaults write com.apple.finder AppleShowAllFiles YES

# Set screenshot output folder
defaults write com.apple.screencapture location ~/Downloads/
