# Note - Terminal Sticky Notes!
Are you tired of dealing with physical sticky notes? Is your computer screen cluttered with sticky 
notes? Are you tired of having to pick up a pen to write your notes? Do you wish for a simpler and
easier way to quickly jot down simple notes that are always within eyesight? Note is here to help 
you solve all of those problems. 

Note is the terminal sticky note system. It makes it easy to jot down simple notes that will always
be within eyesight.

## Usage
The best way to make sure your notes are always within eyesight is to add `note ls -p` to your
shell's source file. That way, each time you open your terminal you will be greeted with all of your
notes.

Stick a note to your terminal
```
note stick <note>

// Specify a notes priority
note stick <note> --priority <priority>
```

List all of your notes
```
note list

// To list notes with their respective IDs
note list --show_id

// To list notes in order of priority
note list --priority
```

Toss a note to the trash
```
note toss <ids>...
```

Made a spelling mistake? Quickly edit your notes like so
```
note edit <id> <note>

// Edit a notes priority
note edit <id> --priority <priority>

// Edit a note and its' priority
note edit <id> <note> --priority <priority>
```

If you are ever unsure about what a command does or how its' syntax is simply run `note help` or
`note <command> --help` if you want to know more about a specific command.

## Customization
You can customize Note in various ways through environment variables
```
// Set message for note to greet you with
NOTE_MESSAGE

// Set color and style of the message
NOTE_MESSAGE_COLOR
NOTE_MESSAGE_STYLE

// Set message for note to greet you with when there are no notes
NOTE_EMPTY_MESSAGE

// Set the color and style of the 'empty' message
NOTE_EMPTY_MESSAGE_COLOR
NOTE_EMPTY_MESSAGE_STYLE

// Controls the color and style of a given priority (1, 2, 3, 4, 5)
NOTE_P1_COLOR
NOTE_P1_STYLE
```

Style can be any one of `bold`, `italic` or `underline`.
And color can be any one of:
+ `black`
+ `blue`
+ `cyan`
+ `green`
+ `magenta`
+ `red`
+ `magenta`
+ `red`
+ `white`
+ `yellow`
+ `bright black`
+ `bright blue`
+ `bright cyan`
+ `bright green`
+ `bright magenta`
+ `bright red`
+ `bright magenta`
+ `bright red`
+ `bright white`
+ `bright yellow`

## Aliases
+ `s` is an alias for `stick`
+ `ls` is an alias for `list`
+ `t` is an alias for `toss`
+ `e` is an alias for `edit`
