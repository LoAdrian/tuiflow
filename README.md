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
```yaml
app_title: example file explorer # the title of the app
controls: # the controls usable to control the flow
  selection_up: # a special reserved control to select the line above the current one
    name: selection up # the display name of the control
    key: !Char 'k' # the character key that will trigger the control
  selection_down: # a special reserved control to select the line below the current one
    name: selection down
    key: !Char 'j'
  quit: # a special reserved control to quit the app
    name: quit
    key: !Char 'q'
  custom_controls: # custom controls that can be used to trigger transitions between states
    moveback: # the name of the control
      name: move back # the display name of the control
      key: !Char 'h'
    moveinto:
      name: move into
      key: !Char 'l'
initial_command: ls -d -1 "$PWD/"** # the command that will be run to get the initial lines to display
initial_state: show_files # the state that will be shown first
states: # the states of the app
  show_files: # the name of the state
    transitions: # the transitions that can be triggered in this state
      - control_name: moveinto # the name of the control that will trigger this transition
        selection_filter: (?<x>.*) # the regex that will match the selected display-line (here the whole line)
        command_pattern: ls -d -1 "<x>/"** # the command that will be run when this transition is triggered (the <x> will be replaced by the matched group of the selection_filter)
        next_state: show_files # the target state of the transition
      - control_name: moveback
        selection_filter: (?<x>.*)\/.*\/.*
        command_pattern: ls -d -1 "<x>/"**
        next_state: show_files
    line_filter: (?<path>.+) # the regex for the line_display_pattern
    line_display_pattern: <path> # the pattern that will be used to display the lines (the <path> will be replaced by the matched group of the line_filter)
```
### Possible Keys:
- `!Char '<char>'`
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

## Planned Features
- [x] Basic regex state transitions  
- [ ] hide parts of a displayed line by tagging it in the regex or alternatively make the transitions receive the lines without the `line_display_pattern` mask.  
- [ ] allow multiple actions with the same key on the same transition but with different regex, choosing the first matching action  
- [ ] add an input-state-type that allows the user to input something instead of just selecting lines  
- [ ] add versions to the flow file and create a compatibility list  
- [ ] create a default search function to quickly select lines based on a keyboard input  
- [ ] allow for defaults if the command did not return any lines, either by using default values or alternative commands  
- [ ] allow for multiple panes or tabs but sensitive to values of other panes or tabs. 
- [ ] add an argument-input state that can be opted for as an initializer state that takes some argument and / or can be piped to.  
- [ ] add a prose state type that shows a single page of prose.  
- [ ] add a state type that is able to display interactive cli programs (like other tuiflows or vim or whatever) and that may or may not return to the initiating tuiflow after execution finishes.  
- [ ] create terminal states.  
- [ ] add autotransitioning states.
- [ ] create context-sensitive / selection-sensitive transitions.  
