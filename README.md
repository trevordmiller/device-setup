# settings

Trevor's settings

# Desktop setup

- Install Homebrew: `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`
- Install Google Drive: `brew cask install google-drive`
- Rename Google Drive directory: `mv ~/Google\ Drive  ~/drive`
- Start Google Drive directory sync and wait for initial completion
- Run set up script: `. ~/drive/settings/index`

_Most things are taken care of automatically with the set up script ^, but some manual items are needed:_

- iCloud (disable everything except Find My Mac, Keychain, Photos, and Messages inside iCloud Drive)
- Remove all dock icons
- Login items (`Google Drive`, `Google Photos Backup`, `RescueTime`)
- Text Message Forwarding (on)
- Do Not Disturb scheduled (on, from 11pm-7am), when display is sleeping (on), when mirroring to TVs and projectors (on), allow repeated calls (on), sort order (recents by app)
- Night Shift (scheduled, from sunset to sunrise)
- Find My Mac (on)
- Highlight color (Nova cyan)
- Prevent computer from sleeping automatically when display is off (on)
- Show all filename extensions (on)
- Show warning before changing an extension (off)
- Power adapter, turn display off after (never)
- Default browser (Chrome)
- Security & Privacy control permissions (on)
- Alert sound (glass)
- Alert volume (full)
- Message sound (popcorn)
- Keyboard modifier keys caps lock (escape)
- Correct spelling automatically (off)
- Remove items from the Trash after 30 days (on)
- Keep folders on top when sorting by name (on)
- Spotify open on login (off), high quality streaming (on), download each playlist
- Screenflow countdown (0 seconds), still images default duration (3 seconds)
- Garage Band add composer and author (`Perfect Particle`)
- Finder sidebar condense (`Google Drive`, `Downloads`, `AirDrop`)
- WakaTime config (add API key to `~/.wakatime.cfg`)
- Chrome extensions (1Password, SiteCop, Grammarly, React DevTools)
- Google safe search (on)
- Google Photos Backup size (`Original`)
- Time Machine backup on external hard drive
- Backups working in background (Google Drive, Google Photos Backup, Time Machine external drives)

# Mobile setup

- Hardware
  - Silent toggle switch (on)
- Download 3rd party apps (1Password, Google Photos, Gospel Library, Day One, Spotify)
- Desktop
  - [iPhone as 2fa option](https://appleid.apple.com)
- Dock
  - Todoist (web link)
  - `Applications` folder
    - Messages
    - Phone
    - Calendar
    - Gospel Library
      - Download items (scriptures, past few conferences)
    - Day One
      - Day One enable sync (on)
    - Clock
      - Bedtime (11pm - 7am)
    - Spotify
      - Download playlists
    - Maps
    - Expo
    - Safari
      - AutoFill Names and Passwords (off)
      - Google safe search (on)
  - `Background` folder
    - Settings
      - Add multiple Touch ID fingers
      - Keyboard auto-Capitalization (off), auto-Correction (off)
      - Cellular (turn off large media apps)
      - Do Not Disturb scheduled (on, from 11pm-7am), allow calls from (Amy), repeated calls (on), silence (always)
      - Night Shift (scheduled, from sunset to sunrise)
      - Silence (always)
      - Restrictions (on), explicit (off), movies (PG-13), TV Shows (TV-14), explicit Language (off), websites (Limit Adult Content)
      - Auto-Brightness (on)
      - Wallpaper
      - User avatar
      - Text message forwarding (on)
      - Keep text messages (1 Year)
      - Text message read receipt (off)
      - Do Not Track (on)
    - 1Password
      - 1password accounts (tramy, work), touch ID (on), Watchtower (on)
    - Find iPhone
      - Store last known location (on)
    - Photos
      - iCloud Photo Library (on)
    - Google Photos
      - Upload size (original)
    - Camera
    - Contacts
      - Special sound/vibrate pattern for Amy
      - Emergency bypass call and text for Amy
    - Health
      - Medical ID (fill out)
    - Wallet
      - Add debit card to Apple Wallet for Apple Pay
    - App Store
- Delete unused apps

# Router setup

- [OpenDNS Family Shield](https://www.opendns.com/setupguide/?url=familyshield)
