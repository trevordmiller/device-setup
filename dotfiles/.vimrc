" Enhance grep to ignore .gitignore items
set grepprg=git\ --no-pager\ grep\ --no-color\ -n\ $*
set grepformat=%f:%l:%m,%m\ %f\ match%ts,%f

" Enhance color scheme to use terminal color palette
colorscheme nord
