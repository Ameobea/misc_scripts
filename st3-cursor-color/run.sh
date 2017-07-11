#!/bin/sh

# Sets the color of the cursor in my current Sublime Text 3 theme file to a different color depending on the date.

cargo build && ./target/debug/st3-cursor-color "/Users/casey/Library/Application Support/Sublime Text 3/Packages/User/SublimeLinter/kiwi (SL).tmTheme"
