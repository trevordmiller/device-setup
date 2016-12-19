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

## Screens

- 1 (Tasks, fullscreen)
  - Todoist
- 2 (Development, fullscreen split)
  - Hyper (2/3rds)
  - Chrome (1/3rd)
    - localhost
- 3 (Development Tools)
  - Chrome
    - github repo
    - docs
- 4 (Break sync)
  - Messages
  - Mail
  - Slack
  - Spotify
- 5 (Daily sync)
  - Calendar
  - Anki
  - Day One
  - Photos
  - Chrome
    - Bookmarks > daily-sync

### System Preferences

- Account system logins (personal, work)
- Login items (`Todoist`, `Google Drive`, `Google Photos Backup`, `Flux`, `RescueTime`)
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
  - Text
    - Correct spelling automatically (off)
- Mission Control
  - Automatically rearrange Spaces based on most recent use (off)
  - When switching to an application, switch to a Space with open windows for the applciation (on)
  - Group windows by application (off)
  - Displays have separate spaces (off)
- Accessibility
  - Zoom
    - Use keyboard shortcuts to zoom (on)
- Finder
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
- Screenflow
  - Countdown (0 seconds)
  - Default durations
    - Still images (3 seconds)
  - Exporting
    - 100% size

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

## Screens

- 1 (Root)
  - Todoist
- 2 (Productivity)
  - Day One
    - Enable Sync (yes)
    - Connect Services (Calendars, Twitter, Facebook)
  - Anki
    - Synchronize (download from AnkiWeb)
  - Gospel Library
    - Settings
      - Sign In
      - List Mode (on) 
    - Download (Scriptures, past few conferences)
  - Safari Queue
- 3 (Fun)
  - Spotify
    - Download (favorites)
  - YouTubde
  - Risk
  - Garage Band
  - Safari
- 4 (Utilities)
  - Drive
  - 1Password
  - Calendar
    - Calendars (personal, tramy, work, events found in apps)
  - Phone
  - Messages
  - Photos
    - Use iCloud Photo Library (on)
  - Google Photos
    - Upload size (original)
  - Camera
  - Google Maps
  - Nest
  - Contacts
  - Venmo
    - Touch ID
  - Clock
    - Alarms
  - Find iPhone
    - Store last known location (on)
  - App Store
  - Settings
- 5 (Background)
  - Google Sheets
  - Google Docs
  - Health
    - Add Medical ID
  - Wallet

_Delete other apps_

## Settings

- Keyboards
  - Auto-Capitalization (off)
  - Auto-Correction (off)
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
