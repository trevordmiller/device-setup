dotfilesDirectoryPath=~/projects/settings/dotfiles
files=".bash_profile .bashrc .gitconfig .npmrc .slate .vimrc"

echo "Changing to the $dotfilesDirectoryPath directory"
cd $dotfilesDirectoryPath
echo "...done"

echo "Creating symlink to $file in home directory"
for file in $files; do
  ln -s $dotfilesDirectoryPath/$file ~/$file
done
echo "...done"

echo "Sourcing .bashrc and .vimrc"
source ~/.bashrc
source ~/.vimrc
echo "...done"
