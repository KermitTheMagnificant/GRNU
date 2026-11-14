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
### Immediate Planned Changes
1. cat
2. chmod/chown/chroot
3. cp
4. dd
5. mkdir
6. mv
7. yes
8. kill
9. whoami
10. rm
### Ambitious Planned Changes
(I wouldn't count on getting to this point, assuming this stays a personal project.)
1. nano
2. bash
3. GRUB
