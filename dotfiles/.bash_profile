# =====================================
# CUSTOM PROMPT
# =====================================

source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1
WORKING_DIRECTORY="\W"
GIT_BRANCH='$(__git_ps1 "[%s]")'
USER="\$"
COLOR_BASE="\[$(tput sgr0)\]"
COLOR_YELLOW="\[$(tput setaf 3)\]"
export PS1="$COLOR_YELLOW$WORKING_DIRECTORY$GIT_BRANCH$USER$COLOR_BASE "


# =====================================
# VI BINDINGS
# =====================================

set -o vi


# =====================================
# GIT AUTOCOMPLETE
# =====================================

source ~/.git-completion.sh


# =====================================
# FUNCTIONS
# =====================================

function screencastScripts {
  killall node ; parallelshell 'npm run dev' 'browser-sync start --server "dist" --files "src/**.*" --port 4000 --ui-port 4001 --no-open'
}
export -f screencastScripts

function domoScripts {
  killall node ; parallelshell 'npm start' 'npm run test:unit --watch' 'browser-sync start --proxy "localhost:3000" --files "src/**.*" --port 4000 --ui-port 4001 --no-open'
}
export -f domoScripts

