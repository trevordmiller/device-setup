# settings

Trevor's personal device settings

# Workflow

- **Task list:** use task list as a single source of truth
- **Desktop only work:** remove work items from mobile devices so work is completed on desktop only
- **Single tasking:** complete tasks one at a time, while closing any apps/windows/tabs not needed for current task 
- **Keyboard shortcuts:** use spotlight to open/switch between apps, then keyboard shortcuts inside apps
- **Checks:** once a day, run [`script-check`](scripts/script-check), then a few times a day, run [`script-microcheck`](scripts/script-microcheck) to stay up-to-date

# Desktop setup

- Install Homebrew: `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`
- Install Google Drive: `brew cask install google-drive`
- Rename Google Drive directory: `mv ~/Google\ Drive  ~/drive`
- Start Google Drive directory sync and wait for initial completion
- Run set up script: `. ~/drive/settings/index`

_Most things are taken care of automatically with the set up script ^, but some manual items are needed:_

- Add accounts (personal, work)
- Clean up dock (Todoist, remove everything else)
- Dock auto hide (on)
- Menu Bar auto hide (on)
- Login items (`Todoist`, `Google Drive`, `Google Photos Backup`, `Flux`, `RescueTime`)
- Text Message Forwarding (on)
- Do Not Disturb when display is sleeping (on), when mirroring to TVs and projectors (on), allow repeated calls (on)
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
- Finder sidebar condense (`Google Drive`, `Downloads`, `AirDrop`)
- WakaTime config (add API key to `~/.wakatime.cfg`)
- Calendars (personal Nova cyan, tramy Nova magenta, work Nova blue)
- Chrome extensions (1Password, Grammarly, JSON Formatter, React DevTools)
- Google safe search (on)
- Google Photos Backup size (`Original`)
- Time Machine backup on external hard drive
- Backups working in background (Google Drive, Google Photos Backup, Time Machine external drives)

# Mobile devices setup

- Silent toggle switch (on)
- Add multiple Touch ID fingers
- On desktop, add [iPhone as 2fa option](https://appleid.apple.com)
- Download 3rd party apps (1Password, Day One, Anki, Gospel Library, Spotify, Drive, Google Maps, Nest)
- 1password accounts (tramy, work), touch ID (on), Watchtower (on)
- Sign in to all apps
- Delete unused stock installed apps
- Dock Todoist, put other apps in `Other` folder
- Mail signature (clear)
- Day One enable sync (on)
- Anki (sync from AnkiWeb)
- Gospel Library download (scriptures, past few conferences)
- Spotify download playlists
- Calendars (personal, tramy, work, events found in apps)
- Photos use iCloud Photo Library (on)
- Google Photos upload size (original)
- Clock bedtime settings
- Venmo Touch ID (on)
- Find iPhone store last known location (on)
- Health medical ID (on)
- Keyboard auto-Capitalization (off), auto-Correction (off)
- Cellular (turn off large media apps)
- Do Not Disturb scheduled (on, from 10pm-7am)
- Silence (always)
- Restrictions (on), explicit (off), movies (PG-13), TV Shows (TV-14), explicit Language (off), websites (Limit Adult Content)
- Auto-Brightness (on)
- Night Shift (scheduled,from 10pm-7am)
- Wallpaper
- User avatar
- Text Message Forwarding (on), keep messages (1 Year)
- Safari AutoFill Names and Passwords (off), Google safe search (on)
- Do Not Track (on)

# Router setup

- Configure [OpenDNS Family Shield](https://www.opendns.com/setupguide/?url=familyshield)
