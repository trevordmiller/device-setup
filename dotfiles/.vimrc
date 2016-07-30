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

" PROJECT FIND & REPLACE
Plugin 'Olical/vim-enmasse'

" SYNTAX CHECKING
Plugin 'scrooloose/syntastic'

" LANGUAGE EXTENSIONS
Plugin 'othree/html5.vim'
Plugin 'pangloss/vim-javascript'
Plugin 'salomvary/vim-eslint-compiler'
Plugin 'mxw/vim-jsx'
Plugin 'tmhedberg/matchit'

" TIME TRACKING
Plugin 'wakatime/vim-wakatime'

" AESTHETICS
Plugin 'chriskempson/base16-vim'

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

" CTRLP
let g:ctrlp_working_path_mode = 0

" SYNTASTIC
let g:syntastic_check_on_open = 1
let g:syntastic_always_populate_loc_list = 1
let g:syntastic_check_on_wq = 0
let g:syntastic_javascript_checkers = ['eslint']
let g:syntastic_javascript_eslint_exec = 'eslint_d'

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
nnoremap <C-J> <C-W><C-J>
nnoremap <C-K> <C-W><C-K>
nnoremap <C-L> <C-W><C-L>
nnoremap <C-H> <C-W><C-H>

" SWAP FILES
set noswapfile

" GOTO FILES
set suffixesadd+=.js

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
set wildignore+=*/build/*
set wildignore+=*/lib/*
set wildignore+=*/dist/*
set wildignore+=*/compiled/*
set wildignore+=*/cache/*

" CLIPBOARD
set clipboard=unnamed

" AESTHETICS
syntax enable
set background=dark
colorscheme base16-ocean
