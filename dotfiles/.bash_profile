# ==================================================================
# VI STYLE EDITING
# ==================================================================

set -o vi


# ==================================================================
# HOMEBREW LOCATION
# ==================================================================

export PATH="/usr/local/bin:$PATH"


# ==================================================================
# CUSTOM PROMPT
# ==================================================================

# DIRECTORY
WORKING_DIRECTORY="\W"

# UTILITY
PROMPT_SYMBOL="\$"

# GIT PROMPT
source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1
GIT_BRANCH='$(__git_ps1 "[%s]")'

# COLORS
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

# PROMPT
export PS1="$COLOR_CYAN$WORKING_DIRECTORY$COLOR_ORANGE$GIT_BRANCH$COLOR_CYAN$PROMPT_SYMBOL$COLOR_BASE "

# GIT COMPLETION
source ~/.git-completion.sh
