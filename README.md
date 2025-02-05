# Purpose

I want to be be able to save my 'location by path' and be able to go to it easily, whether that is through a command (with completions argc maybe?)
Or a tui
will need to store persistence between uses, perhaps just saving to a configured file


### FUNCTIONING
- Compile binary and install to save-point in PATH
- add script like save-point.sh to respective PATH
  - This enables the parent shell to automatically get access to processes temp file/clean up
- Add to ~/.bashrc (or other config on boot etc) a function that souces save-point.sh to enable folder changes

