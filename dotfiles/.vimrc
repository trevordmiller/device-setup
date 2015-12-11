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
Plugin 'pangloss/vim-javascript'
Plugin 'mxw/vim-jsx'
Plugin 'chriskempson/base16-vim'

" VUNDLE SETUP END
call vundle#end() 
filetype plugin indent on


" =====================================
" PLUGIN SETTINGS
" =====================================

" CTRL P
" Index from initial directory opened with vim
let g:ctrlp_working_path_mode = 0

" JSX
" Use JSX plugin on .js files
let g:jsx_ext_required = 0


" =====================================
" CORE SETTINGS
" ====================================

" THEME 
syntax enable
colorscheme base16-ocean
set background=dark

" SWAP FILES
set noswapfile

" SEARCH
set ignorecase
set smartcase
set wrapscan

" WILD MENU
set wildmenu
set wildignore+=.git
set wildignore+=node_modules
set wildignore+=cache
set wildignore+=*.swp
set wildignore+=*.zip
set wildignore+=*.png,*.jpg,*.gif
set wildignore+=*.pdf
set wildignore+=*DS_Store*

" SPLITS
set splitright
nnoremap <C-J> <C-W><C-J>
nnoremap <C-K> <C-W><C-K>
nnoremap <C-L> <C-W><C-L>
nnoremap <C-H> <C-W><C-H>

" TABS
set backspace=indent,eol,start
set smarttab
set expandtab
set tabstop=2
set softtabstop=2
set shiftwidth=2

" LINES
set nowrap
set cursorline
set number

" GOTO FILE
" Allow use of `gf` with file imports that don't have an explicit extension
set suffixesadd+=.js
