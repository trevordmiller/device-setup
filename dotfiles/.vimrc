" ==================================================================
" PLUGINS
" ==================================================================

" PLUGIN SYSTEM SETUP START
set nocompatible
filetype off
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

" PLUGIN SYSTEM
Plugin 'VundleVim/Vundle.vim'

" PROJECT NAVIGATION
Plugin 'tpope/vim-vinegar'
Plugin 'ctrlpvim/ctrlp.vim'

" GIT
Plugin 'tpope/vim-fugitive'

" PROJECT FIND & REPLACE
Plugin 'Olical/vim-enmasse'

" SYNTAX CHECKING
Plugin 'w0rp/ale'

" MOTION EXTENSIONS
Plugin 'tmhedberg/matchit'
Plugin 'moll/vim-node'

" SNIPPETS
Plugin 'SirVer/ultisnips'

" SYNTAX GROUP EXTENSIONS
Plugin 'othree/html5.vim'
Plugin 'hail2u/vim-css3-syntax'
Plugin 'pangloss/vim-javascript'
Plugin 'mxw/vim-jsx'

" TIME TRACKING
Plugin 'wakatime/vim-wakatime'

" AESTHETICS
Plugin 'trevordmiller/nova-vim'

" DEMOS
Plugin 'rakr/vim-one'

" PLUGIN SYSTEM SETUP END
call vundle#end()
filetype plugin on


" ==================================================================
" PLUGIN SETTINGS
" ==================================================================

" NETRW
let g:netrw_liststyle = 0
let g:netrw_sort_by = 'name'
let g:netrw_sort_direction = 'normal'
let g:netrw_localrmdir='rm -r'

" CTRLP
let g:ctrlp_working_path_mode = 0

" ALE
let g:ale_lint_on_enter = 0
let g:ale_lint_on_text_changed = 0
let g:ale_lint_on_save = 1
let g:ale_lint_delay = 0
let g:ale_linters = {
\  'javascript': ['eslint'],
\  'sh': ['shellcheck'],
\  'json': ['jsonlint'],
\  'css': ['csslint'],
\}

" SNIPPETS
let g:UltiSnipsSnippetsDir='~/drive/settings/snippets'
let g:UltiSnipsExpandTrigger='<tab>'
let g:UltiSnipsJumpForwardTrigger='<tab>'
let g:UltiSnipsJumpBackwardTrigger='<s-tab>'

" VIM-JSX
let g:jsx_ext_required = 0

" ==================================================================
" CORE SETTINGS
" ==================================================================

" MODES
set timeoutlen=1000
set ttimeoutlen=10

" KEYWORDS
set iskeyword+=-

" LINES
set cursorline
set nowrap
set textwidth=0
set wrapmargin=0
set number
set scrolloff=5

" TABS
filetype indent on
set backspace=indent,eol,start
set smarttab
set expandtab
set tabstop=2
set softtabstop=2
set shiftwidth=2
set autoindent
set smartindent
set indentkeys+=O,o

" SEARCH
set incsearch
set wrapscan

" STATUS LINE
set laststatus=2

" WINDOW SPLITS
set splitright
nnoremap <c-h> <c-w>h
nnoremap <c-j> <c-w>j
nnoremap <c-k> <c-w>k
nnoremap <c-l> <c-w>l

" SWAP FILES
set noswapfile

" WILD MENU
set wildmenu
set wildignore+=*.zip
set wildignore+=*.png,*.jpg,*.gif
set wildignore+=*.pdf
set wildignore+=*.swp
set wildignore+=*/.git/*
set wildignore+=*DS_Store*
set wildignore+=Icon
set wildignore+=*/node_modules/*
set wildignore+=npm-debug.log
set wildignore+=.eslintcache
set wildignore+=*.snap
set wildignore+=*/build/*
set wildignore+=*/lib/*
set wildignore+=*/dist/*
set wildignore+=*/compiled/*
set wildignore+=*/cache/*

" CLIPBOARD
set clipboard=unnamed

" SPELLCHECK
set spelllang=en
set complete+=kspell
set spellfile=$HOME/drive/settings/syncfiles/en.utf-8.add
autocmd BufRead,BufNewFile *.md setlocal spell
autocmd BufRead,BufNewFile *.txt setlocal spell
autocmd FileType gitcommit setlocal spell

" AESTHETICS
set termguicolors
colorscheme nova
