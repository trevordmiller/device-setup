" PLUGINS
call plug#begin('~/.vim/plugged')
Plug 'ctrlpvim/ctrlp.vim'
Plug 'tpope/vim-vinegar'
Plug 'mileszs/ack.vim'
Plug 'SirVer/ultisnips'
Plug 'mbbill/undotree'
Plug 'tmhedberg/matchit'
Plug 'othree/html5.vim'
Plug 'hail2u/vim-css3-syntax'
Plug 'pangloss/vim-javascript'
Plug 'moll/vim-node'
Plug 'mxw/vim-jsx'
Plug 'w0rp/ale'
Plug 'wakatime/vim-wakatime'
Plug 'trevordmiller/nova-vim'
Plug 'rakr/vim-one'
call plug#end()

" COMPATABILITY
set nocompatible

" LINES
set cursorline
set nowrap
set textwidth=0
set wrapmargin=0
set number
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
autocmd FileType gitcommit setlocal spell

" LANGUAGE EXTENDING
let g:jsx_ext_required = 0

" FILE SEARCHING
set incsearch
set wrapscan

" PROJECT SEARCHING
let g:ackprg = 'ag --hidden --path-to-ignore ~/.agignore --vimgrep' 

" PROJECT EXPLORING
let g:netrw_liststyle = 0
let g:netrw_sort_by = 'name'
let g:netrw_sort_direction = 'normal'
let g:netrw_localrmdir='rm -r'

" FUZZY FINDING
let g:ctrlp_working_path_mode = 0
let g:ctrlp_user_command = 'ag %s -l --hidden --path-to-ignore ~/.agignore --nocolor -g ""'
let g:ctrlp_use_caching = 0

" SNIPPETS
let g:UltiSnipsExpandTrigger='<tab>'
let g:UltiSnipsJumpForwardTrigger='<tab>'
let g:UltiSnipsJumpBackwardTrigger='<s-tab>'
set dictionary+=~/drive/settings/syncfiles/snippet-names.txt

" LINTING
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

" WINDOW SPLITS
set splitright
nnoremap <c-h> <c-w>h
nnoremap <c-j> <c-w>j
nnoremap <c-k> <c-w>k
nnoremap <c-l> <c-w>l

" STATUS LINE
set laststatus=2

" COLOR SCHEME
colorscheme nova
