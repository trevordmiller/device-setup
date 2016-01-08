" =====================================
" VUNDLE
" =====================================

" VUNDLE SETUP START
set nocompatible
filetype off
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

" GITHUB PLUGINS
Plugin 'VundleVim/Vundle.vim'
Plugin 'tpope/vim-vinegar'
Plugin 'ctrlpvim/ctrlp.vim'
Plugin 'Olical/vim-enmasse'
Plugin 'elzr/vim-json'
Plugin 'pangloss/vim-javascript'
Plugin 'mxw/vim-jsx'
Plugin 'chriskempson/base16-vim'

" VUNDLE SETUP END
call vundle#end()
filetype plugin on


" =====================================
" PLUGIN SETTINGS
" =====================================

" CTRLP
" Index from initial directory opened with vim
let g:ctrlp_working_path_mode = 0

" VIM-JSX
" Use JSX plugin on .js files
let g:jsx_ext_required = 0


" =====================================
" CORE SETTINGS
" ====================================

" SWAP FILES
set noswapfile

" SYNTAX COLOR
syntax enable
set background=dark
colorscheme base16-ocean

" SPLITS
set splitright
nnoremap <C-J> <C-W><C-J>
nnoremap <C-K> <C-W><C-K>
nnoremap <C-L> <C-W><C-L>
nnoremap <C-H> <C-W><C-H>

" LINES
set cursorline
set nowrap
set number

" GOTO FILE
" Allow use of `gf` with file imports that don't have an extension
set suffixesadd+=.js

" SCROLLOFF
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

" STATUSLINE
set laststatus=2

" SEARCH
set ignorecase
set smartcase
set wrapscan

" WILD MENU
set wildmenu
set wildignore+=*/.git/*
set wildignore+=*/cache/*
set wildignore+=*/compiled/*
set wildignore+=*/dist/*
set wildignore+=*/node_modules/*
set wildignore+=npm-debug.log
set wildignore+=*.zip
set wildignore+=*.png,*.jpg,*.gif
set wildignore+=*.pdf
set wildignore+=*DS_Store*
set wildignore+=*.swp
