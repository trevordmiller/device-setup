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
let g:ctrlp_working_path_mode = 0

" JSX
let g:jsx_ext_required = 0


" =====================================
" CORE SETTINGS
" ====================================

" TIMEOUTS
set timeoutlen=1000 ttimeoutlen=10

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

" GF
set suffixesadd+=.js

" SPLITS
set splitright
nnoremap <C-J> <C-W><C-J>
nnoremap <C-K> <C-W><C-K>
nnoremap <C-L> <C-W><C-L>
nnoremap <C-H> <C-W><C-H>

" STYLE
syntax enable
colorscheme base16-ocean
set background=dark
set nowrap
set cursorline
set number
set backspace=indent,eol,start
set smarttab
set expandtab
set tabstop=2
set softtabstop=2
set shiftwidth=2
