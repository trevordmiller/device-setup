# Git

## View help

```shell
git --help
```

## Start working in an existing repo

```shell
git clone some_repo
cd some_repo
```

## Search across tracked files

```shell
git grep some_regex
```

## Status

```shell
git status
```

## Diff

```shell
git diff
```

## Diff staged

```shell
git diff --staged
```

## Diff with remote branch

```shell
git fetch
git diff origin/master
```

## Diff between branches

```shell
git diff some_branch..another-branch
```

## Diff between tags

```shell
git diff some_tag another-tag
```

## Restore working directory version

```shell
git checkout some_paths
```

## Stage

```shell
git stage some_paths
```

## Stage hunks

```shell
git add -p
# Respond to each hunk with y (yes), n (no), or s (split) to break down into smaller hunks
```

## Commit what is staged

```shell
git commit
# Opens default editor
# Fill out commit message
# Save and quit file to let commit finish
```

## Push

```shell
git push
```

## Sync a pull request branch

```shell
# With a merge commit
git checkout master
git pull
git checkout some_branch
git merge master
# Fix conflicts
git push

# Without a merge commit re-writing history as if the commits in the pull request branch happened on-top
git fetch
git rebase origin/master
# Fix conflicts
git push -f
```

## Undo a single commit

```shell
# With a revert commit
git log
# Find the reference of the commit to undo
git revert some_reference
```

## Blame

```shell
git blame some_paths
```

## Search

```shell
git log -p -S "some_string"
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

## Sync cache with an updated .gitignore

```shell
git rm -r --cached .
git add .
git commit
```
