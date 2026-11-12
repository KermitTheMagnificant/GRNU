<div align ="left">
  <img src="GRNU.svg" alt="GRNU Logo" width="250">
</div>

# GRNU
GNU Rust, Not Unix, a rewrite of GNU software in rust.

You might be saying "Isn't this just uutils?" and you would be right, except for this was made as a way for myself to learn 1. Rust 2. OS system development, so I don't really care (also I plan on adding more than just coreutils).
All coreutil commands will be the same as their GNU coreutil commands except with 'r' added to the beginning, for example, 'rls' instead of 'ls'.

## Current Features 
### rls 
- current tags: -a
- DESC: Lists file in current directory, currently the only util thus far.
### rpwd
- current tags: none
- DESC: Prints Working Directory. Not tested as I am currently on Windows and cargo isn't installing, will test when I get the chance.

## Planned Future Features
1. More tags for all existing commands
2. more coreutils commands such as cd, mv, cp, etc...
3. Non coreutils gnu packages, such as tar, sysutils, and maybe even GRUB! (don't get your hopes up for that one)
