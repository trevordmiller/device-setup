# ==================================================================
# PATH
# ==================================================================

SCRIPTS_PATH="$HOME/projects/settings/scripts"
SCRIPT_PROJECT_PATH="$HOME/projects/settings/scripts/script-project"
HOMEBREW_PATH="/usr/local/bin"
export PATH="$PATH:$SCRIPTS_PATH:$SCRIPT_PROJECT_PATH:$HOMEBREW_PATH"


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
COLOR_STATEMENT='\[\033[01;33m\]'
COLOR_RESET='\[\033[0m\]'

# RENDER
export PS1="$NEWLINE${COLOR_USER_CURRENT_STATE}$WORKING_DIRECTORY$NEWLINE${COLOR_STATEMENT}$PROMPT_SYMBOL${COLOR_RESET} "

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

alias grep='grep -I --color=auto --exclude-dir={.git,node_modules,build,lib,.next,__snapshots__,flow-typed,priv} --exclude=package-lock.json'
alias ls='ls -G'
alias less='less -R'
