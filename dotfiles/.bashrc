# PROMPT

export PS1='\[$(tput setaf 3)\]\W\\$\[$(tput sgr0)\] '

# VI BINDINGS

set -o vi

# FUNCTIONS

function personalDev() {
  parallelshell 'npm run dev' 'browser-sync start --server "dist" --files "src/**.*" --port 4000 --ui-port 4001 --no-open';
}
export -f personalDev

function domoDev() {
  parallelshell 'npm start' 'mocha ./src/**/*Test.js --compilers js:babel/register --watch --reporter min' 'browser-sync start --proxy "localhost:3000" --files "src/**.*" --port 4000 --ui-port 4001 --no-open';
}
export -f domoDev
