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

- Run the init script (`. ~/drive/settings/scripts/script-init`)

## Manual

_Most things are automatically set up with the init script ^, but these are the few outstanding manual items_

### System Preferences

- Account system logins (iCloud, )
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

### App Config

- Finder sidebar condense (`Google Drive`, `Downloads`, `AirDrop`)
- Karabiner setting sync (`private.xml` with `~/drive/settings/karabiner/private.xml`)
- Key repeat spacing (`Key repeat` to `20ms` in Karabiner)
- Key repeat delay (`Delay until repeat` to `200ms` in Karabiner)
- FN + media key use (`Change f1...f12` in Karabiner)
- Caps lock -> FN (CAPS lock to FN in Seil)
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

- Todoist (Dock)
- Day One
  - Enable Sync (yes)
  - Connect Services (Calendars, Twitter, Facebook)
- Gospel Library
  - Settings
    - Sign In
    - List Mode (on) 
  - Download (Scriptures, past few conferences)
- Phone
- Messages
- Calendar
  - Calendars (personal, tramy, work, events found in apps)
- Mail
- Slack
- Anki
  - Synchronize (download from AnkiWeb)
- Safari Queue
- Google Drive
- Sleep Cycle
  - Microphone, push notifications (allow)
- Google Maps
- Spotify
  - Download (favorites)
- Twitter
- Facebook
- YouTube
- Mint
- Chrome
- "Background" folder
  - 1Password
  - Photos
    - Use iCloud Photo Library (on)
  - Google Photos
    - Upload size (original)
  - Camera
  - Health
    - Add Medical ID
  - Contacts
  - Find iPhone
    - Store last known location (on)
  - Nest
  - IF
  - Sheets
  - Docs
  - Wallet
  - FaceTime
  - Clock
  - Safari
  - App Store
  - Settings

_Delete other apps_

## Settings

- Cellular
  - 
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
- Mail
  - A    - “Phone” phonecalls, voicemails.ccounts
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
- Twitter (login)
- Facebook (login)
  - Calendars (off)
      


- Add 2nd thumb to touch ID




- Account system logins (iCloud, Facebook, Twitter)
- "Text Message Forwarding" (on)
- "Do Not Disturb" schedule (10pm - 7am)
- "Do Not Disturb" allow favorites (on)
- "Find My Phone" (on)
- "Restrictions" (on)
- Auto brightness (on)
- "Night Shift" schedule (8pm - 8am)
- Install 1Password for app logins
- Install and configure apps
  - Home screen
    - Todoist
    - Day One
    - _Calendar_
    - _Mail_
    - Google Drive
    - _Health_
    - Sleep Cycle
    - Google Maps
    - Spotify
    - Gospel Library
      - LDS account data sync
    - Anki
      - Anki web data sync
    - _iBooks_
    - Buffer
    - Mint
    - _Messages_
    - _Phone_
    - _Facetime_
    - Photos
    - YouTube
    - Chrome
    - _Settings_
  - Background folder
    - 1Password
    - _Clock_
      - Alarms (7am)
    - _Camera_
    - _Contacts_
    - _Find iPhone_
    - Google Photos
      - Backup size (`Original`)
    - IF
    - Nest
    - QS Access

## Share menu

- Apps
  - Todoist
  - Day One
  - Messages
  - Mail
  - Slack
  - Google Drive
  - iCloud Photo Sharing
  - Twitter
  - Facebook
- Extensions
  - 1Password
