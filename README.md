# devtimer

**devtimer** is a simple CLI tool written in Rust to track the time you spend coding on your projects. It lets you start, pause, resume, and stop a timer, while maintaining a cumulative total of your coding time in a JSON file.

## Features
- **Start Timer:** Begin tracking time with `devtimer start`
- **Pause Timer:** Pause the timer and display the elapsed time with `devtimer break`
- **Resume Timer:** Resume a paused timer with `devtimer back`
- **Stop Timer:** Stop the timer and add the current session's time to your cumulative total with `devtimer stop`
- **View Total Time:** Display the total time spent coding with `devtimer spent`

## Dependencies
This project uses the following Rust crates:
- **clap:** For parsing command-line arguments
- **serde** and **serde_json:** For JSOn serialization and deserialization
- **dirs:** For safe static initialization of global variables

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/stigsec/devtimer.git
   cd devtimer
   ```
2. **Build the Project:**

   ```bash
   cargo build --release
   ```
   The compiled binary will be located in ```target/release/devtimer```

## Usage

```bash
Track time spent coding on a project

Usage: devtimer [COMMAND]

Commands:
  start  Starts the timer
  break  Pauses the timer and shows current session time
  back   Resumes the timer
  stop   Stops the timer and adds session time
  spent  Shows total time spent coding
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuring the Timer Data File Location

By default, the timer data is stored in a JSON file. You can customize its location by setting the ```DEVTIMER_PATH``` environment variable.
For example, to store the timer data in ```~/.config/timer.json```, add the following line to your shell configuration file (e.g., ```~/.bashrc``` or ```~/.zshrc```):
```bash
export DEVTIMER_PATH="$HOME/.config/timer.json"
```
After updating your shell configuration, reload it with:
```bash
source ~/.bashrc
```
Alternatively, you can set the variable temporarily in your current terminal session:
```
export DEVTIMER_PATH="$HOME/.config/timer.json"
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE file](LICENSE) for more details.



---

Developed by [stigsec](https://github.com/stigsec).
