# mouse-tracker
## Description
Sometimes it is necessary to step away from the computer, but leave the screen on. Personally, such situations make me panic: "What if someone comes up and does something naughty with my precious data". 
That's why I wrote this project, which tracks changes in the mouse position when you step away from your workplace.
## What it does
The programme works in the form of a daemon. At the moment when you need to step away, you press a special key combination that moves your cursor to a position known only to you in advance. In case someone moves the mouse from this position, the webcam will take 3 pictures of the intruder, send them to Telegram and shut down the computer. When you return to the computer, you will either see it switched off and realise that someone tried to access it, or press another key combination, thus deactivating the script.
## What it uses
The entire programme is written in Rust using third-party open-source libraries. The language was chosen for its power and ease of use with peripherals such as keyboard, mouse, webcam.
## Install
You can download the GitHub repository as standard. Now you need to build the project yourself.
```bash
cargo build
```
### Note
You will need to run the binary as sudo. Environment variables such as TELOXIDE_TOKEN and CHAT_ID are also required. These can be written in the shell configuration file or passed along with the daemon startup. 
```bash
sudo TELOXIDE_TOKEN=$TELOXIDE_TOKEN CHAT_ID=$CHAT_ID ./target/release/mouse_tracker
```
Where CHAT_ID is an ID of the chat where you added your bot.
