<div align ="left">
  <img src="GRNU.svg" alt="GRNU Logo" width="250">
</div>

# GRNU
GNU Rust, Not Unix, a rewrite of GNU software in rust (not affiliated with GNU).

You might be saying "Isn't this just uutils?" and you would be right, except for this was made as a way for myself to learn 1. Rust 2. OS system development, so I don't really care (also I plan on adding more than just coreutils).
All coreutil commands will be the same as their GNU coreutil commands except with 'r' added to the beginning, for example, 'rls' instead of 'ls'.

## Installation
GRNU currently runs on Linux and other Unix-like systems only.

1. Install Rust if you don't have it: https://rustup.rs
2. If you have rustup's old `rls` (the retired Rust Language Server) in `~/.cargo/bin`, remove it, since it uses the same name:
   ```bash
   rm -f ~/.cargo/bin/rls
   ```
3. Install every rutils command at once:
   ```bash
   cargo install --git https://github.com/KermitTheMagnificant/GRNU.git rutils
   ```

To update to the latest version, run the same `cargo install` command again.

## Current Features 
### rls 
- current tags: -a, -l
- DESC: Lists files in a directory (the current one by default).
### rpwd
- current tags: none
- DESC: Prints Working Directory.

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
