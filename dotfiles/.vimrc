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
set wildignore+=*.zip,*.png,*.jpg,*.gif,*.pdf,*DS_Store*,*/.git/*,*/node_modules/*,*/build/*,*/lib/*,*/.next/*,*/__snapshots__/*,*/flow-typed/*,package-lock.json
set grepprg=grep\ -In\ --exclude-dir={.git,node_modules,build,lib,.next,__snapshots__,flow-typed}\ --exclude=package-lock.json

" SPELLCHECK
set spelllang=en
set complete+=kspell
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

" PERFORMANCE
set synmaxcol=200


" ==================================================================
" PLUGIN MANAGEMENT
" ==================================================================

call plug#begin('~/.vim/plugged')

" COLOR SCHEME
Plug 'trevordmiller/nova-vim'

" EXTENDED LANGUAGES
Plug 'othree/html5.vim'
Plug 'hail2u/vim-css3-syntax'
Plug 'pangloss/vim-javascript'
Plug 'mxw/vim-jsx'
Plug 'elixir-lang/vim-elixir'

" EXTENDED % MATCHING
Plug 'tmhedberg/matchit'

" SNIPPETS
Plug 'KeyboardFire/vim-minisnip'

" INLINE LINTING/TYPE CHECKING/FORMATTING
Plug 'w0rp/ale'

" TIME TRACKING
Plug 'wakatime/vim-wakatime'

call plug#end()


" ==================================================================
" PLUGIN CONFIG
" ==================================================================

" COLOR SCHEME
colorscheme nova

" EXTENDED LANGUAGES
let g:jsx_ext_required = 0

" SNIPPETS
let g:minisnip_dir = '~/projects/settings/dotfiles/.snippets/'
set dictionary=~/projects/settings/dotfiles/.snippets/index.txt

" INLINE LINTING/TYPE CHECKING/FORMATTING
let g:ale_linters = {
\  'javascript': ['eslint', 'flow', 'prettier'],
\}
let g:ale_fixers = {
\  'javascript': ['eslint', 'prettier'],
\}
let g:ale_javascript_prettier_options = '--no-semi --single-quote --trailing-comma es5'
let g:ale_fix_on_save = 1
