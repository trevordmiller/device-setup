dotfilesDirectoryPath=~/projects/settings/dotfiles
files=".bash_profile .git-prompt.sh .git-completion.sh .gitconfig .vimrc .npmrc .slate"

echo "Changing to the $dotfilesDirectoryPath directory"
cd $dotfilesDirectoryPath
echo "...done"

echo "Creating symlink to $file in home directory"
for file in $files; do
  ln -s $dotfilesDirectoryPath/$file ~/$file
done
echo "...done"

echo "Restart shell to see changes"
