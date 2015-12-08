# GIT AUTOCOMPLETE
source ~/.git-completion.sh

# GIT PROMPT
source ~/.git-prompt.sh
export GIT_PS1_SHOWDIRTYSTATE=1

# CUSTOM PROMPT
WORKING_DIRECTORY="\W"
GIT_BRANCH='$(__git_ps1 "[%s]")'
ACCESS="\$"
COLOR_BASE="\[$(tput sgr0)\]"
COLOR_CYAN="\[$(tput setaf 6)\]"
COLOR_YELLOW="\[$(tput setaf 3)\]"
export PS1="$COLOR_CYAN$WORKING_DIRECTORY$COLOR_YELLOW$GIT_BRANCH$COLOR_YELLOW$ACCESS$COLOR_BASE "

# VI BINDINGS
set -o vi

# WORKFLOW FUNCTIONS
function personalDev {
  parallelshell 'npm run dev' 'browser-sync start --server "dist" --files "src/**.*" --port 4000 --ui-port 4001 --no-open'
}
export -f personalDev
function domoDev {
  parallelshell 'npm start' 'mocha ./src/**/*Test.js --compilers js:babel/register --watch --reporter min' 'browser-sync start --proxy "localhost:3000" --files "src/**.*" --port 4000 --ui-port 4001 --no-open'
}
export -f domoDev
