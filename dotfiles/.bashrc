# ==================================================================
# PATH
# ==================================================================

SCRIPTS_PATH="$HOME/drive/settings/scripts"
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

# COLOR
COLOR_PURPOSE_USER_CURRENT_STATE=$"\e[36m"
COLOR_PURPOSE_IDENTIFIER=$"\e[34m"
COLOR_RESET=$"\e[0m"

# GIT
source ~/.git-completion.sh
source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1
GIT_BRANCH='$(__git_ps1 "[%s]")'

# RENDER
export PS1="${COLOR_PURPOSE_IDENTIFIER}$WORKING_DIRECTORY${COLOR_RESET}${COLOR_PURPOSE_USER_CURRENT_STATE}$GIT_BRANCH${COLOR_RESET}${COLOR_PURPOSE_IDENTIFIER}$PROMPT_SYMBOL${COLOR_RESET} "

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

alias ls="ls -G"
alias grep="grep --color=auto"
alias less="less -R"
