# RSTOW - An stow alternative written in Rust

Rstow functions the same as the GNU program
[stow](https://www.gnu.org/software/stow/). It's main function is to manage
my *dotfiles* in the *.dotfiles* folder. Rstow creates symlinks to the proper
locations.

Rstow is written as an exercise in Rust. It is by no means intented as a serious
alternative to GNU Stow nor will I publish this program.

### Layout

My first goal is to keep it simple. I'll create a basic app that:
    - Enters the .dotfiles folder
    - Travels through each file
    - Create symlinks to each file in the correct location

From there on I'll implement other featurs such as:
    - create directories
    - ask to overwrite files
    - use custom directories

---
Created by Mark Broeders
27-09-2021
