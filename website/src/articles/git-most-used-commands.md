# Most used commands in Git

## View documentation

- `git --help`

## Start working in an existing repo

- `git clone {existing}`
- `cd {existing}`
- `git config user.name "Trevor D. Miller"`
- `git config user.email "5497885+trevordmiller@users.noreply.github.com"`
- Add any missing `.gitignore` items specific to my setup like `.DS_Store` and `*.swp`

## Search across tracked files

- `git grep {regex}`

## Status

- `git status`

## Diff with staged

- `git diff`

## Restore working directory version

- `git checkout {paths}`

## Stage

- `git stage {paths}`

## Stage hunks

- `git add -p`
- Respond to each hunk with `y` (yes), `n` (no), or `s` (split) to break down into smaller hunks

## Commit what is staged

- `git commit`
- Opens default editor
- Fill out commit message
- Save and quit file to let commit finish

## Push

- `git push`

## Rebase

- `git fetch && git rebase origin/master`
- Fix conflicts
- `git push -f` to overwrite history

## Blame

- `git blame {files}`

## Search

- `git log -p -S {string}`

## Find the commit that broke something

- `git bisect start`
- `git bisect bad`
- `git bisect good {reference}` with reference to when things were known to work
- `git bisect {bad/good}` repeated until binary search is complete
- `git bisect reset` to clean up
