# Usage
## Notice
This software/romhack is still heavily in beta. Research, testing, and development are still being done.
That being said, if you do encounter any type of unexpected crash, please report them back to me (@mid#1749) on discord by Direct Message.

## Building

1) Open the romhack-patcher
2) Click "Open Patch" > Select the tpgz_0.0.1.patch file
3) Click "Open ISO" > Select your Twilight Princess NTSC-U (USA) ISO
4) Click Apply and then select a location for the new ISO, type in a name similar to "tpgz.iso"
#### !!!!!!!!!!!!!! Note: The window may freeze while the patcher is working. This is normal. !!!!!!!!!!!!!!
5) When it is done patching open in Dolphin or on console via Nintendont

ENJOY!

## Usage
### Controls:
- L + R + D-down to open menu
- A to enter a menu
- B to leave a menu

### Cheats Menu:
- A to Activate / Deactivate

### Inventory:
- D-Left/D-right to scroll/change
- X to remove selected entry
- Y to add an entry

## Memory
- D-Up/D-Down to select watch entry
- A to enter Watch Entry
- D-Left/D-right to select option
### Options
- X/Y: D-Up/D-Down to increase/decrease
- Hex: A to toggle
- Type: D-Up/D-Down to change
- Show: A to toggle
- Offset: Y to enable, X to disable (enabling offset sets the selected address as a pointer)
### Address / Offset (if enabled):
- A to enter
- B to exit
- D-Left/D-right to select "digit"
- D-Up/D-Down to change (all in hex)

## Additions

I've provided some default memory values for the sake of convenience, they are as follows in the list:
- Link's Speed
- Link's Angle
- Link's X Position
- Link's Y Position
- Link's Z Position
- Current Stage
- Current Room
- Current Spawn Point
- Boss Flags

## Helping Out

Bonus: Note this is <em>NOT REQUIRED</em> but is very helpful for development incase you encounter a crash or bug.

If you are using Dolphin to play this romhack you can play with dolphin in "Debug Mode" to get
detailed information about any crashes and bugs that may occur. If you wish to enable this you can follow these steps:
1.	Place the bundled "GZ2E01.map" file in your "Documents/Dolphin Emulator/Maps" folder. (https://imgur.com/QlaGTEC)
2.	a) With newer versions of Dolphin this can be enabled by selecting "Show Debugging UI" under Config > Interface. (https://imgur.com/eoWsPZn)
	b) With older version this must be enabled by adding a launch command to a Dolphin shortcut. (https://imgur.com/a/9ePOvWL)
3.	Whith those options enabled Open the Debug Dolphin and under "View" menu, enable "Log" and "Log Configuration". (https://imgur.com/uAswyPY)
4.	Make sure under the "Log Configuration" tab "OSReport" is selected; all others can be either on or off. (https://imgur.com/3KwkA3d)
	(though you probably want them off to not clutter things up)

This has the benefit of outputting to the log any crash info that may occur, allowing for better research and
quicker resolutions. If you do decide to take the extra steps to enable these added features,
please screenshot Dolphin's Log Window if any crashes should occur, and provide them when reporting a bug.
