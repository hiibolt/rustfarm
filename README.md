# rustfarm

<img src="https://github.com/user-attachments/assets/cee018be-afc3-4937-9ff4-3f538f636840" width=250>

## About
An automatic **Renown**, **XP**, and **Battle Pass XP** farm for **Rainbow Six Siege**. 

This is a prototype and proof of concept. A hardened [Rust](https://www.rust-lang.org/) version is in the works, and will support a variety of games with a more robust template system.

Should you encounter issues, please report them directly to me on Discord or Telegram via `@hiibolt`.

## Setup
### Prerequisites:
- Install [Git for Windows](https://gitforwindows.org/)
- Install [Python 3.9 from the Windows Store](https://apps.microsoft.com/detail/9p7qfqmjrfp7?hl=en-us&gl=US)

### Download rustfarm:
- Open **Command Prompt**
- `> git clone https://github.com/hiibolt/rustfarm.git`
- `> cd rustfarm`
- `> python -m venv .venv`
- `> .venv\Scripts\activate.bat`
- `> pip install -r requirements.txt`

### Set up Siege
- Open Siege on an account eligible to play Casual, 2FA not required
- Set the following settings:
  - Resolution: 1920x1080
  - Display Mode: Borderless
  - Aspect Ratio: 4:3
  - HUD Display Area: 100
  - Menu Display Area: 100
- **IMPORTANT** - Navigate back to the main menu

### Launch `rustfarm`
- `> .venv\Scripts\activate.bat`
- `> python main.py`
- Quickly, tab back into Siege
- **IMPORTANT** - Place your cursor just above and to the left of the bottom right of your display, as it is very important your mouse does not hover over any buttons

## Notes
- `rustfarm` is a *robot* application, meaning it cannot be run in the background, as it forceably takes control of your mouse and keyboard
- Siege must be on the display marked as your "primary display" in your display settings
- Siege has many random errors that which are difficult to properly predict, as many block interaction when they prompted. Should an error occur, you will need to manually close it. `rustfarm` will automatically get back on track once it is closed. This is exceptionally rare because `rustfarm` will take care of most but not all errors automatically
- Your overall account KD will be negatively impacted by using this application, and you do risk a `Botting` account ban, for which I am not responsible. By using this program, you understand this risk.
