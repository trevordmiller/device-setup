# settings

Trevor's settings

# Desktop setup

- Install Homebrew: `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`
- Clone this repo into `~/`
- Run set up script: `. ~/settings/index`

_Most things are taken care of automatically with the set up script ^, but some manual items are needed:_

- iCloud (disable everything except Contacts, iBooks, Find My Mac, Keychain, Photos, and Messages inside iCloud Drive)
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
- Finder sidebar condense (`trevordmiller`)
- WakaTime config (add API key to `~/.wakatime.cfg`, automatically done with command line prompt on first run of Vim)
- Chrome extensions (1Password, Grammarly, React DevTools)
- Google safe search (on)
- Google Photos Backup size (`Original`)
- Time Machine backup on external hard drive
- Backups working in background (Time Machine external drives)

# Mobile setup

- Hardware
  - Silent toggle switch (on)
- Download 3rd party apps (1Password, Google Photos, Gospel Library, Day One, Spotify)
- Desktop
  - [iPhone as 2fa option](https://appleid.apple.com)
- `Applications` folder
  - Todoist
  - Calendar
  - Messages
  - Phone
  - Mail
  - Anki
  - Gospel Library
    - Download items (scriptures, past few conferences)
  - Instapaper
    - Image thumbnails (off)
    - Scroll Mode (iBooks style pagination)
    - Instant Sync (on)
    - Sharing accounts (twitter, post notes)
    - Download limits (minimum)
  - iBooks
    - Download epubs
  - Day One
    - Day One enable sync (on)
  - Safari
    - AutoFill Names and Passwords (off)
    - Google safe search (on)
  - Clock
    - Bedtime (11pm - 7am)
  - Spotify
    - Download playlists
  - Maps
  - Lyft
  - 1Password
    - 1password accounts (tramy, work), touch ID (on), Watchtower (on)
  - Camera
  - Photos
    - iCloud Photo Library (on)
  - Google Photos
    - Upload size (original)
  - Contacts
    - Special sound/vibrate pattern for Amy
    - Emergency bypass call and text for Amy
  - Wallet
    - Add debit card to Apple Wallet for Apple Pay
  - Find iPhone
    - Store last known location (on)
  - Health
    - Medical ID (fill out)
  - App Store
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
- Delete unused apps

# Router setup

- [OpenDNS Family Shield](https://www.opendns.com/setupguide/?url=familyshield)
