# ==================================================================
# PATH
# ==================================================================

SCRIPTS_PATH="$HOME/drive/settings/scripts"
SCRIPTS_UTILS_PATH="$HOME/drive/settings/scripts/utils"
HOMEBREW_PATH="/usr/local/bin"
YARN_PATH="$HOME/.yarn/bin"
RBENV_PATH="$HOME/.rbenv/bin"
export PATH="$PATH:$SCRIPTS_PATH:$SCRIPTS_UTILS_PATH:$HOMEBREW_PATH:$YARN_PATH:$RBENV_PATH"


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

alias grep='grep -I --color=auto --exclude-dir={.git,node_modules,build,lib,.next,__snapshots__} --exclude=yarn.lock'
alias ls='ls -G'
alias less='less -R'


# ==================================================================
# RAILS
# ==================================================================

if which rbenv > /dev/null; then eval "$(rbenv init -)"; fi
