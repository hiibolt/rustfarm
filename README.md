# rustfarm

<img src="https://github.com/user-attachments/assets/321518bc-3958-47d8-b24e-27fc13bb7824" width=250>

## About
An automatic farm for a variety of types of XP and currency in multiple games, written in Rust with TUI.

<img src="https://github.com/user-attachments/assets/505e699d-d085-4e20-a32a-2bb3f60fcba3" width=500>

### Supported Games:
- **Tom Clancy's Rainbow Six Siege**
- **Apex Legends** (*under construction*)
- **XDefiant** (*under construction*)


## Setup
### From Binary
Download `rustfarm`:

- Click the green "Code" button above
- Click "Download Zip"
- Extract the ZIP

After you have configured the game (see below), launch `rustfarm`:

- Double click `rustfarm.exe`
### From Source
Install the following:

- Install [Git for Windows](https://gitforwindows.org/)
- Install [Rust for Windows](https://www.rust-lang.org/tools/install)

Open **Windows Powershell** and run the following:

- `> git clone https://github.com/hiibolt/rustfarm.git`
- `> cd rustfarm`

After you have configured the game (see below), launch `rustfarm`:

- `> cargo run`

## Game Configurations
### Rainbow Six Siege
#### Settings and Startup
- Open Siege on an account eligible to play Casual, 2FA not required
- Set the following settings:
  - Resolution: 1920x1080
  - Display Mode: Borderless (If you encounter issues, try Fullscreen)
  - Aspect Ratio: 4:3
  - HUD Display Area: 100
  - Menu Display Area: 100
- **IMPORTANT** - Navigate back to the main menu and place your cursor in the bottom left (but not completely off) of your monitor
#### Notes
- Siege has a few errors which block interaction when prompted. Should such an error occur, you may need to manually close it. This is, however, exceptionally rare because `rustfarm` will take care of most but not all errors automatically.
### Apex Legends
*Under Construction*
### XDefiant
*Under Construction*

## Addendum
Your overall account KD will likely be negatively impacted by using this application, and you do risk a variety of account bans, for which I am not responsible. By using this program, you understand this risk.

Should you encounter issues, please report them directly to me on Discord or Telegram via `@hiibolt`.
