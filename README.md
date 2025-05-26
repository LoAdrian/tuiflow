```
 _____  __  __  __  ____  __     ______  __    __
/_  _/ / / / / / / / __/ / /    / __  / / /_  / /
 / /  / /_/ / / / / __/ / /__  / /_/ / / // |/ /
/_/  /_____/ /_/ /_/   /____/ /_____/ /___/|__/
```

This program allows you to build list based, terminal-ui like workflows for your favorite cli tools.  
It's very much a work in progress and might not work on your machine.  
It also will contain bugs and weird behavior.  

## Basic usage:  
1. Create a tuiflow yaml file like in examples/dora-the-explorah.yaml
2. run `./tuiflow <your-file>.yaml`

## Basic concepts:
Basically tuiflow will create a statemachine according to the supplied file.
Every state can be thought of a workflow-step and every transition can be thought of a cli-command that will lead to the next state.
The displays always show some amount of lines where one line is always selected.  

Each transition  maps the selected line on the display to a cli-command. Every such *mapping* consists of a regex matching some input and an output pattern that uses the group-names of the regex.  
Each state also maps the results of the cli-command delimited by newlines to the new lines to be displayed in the same way as the transitions map lines to commands.

## YAML file structure:

### Possible Keys:
- `!Char <char>`
- `!Enter`,
- `!Backspace`,
- `!Tab`,
- `!Esc`,
- `!Up`,
- `!Down`,
- `!Left`,
- `!Right`,
- `!Home`,
- `!End`,
- `!PageUp`,
- `!PageDown`,
- `!Delete`,
- `!Insert`,
- `!F <u8>`,
