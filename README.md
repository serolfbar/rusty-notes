# Rusty-Notes 
CLI tool to create notes.

## Why 
I made rusty-notes as both a tool I needed and to learn the basics of rust.

## Installing
Install <a href="https://www.rust-lang.org/en-US/install.html">Rust and Cargo</a> \
Download the source. `git clone https://github.com/serolfbar/rusty-notes.git`\
Go into the rusty-notes directory and `cargo install`
## Use
The use of rusty-notes is pretty simple.
```
Usage : rusty-notes [command] [args]
  add ["note"] : Adds a new note which is between quotes.
  list         : Prints all saved notes.
  remove [id]  : Removes the note assigned to id.
  help         : Prints help.
 
# Examples
rusty-notes add "Hello World"
rusty-notes remove 1
```
## Todo's
-   Add tests.
-   Refactor code.
-   Make better error messages.
-   Add dates to the notes.
## Maybe later
-   Synchronize the notes with Dropbox or something similar.
