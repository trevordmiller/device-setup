# =====================================
# CUSTOM PROMPT
# =====================================

source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1
WORKING_DIRECTORY="\W"
GIT_BRANCH='$(__git_ps1 "[%s]")'
COLOR_BASE="\[$(tput sgr0)\]"
COLOR_BLACK="\[$(tput setaf 0)\]"
COLOR_RED="\[$(tput setaf 1)\]"
COLOR_GREEN="\[$(tput setaf 2)\]"
COLOR_YELLOW="\[$(tput setaf 3)\]"
COLOR_BLUE="\[$(tput setaf 4)\]"
COLOR_MAGENTA="\[$(tput setaf 5)\]"
COLOR_CYAN="\[$(tput setaf 6)\]"
COLOR_WHITE="\[$(tput setaf 7)\]"
COLOR_ORANGE="\[$(tput setaf 9)\]"
COLOR_VIOLET="\[$(tput setaf 13)\]"
export PS1="\n$COLOR_CYAN$WORKING_DIRECTORY$COLOR_ORANGE$GIT_BRANCH$COLOR_CYAN\n\$$COLOR_BASE "


# =====================================
# VI BINDINGS
# =====================================

set -o vi


# =====================================
# GIT AUTOCOMPLETE
# =====================================

source ~/.git-completion.sh
