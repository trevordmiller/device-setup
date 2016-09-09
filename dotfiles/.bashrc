# ==================================================================
# PATH
# ==================================================================

SCRIPTS_PATH="$HOME/Google Drive/settings/scripts"
HOMEBREW_PATH="/usr/local/bin"
PROJECT_NPM_PATH="./node_modules/.bin"
export PATH="$PATH:$SCRIPTS_PATH:$HOMEBREW_PATH:$PROJECT_NPM_PATH"


# ==================================================================
# PROMPT
# ==================================================================

# SYMBOLS
WORKING_DIRECTORY="\W"
PROMPT_SYMBOL="\$"

# COLOR
COLOR_YELLOW=$"\e[33m"
COLOR_CYAN=$"\e[36m"
COLOR_RESET=$"\e[0m"

# GIT
source ~/.git-completion.sh
source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1
GIT_BRANCH='$(__git_ps1 "[%s]")'

# RENDER
export PS1="${COLOR_CYAN}$WORKING_DIRECTORY${COLOR_RESET}${COLOR_YELLOW}$GIT_BRANCH${COLOR_RESET}${COLOR_CYAN}$PROMPT_SYMBOL${COLOR_RESET} "

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
