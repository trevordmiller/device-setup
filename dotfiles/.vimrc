" ==================================================================
" NATIVE CONFIG
" ==================================================================

" WINDOW NAVIGATION
nnoremap <c-h> <c-w>h
nnoremap <c-j> <c-w>j
nnoremap <c-k> <c-w>k
nnoremap <c-l> <c-w>l

" STATUS
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

" DIRECTORY EXPLORER
let g:netrw_banner = 0
let g:netrw_liststyle = 0
let g:netrw_list_hide = '^\./$,^\.\./$'
let g:netrw_hide = 1
let g:netrw_sort_by = 'name'
let g:netrw_sort_direction = 'normal'
let g:netrw_localrmdir='rm -r'

" COMMAND-LINE COMPLETION
set wildmenu

" PATTERN IGNORING
set wildignore+=*.zip,*.png,*.jpg,*.gif,*.pdf,*DS_Store*,*/.git/*,*/node_modules/*,*/build/*,*/lib/*,*/.next/*,*/__snapshots__/*,yarn.lock
set grepprg=grep\ -In\ --exclude-dir={.git,node_modules,build,lib,.next,__snapshots__}\ --exclude=yarn.lock

" SPELLCHECK
set spelllang=en
set complete+=kspell
set spellfile=$HOME/drive/settings/syncfiles/en.utf-8.add
autocmd BufRead,BufNewFile *.md setlocal spell
autocmd BufRead,BufNewFile *.txt setlocal spell
autocmd BufRead,BufNewFile */blog/* setlocal spell
autocmd FileType gitcommit setlocal spell

" CLIPBOARD
set clipboard=unnamed

" HISTORY
set undofile
set undodir=~/.vim/undo_files//
set directory=~/.vim/swap_files//
set backupdir=~/.vim/backup_files//


" ==================================================================
" PLUGIN MANAGEMENT
" ==================================================================

call plug#begin('~/.vim/plugged')

" COLOR SCHEMES
Plug 'trevordmiller/nova-vim'
Plug 'rakr/vim-one'

" EXTENDED LANGUAGES
Plug 'othree/html5.vim'
Plug 'hail2u/vim-css3-syntax'
Plug 'pangloss/vim-javascript'
Plug 'mxw/vim-jsx'

" EXTENDED % MATCHING
Plug 'tmhedberg/matchit'

" SNIPPETS
Plug 'SirVer/ultisnips'

" INLINE LINTING
Plug 'w0rp/ale'

" TIME TRACKING
Plug 'wakatime/vim-wakatime'

call plug#end()


" ==================================================================
" PLUGIN CONFIG
" ==================================================================

" COLOR SCHEMES
colorscheme nova

" EXTENDED LANGUAGES
let g:jsx_ext_required = 0

" SNIPPETS
let g:UltiSnipsExpandTrigger='<tab>'
let g:UltiSnipsJumpForwardTrigger='<tab>'
let g:UltiSnipsJumpBackwardTrigger='<s-tab>'
