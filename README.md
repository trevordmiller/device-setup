# trevordmiller/settings

_Personal device settings_

---

# Desktop (macOS)

## Dependencies

- Install Homebrew `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`
- `brew cask install google-drive`
- Rename `~/Google\ Drive` to `~/drive` and wire up sync
- Clone this repo into `~/drive`

## Automatic

- Run the init script (`. ~/drive/settings/index`)

## Manual

_Most things are automatically set up with the init script ^, but these are the few outstanding manual items_

### System Preferences

- Account system logins (personal, work)
- Login items (`Todoist`, `Google Drive`, `Google Photos Backup`, `Seil`, `Karabiner`, `Slate`, `Flux`, `RescueTime`)
- "Text Message Forwarding" (on)
- "Do Not Disturb" when display is sleeping (on)
- "Do Not Disturb" when mirroring to TVs and projectors (on)
- "Do Not Disturb" allow repeated calls (on)
- "Find My Mac" (on)
- Dock auto hide (on)
- Menu Bar auto hide (on)
- Highlight color (nova.cyan)
- Prevent computer from sleeping automatically when display is off (on)
- Show all filename extensions (on)
- Show warning before changing an extension (off)
- Power adapter, turn display off after (never)
- Default browser (Chrome)
- Notification center keyboard shortcut (`⌘⌥⌃n`)
- "Security & Privacy" control permissions (on)
- "Glass" sound for alerts
- Alert volume (full)
- "Popcorn" sound for messages
- Keyboard
  - Modifier keys
    - caps lock (escape)
- Accessibility
  - Zoom
    - Use keyboard shortcuts to zoom (on)
- Find
  - Preferences
    - Remove items from the Trash after 30 days (on)
    - Keep folders on top when sorting by name (on)

### App Config

- Dock
  - Todoist
  - Day One
  - Anki
  - Drive
  - Remove everything else
- Spotify don't open on login (in app preferences, not system preferences like the other login apps)
- Finder sidebar condense (`Google Drive`, `Downloads`, `AirDrop`)
- Karabiner setting sync (`private.xml` with `~/drive/settings/karabiner/private.xml`)
- Key repeat spacing (`Key repeat` to `20ms` in Karabiner)
- Key repeat delay (`Delay until repeat` to `200ms` in Karabiner)
- FN + media key use (`Change f1...f12` in Karabiner)
- WakaTime config (add API key to `~/.wakatime.cfg`)
- Calendars (personal nova.cyan, tramy nova.magenta, work nova.blue)
- Chrome extensions
  - 1Password
  - Grammarly
  - JSON Formatter
  - React DevTools
  - Redux DevTools
- Google Photos Backup size (`Original`)
- Time Machine discs (`backup-home`, `backup-work`)
- Backups working in background (Google Drive, Google Photos Backup, Time Machine external drives)

---

# Keyboard (Pok3r)

- CAPS lock to FN (DIP switch 3 on)
- Programming layer (Fn+R_Ctrl to start/stop recording)
  - VIM like HJKL
    - Left: FN + H then: FN + J then: PN
    - Down: FN + J then: FN + K then: PN
    - Up: FN + K then: FN + I then: PN
    - Right: already works
    - Home: FN + I then: FN + H then: PN

---

# Mobile (iOS)

## Welcome

- Add Wifi
- Enable Location Services
- Add touch ID
- Add passcode (from 1Password)
- Confirm 2fa phone number
- Confirm Apple Pay card
- Approve iCloud Keychain (from 1Password)
- Set up Siri
- Don't send diagnostics

## From desktop

- Add iPhone as 2fa option (from appleid.apple.com)

## Silent Toggle

- On

## 1Password

- Sign in with QR code (tramy)
- Settings
  - Security
    - Touch ID (on)
    - Lock on Exit (off)
    - 1Password Watchtower (on)
  - 1Password Accounts
    - Add Account 
      - Sign in with QR code (work)

## Home Screen

### core

- Todoist
- Day One
  - Enable Sync (yes)
  - Connect Services (Calendars, Twitter, Facebook)
- Anki
  - Synchronize (download from AnkiWeb)
- Google Drive

### utils

- Calendar
  - Calendars (personal, tramy, work, events found in apps)
- Phone
- Messages
- Google Maps
- Gospel Library
  - Settings
    - Sign In
    - List Mode (on) 
  - Download (Scriptures, past few conferences)
- Safari Queue
- Spotify
  - Download (favorites)
- YouTube
- Venmo
  - Touch ID

### background

- 1Password
- Photos
  - Use iCloud Photo Library (on)
- Google Photos
  - Upload size (original)
- Clock
- Camera
- Contacts
- Find iPhone
  - Store last known location (on)
- Nest
- Sheets
- Docs
- Wallet
- Health
  - Add Medical ID
- Safari
- App Store
- Settings

_Delete other apps_

## Settings

- Cellular
  - App store, Spotify, YouTube (off)
- Do Not Disturb
  - Scheduled (on, from 10pm-7am)
  - Silence (Always)
- General
  - Restrictions (on)
    - Music & News
      - Explicit (off)
    - Movies (PG-13)
    - TV Shows (TV-14)
    - Siri
      - Explicit Language (off)
    - Websites (Limit Adult Content)
- Display & Brightness
  - Auto-Brightness (on)
  - Night Shift (scheduled,from 10pm-7am)
- Wallpaper (set)
- Touch ID & Passcode
  - Add a Fingerprint
- Clock
  - Set daily alarm
- Mail
  - Accounts
    - iCloud
    - Add Account (Google)
      - Description (work)
  - Signature (clear)
- Messages
  - Text Message Forwarding (on)
  - Keep Messages (1 Year)
- Safari
  - AutoFill
    - Names and Passwords (off)
  - Do Not Track (on)

## Share menu

- Apps
  - Todoist
  - Day One
  - Messages
  - Google Drive
  - iCloud Photo Sharing
- Extensions
  - 1Password
