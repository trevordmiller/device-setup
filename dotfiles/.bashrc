# ==================================================================
# PATH
# ==================================================================

SCRIPTS_MAIN_PATH="$HOME/drive/settings/scripts/main"
SCRIPTS_MAIN_PROJECTS_PATH="$HOME/drive/settings/scripts/main/projects"
SCRIPTS_UTILS_PATH="$HOME/drive/settings/scripts/utils"
HOMEBREW_PATH="/usr/local/bin"
PROJECT_NPM_PATH="./node_modules/.bin"
YARN_PATH="$HOME/.yarn/bin"
export PATH="$PATH:$SCRIPTS_PATH:$SCRIPTS_MAIN_PATH:$SCRIPTS_MAIN_PROJECTS_PATH:$SCRIPTS_UTILS_PATH:$HOMEBREW_PATH:$PROJECT_NPM_PATH:$YARN_PATH"


# ==================================================================
# PROMPT
# ==================================================================

# SYMBOLS
WORKING_DIRECTORY="\W"
PROMPT_SYMBOL="\$"

# FORMATTING
NEWLINE="\n"

# COLOR
COLOR_USER_CURRENT_STATE='\[\033[01;36m\]'
COLOR_IDENTIFIER='\[\033[01;34m\]'
COLOR_STATEMENT='\[\033[01;33m\]'
COLOR_RESET='\[\033[0m\]'

# GIT
source ~/.git-completion.sh
source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1
GIT_BRANCH='$(__git_ps1 "[%s]")'

# RENDER
export PS1="$NEWLINE${COLOR_IDENTIFIER}$WORKING_DIRECTORY${COLOR_USER_CURRENT_STATE}$GIT_BRANCH$NEWLINE${COLOR_STATEMENT}$PROMPT_SYMBOL${COLOR_RESET} "

# VI BINDINGS
set -o vi


# ==================================================================
# EDITOR
# ==================================================================

export VISUAL=vim
export EDITOR="$VISUAL"


# ==================================================================
# ALIASES
# ==================================================================

alias ls='ls -G'
alias grep='grep --color=auto'
alias less='less -R'
alias ag='ag --hidden --path-to-ignore ~/.agignore'
