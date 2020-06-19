# Git

My most used terminal commands for Git version control.

## View documentation

```shell
git --help
```

## Setup an existing repository on your local machine

```shell
git clone some_url
cd some_directory
```

## Create a feature branch

```shell
git checkout some_main_branch
git pull
git checkout -b some_branch
```

## Check the local repository state

```shell
git status
```

## Show changes in the working directory

```shell
git diff
```

## Undo changes in the working directory

```shell
git clean some_paths
```

## Save changes in the working directory to the staging area

```shell
# By paths
git add some_paths

# OR

# By smaller pieces inside files
git add -p
# Respond to each hunk with y (yes), n (no), or s (split) to break down into smaller pieces
```

## Show changes in the staging area

```shell
git diff --staged
```

## Undo changes in the staging area (back to the working directory)

```shell
git reset HEAD some_paths
```

## Save changes in the staging area to commits

```shell
git commit
# Opens default editor
# Fill out commit message
# Save and quit file to let commit finish
```

## Show changes in commits

```shell
git fetch
git diff origin/some_main_branch
```

## Undo commits

```shell
# With a revert commit
git log
# Find the reference of the commit to undo
git revert some_reference
```

## Save changes in commits to the remote repository

```shell
git push
```

## Sync a pull request branch with changes in the remote repository

```shell
# With a merge commit
git checkout some_main_branch
git pull
git checkout some_branch
git merge some_main_branch
# Fix conflicts
git push

# OR

# Without a merge commit re-writing history as if the commits in the pull request branch happened on-top
git fetch
git rebase origin/some_main_branch
# Fix conflicts
git push -f
```

## Show the difference between branches

```shell
git diff some_branch..another_branch
```

## Show the difference between tags

```shell
git diff some_tag another_tag
```

## Find which author made changes to a file

```shell
git blame some_file
```

## Search for changes in the repository history

```shell
git log -p -S "some_pattern"
```

## Search for a pattern across all tracked files in the repository

```shell
git grep some_pattern
```

## Find the commit that broke something

```shell
git bisect start
git bisect bad
# Add reference to when things were known to work
git bisect good some_reference
# Repeated bad/good until the binary search is complete
git bisect bad/good
# Clean up
git bisect reset
```
