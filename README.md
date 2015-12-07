# trevordmiller

_Personal setup for new machines_

---

# Home directory

- **Clone this repo into `~/projects/`**
- **Symlink dotfiles to `~`:** `~/projects/trevordmiller-settings/dotfiles/bootstrap.sh`
- **Install Vim plugins:** clone [Vundle](https://github.com/VundleVim/Vundle.vim), then `:BundleInstall`

---

# Automator

- Restore iCloud automator apps

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

# Downloads

- [Karabiner](https://pqrs.org/osx/karabiner/index.html.en)
- [Seil](https://pqrs.org/osx/karabiner/seil.html.en)
- Git latest
- Node (n + `n latest`)
- npm (`npm install npm@latest -g`)
- `npm install -g `global npm packages
    - `parallelshell`
    - `browsersync`
- iTerm2 + link settings to `~/projects/trevordmiller-settings/iterm`
- Chrome
- Slate + symlink [~/.slate]()
- Anki
- Sketch (Mac app store)
- Skitch (Mac app store)
- Slack (Mac app store)
- Tweetbot (Mac app store)
- Spotify (Mac app store)
- All [Egghead gear setup apps and settings](https://trello.com/c/Emwk89vh/158-gear-setup), including Screenflow (Mac app store)

---

# Keyboards

## Common

- Karabiner "Key Repeat":
    - Delay until repeat: 30ms
    - Key Repeat: 20ms

## For Pok3r

- Karabiner: "Change f1...f12"
- DIP switch 3: On (caps lock to fn)
- Programming layer (Fn+R_Ctrl to start/stop recording)
  - VIM like HJKL
    - Left: FN + H then: FN + J then: PN
    - Down: FN + J then: FN + K then: PN
    - Up: FN + K then: FN + I then: PN
    - Right: already works ;)
    - Home: FN + I then: FN + H then: PN

## For MacBook keyboard

- Karabiner: [private.xml](https://github.com/trevordmiller/trevordmiller/blob/master/karabiner/private.xml) for vim-like arrow keys
- Seil: caps lock -> fn
