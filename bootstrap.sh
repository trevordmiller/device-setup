#!/bin/sh


# ==================================================================
# COMMAND LINE PACKAGES (WITH HOMEBREW)
# ==================================================================

# Install Homebrew, then use it to install OSX packages
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
brew install git
brew install vim
brew install node
brew install haskell-stack


# ==================================================================
# GRAPHICAL APPS (WITH HOMEBREW CASK)
# ==================================================================

# Install Homebrew Cask, then use it to install OSX graphical apps
brew tap caskroom/cask
brew cask install anki
brew cask install dash
brew cask install google-chrome
brew cask install google-photos-backup
brew cask install iterm2
brew cask install karabiner
brew cask install screenflow
brew cask install seil
brew cask install sketch
brew cask install skitch
brew cask install slack
brew cask install slate
brew cask install spotify


# ==================================================================
# DOTFILES
# ==================================================================

# Create dotfile symlinks in home directory
dotfilesDirectoryPath=~/projects/settings/dotfiles
files=".bash_profile .git-prompt.sh .git-completion.sh .gitconfig .vimrc .npmrc .slate"
cd $dotfilesDirectoryPath
for file in $files; do
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


# ==================================================================
# END
# ==================================================================

echo "Restart shell to see changes"
