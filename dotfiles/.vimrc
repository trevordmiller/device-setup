" ==================================================================
" PLUGINS
" ==================================================================

" VIM-PLUG MANAGEMENT
call plug#begin('~/.vim/plugged')
Plug 'othree/html5.vim'
Plug 'hail2u/vim-css3-syntax'
Plug 'pangloss/vim-javascript'
Plug 'mxw/vim-jsx'
Plug 'tmhedberg/matchit'
Plug 'w0rp/ale'
Plug 'SirVer/ultisnips'
Plug 'wakatime/vim-wakatime'
Plug 'trevordmiller/nova-vim'
Plug 'rakr/vim-one'
call plug#end()

" LANGUAGES
let g:jsx_ext_required = 0

" LINTING
let g:ale_lint_on_enter = 0
let g:ale_lint_on_text_changed = 0
let g:ale_lint_on_save = 1
let g:ale_lint_delay = 0
let g:ale_linters = {
\  'javascript': ['eslint'],
\}

" SNIPPETS
let g:UltiSnipsExpandTrigger='<tab>'
let g:UltiSnipsJumpForwardTrigger='<tab>'
let g:UltiSnipsJumpBackwardTrigger='<s-tab>'

" COLOR SCHEME
colorscheme nova


" ==================================================================
" NATIVE
" ==================================================================

" DIRECTORY EXPLORER
let g:netrw_banner = 0
let g:netrw_liststyle = 0
let g:netrw_list_hide = '^\./$,^\.\./$'
let g:netrw_hide = 1
let g:netrw_sort_by = 'name'
let g:netrw_sort_direction = 'normal'
let g:netrw_localrmdir='rm -r'

" COMPATIBILITY
set nocompatible

" WINDOWS
set splitright
nnoremap <c-h> <c-w>h
nnoremap <c-j> <c-w>j
nnoremap <c-k> <c-w>k
nnoremap <c-l> <c-w>l

" STATUS LINE
set laststatus=2

" SEARCH
set incsearch
set wrapscan

" LINES
set number
set cursorline
set scrolloff=5

" TABS
set backspace=indent,eol,start
set smarttab
set expandtab
set tabstop=2
set softtabstop=2
set shiftwidth=2
set autoindent
set smartindent
set indentkeys+=O,o

" KEYWORDS
set iskeyword+=-

" WILD MENU
set wildmenu

" CLIPBOARD
set clipboard=unnamed

" HISTORY
set undofile
set undodir=~/.vim/undo_files//
set directory=~/.vim/swap_files//
set backupdir=~/.vim/backup_files//

" SPELLCHECKING
set spelllang=en
set complete+=kspell
set spellfile=$HOME/drive/settings/syncfiles/en.utf-8.add
autocmd BufRead,BufNewFile *.md setlocal spell
autocmd BufRead,BufNewFile *.txt setlocal spell
autocmd BufRead,BufNewFile */blog/* setlocal spell
autocmd FileType gitcommit setlocal spell
