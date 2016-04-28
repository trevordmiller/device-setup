#!/bin/bash

function osx_system_settings {
  set -euo pipefail

  # Show hidden files
  defaults write com.apple.finder AppleShowAllFiles YES

  # Set screenshot output folder
  defaults write com.apple.screencapture location ~/Downloads/
}


function command_line_packages {
  set -euo pipefail

  # Install Homebrew
  /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

  # Use Homebrew to install command line packages
  for package in $1; do
    brew install $package
  done

  # Package specific setup
  n stable
  cd ~/
  stack setup
  stack install hdevtools
  npm install -g eslint_d
}

function dotfiles {
  set -euo pipefail

  # Create dotfile symlinks in home directory
  for file in $1; do
    ln -s $2/$file ~/$file
  done
}

function vim_plugins {
  set -euo pipefail

  # Install Vundle
  git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim

  # Use Vundle to install Vim plugins
  vim +PluginInstall +qall
}

function graphical_apps {
  set -euo pipefail

  # Open Brew tap for Cask
  brew tap caskroom/cask

  # Use Cask to install graphical apps
  for app in $1; do
    brew cask install $app
  done
}

function bootstrap {
  osx_system_settings
  command_line_packages "git vim node n ghc haskell-stack"
  dotfiles ".bash_profile .vimrc .git-prompt.sh .git-completion.sh .gitconfig .npmrc .ghci .slate" ~/projects/settings/dotfiles 
  vim_plugins
  graphical_apps "anki dash google-chrome google-photos-backup iterm2 karabiner screenflow seil sketch skitch slack slate spotify"
}
