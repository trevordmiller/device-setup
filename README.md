# trevordmiller/settings

_Personal setup for new machines_

---

# Automatic

- Create the `~/projects` directory and clone this repo into it
- Run `. ~/projects/settings/bootstrap.sh` to set up all the things

---

# Manual

## OSX Native App Settings

- Finder sidebar should only have two items in it: `~/projects` and `~/Downloads`
- Automatically hide Dock and menu bar
- Restore iCloud automator apps to dock
- Set up Time Machine backups
- Turn on "Find My Mac"
- Turn on "Do Not Disturb" schedule
- Notification center widgets (calendar and tasks)
- Notification center keyboard shortcut
- Sync iCloud account
- Add Internet accounts (Google, Facebook, Twitter)

## 3rd Party Stuff

_Most 3rd party packages, graphical app installs, and settings were configured automatically by the `bootstrap.sh` script ^ - these are just the extra items that need to be done manually after that_

- Download Todoist (Mac app store)
- Download iMovie (Mac app store)
- [Download RDM](http://cl.ly/2A441v222i33/download/RDM.tar.gz)
- Add WakaTime API key to `~/.wakatime.cfg`
- Sync and update Karabiner settings:
    - Link `private.xml` with `~/projects/settings/karabiner/private.xml`
    - "Key Repeat":
        - Delay until repeat: 200ms
        - Key Repeat: 20ms
    - "Change f1...f12"
- Update Seil settings:
  - CAPS lock set to fn
- Sync iTerm2 settings:
  - Sync settings to `~/projects/settings/iterm`
- Sync Dash settings:
  - Sync settings to `~/projects/settings/dash/Dash.dashsync`
  - Sync snippets to `~/projects/settings/dash/snippets.dash`
- Update Google Photos Backup to run automatically

---

# Hardware

## Pok3r Keyboard

- DIP switch 3: On (caps lock to fn)
- Programming layer (Fn+R_Ctrl to start/stop recording)
  - VIM like HJKL
    - Left: FN + H then: FN + J then: PN
    - Down: FN + J then: FN + K then: PN
    - Up: FN + K then: FN + I then: PN
    - Right: already works ;)
    - Home: FN + I then: FN + H then: PN
