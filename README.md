# settings

Trevor's settings

# Desktop setup

- Install Homebrew: `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`
- Install Google Drive: `brew cask install google-drive`
- Rename Google Drive directory: `mv ~/Google\ Drive  ~/drive`
- Start Google Drive directory sync and wait for initial completion
- Run set up script: `. ~/drive/settings/index`

_Most things are taken care of automatically with the set up script ^, but some manual items are needed:_

- Clean up dock (Todoist, remove everything else)
- Dock auto hide (on)
- Menu Bar auto hide (on)
- Login items (`Todoist`, `Google Drive`, `Google Photos Backup`, `Flux`, `RescueTime`)
- Text Message Forwarding (on)
- Do Not Disturb when display is sleeping (on), when mirroring to TVs and projectors (on), allow repeated calls (on)
- Do Not Disturb scheduled (on, from 11pm-7am)
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
- Chrome extensions (1Password, Grammarly, JSON Formatter, React DevTools)
- Google safe search (on)
- Google Photos Backup size (`Original`)
- Time Machine backup on external hard drive
- Backups working in background (Google Drive, Google Photos Backup, Time Machine external drives)

# Mobile setup

- Silent toggle switch (on)
- Add multiple Touch ID fingers
- On desktop, add [iPhone as 2fa option](https://appleid.apple.com)
- Download 3rd party apps (1Password, Day One, Anki, Gospel Library, Spotify, Drive, Google Maps, Nest)
- 1password accounts (tramy, work), touch ID (on), Watchtower (on)
- Sign in to all apps
- Delete unused stock installed apps
- Dock Todoist, put other apps in `Other` folder
- Day One enable sync (on)
- Anki (sync from AnkiWeb)
- Gospel Library download (scriptures, past few conferences)
- Spotify download playlists
- Photos use iCloud Photo Library (on)
- Google Photos upload size (original)
- Clock bedtime settings
- Venmo Touch ID (on)
- Find iPhone store last known location (on)
- Health medical ID (on)
- Keyboard auto-Capitalization (off), auto-Correction (off)
- Cellular (turn off large media apps)
- Do Not Disturb scheduled (on, from 11pm-7am)
- Night Shift (scheduled, from sunset to sunrise)
- Silence (always)
- Restrictions (on), explicit (off), movies (PG-13), TV Shows (TV-14), explicit Language (off), websites (Limit Adult Content)
- Auto-Brightness (on)
- Wallpaper
- User avatar
- Text Message Forwarding (on), keep messages (1 Year)
- Safari AutoFill Names and Passwords (off), Google safe search (on)
- Do Not Track (on)

# Router setup

- Configure [OpenDNS Family Shield](https://www.opendns.com/setupguide/?url=familyshield)
