# SubilmeText 3 Cursor Color Changer

This is a small Rust utility that automatically changes the color of the cursor color in a specified theme file based on the current season.  It's been tested only on my themes and is meant to be executed on a cron task.

The color is calculated by interpolating between a range of colors defined in the script itself and is designed to change gradually as the seasons change.  Green in the summer, orange in the fall, blue in the winter, magenta in the spring.  The color is changed by parsing the theme file's XML, finding the old caret color, and replacing it with the newly generated color.  This means that the old theme must define a caret color for this to work.

A backup is made of the theme before any modifications are made and it is saved to `themeName.tmTheme_backup`.
