# trevordmiller

Personal setup for new machines. Cherry-picking from Time Machine should be the easiest backup option, but this serves as a back-up/documentation.

---

# Shell

- ZSH + [~/.zshrc]() + [pure theme](https://github.com/sindresorhus/pure)

---

# Editor

- [~/.vimrc]()
- Install Vundle: `git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim`
- :BundleInstall

---

# Keyboards

## Common

- [Karabiner](https://pqrs.org/osx/karabiner/index.html.en) "Key Repeat":
    - Delay until repeat: 30ms
    - Key Repeat: 20ms

## For Pok3r

- [Karabiner](https://pqrs.org/osx/karabiner/index.html.en): "Change f1...f12"
- DIP switch 3: On (caps lock to fn)
- Programming layer (Fn+R_Ctrl to start/stop recording)
  - VIM like HJKL
    - Left: FN + H then: FN + J then: PN
    - Down: FN + J then: FN + K then: PN
    - Up: FN + K then: FN + I then: PN
    - Right: already works ;)
    - Home: FN + I then: FN + H then: PN

## For MacBook keyboard

- [Karabiner](https://pqrs.org/osx/karabiner/index.html.en): [private.xml](https://github.com/trevordmiller/trevordmiller/blob/master/karabiner/private.xml) for vim-like arrow keys
- [Seil](https://pqrs.org/osx/karabiner/seil.html.en): caps lock -> fn

---

# Automator

- iCloud automator apps

---

# OS Config

- Time Machine backups
- Show hidden files: `defaults write com.apple.finder AppleShowAllFiles YES`, then `option + right click` finder icon and relaunch
- Set screenshot folder: `defaults write com.apple.screencapture location ~/Pictures/Screenshots/`, then `killall SystemUIServer`
- Turn on "Find My Mac"
- Turn on "Do Not Disturb" schedule
- Notification center match iPhone + Notification center keyboard shortcut
- Automatically hide Dock
- internet accounts like Facebook, Twitter etc.
- Reminders + iCloud
- Calendar + iCloud
- Mail + iCloud
- Messages + iCloud

---

# Software

- Git + git config
    ```
    $ git config --global user.name "John Doe"
    $ git config --global user.email johndoe@example.com
    ```
- Node + n + `n stable`
- npm + npm config:
    ```
    $ npm config set init.author.name "John Doe"
    $ npm config set init.author.email johndoe@example.com
    $ npm config set init.author.url http://johndoe.com
    $ npm config set init.license MIT
    ```
- iTerm2 + [???]()
- Chrome
- Slate + [~/.slate]()
- Anki
- Sketch (Mac app store)
- Skitch (Mac app store)
- Slack (Mac app store)
- Tweetbot (Mac app store)
- Spotify (Mac app store)
- All [Egghead gear setup apps and settings](https://trello.com/c/Emwk89vh/158-gear-setup), including Screenflow (Mac app store)
