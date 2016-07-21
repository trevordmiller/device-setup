#!/bin/bash

# Bash strict mode
set -euo pipefail

function osx_system_settings {

  # Show hidden files
  defaults write com.apple.finder AppleShowAllFiles YES
  killall Finder

  # Set screenshot output folder
  defaults write com.apple.screencapture location ~/Downloads
  killall SystemUIServer
}


function command_line_packages {

  # Install Homebrew
  /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

  # Use Homebrew to install command line packages
  for package in $1; do
    brew install $package
  done

  # Package specific setup
  n stable
  npm install -g eslint_d
  cd ~/
  stack setup
  stack install hdevtools
}

function dotfiles {

  # Create dotfile symlinks in home directory
  for file in $1; do
    ln -s $2/$file ~/$file
  done
}

function vim_plugins {

  # Install Vundle
  git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim

  # Use Vundle to install Vim plugins
  vim +PluginInstall +qall
}

function graphical_apps {

  # Open Brew tap for Cask
  brew tap caskroom/cask

  # Use Cask to install graphical apps
  for app in $1; do
    brew cask install $app
  done
}

function bootstrap {
  osx_system_settings
  command_line_packages "bash git n node vim youtube-dl"
  dotfiles ".bash_profile .vimrc .git-prompt.sh .git-completion.sh .gitignore .gitconfig .npmrc .ghci .eslintrc .slate" ~/Google Drive/settings/dotfiles 
  vim_plugins
  graphical_apps "1password anki dash google-chrome google-drive google-photos-backup iterm2 karabiner screenflow seil sketch skitch slack slate spotify flux rescuetime iexplorer"
}

# Run
bootstrap
