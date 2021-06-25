# dottie (WIP)

[![build](https://github.com/crispgm/dottie/actions/workflows/ci.yml/badge.svg)](https://github.com/crispgm/dottie/actions/workflows/ci.yml)

dottie is yet another `.dotfiles` manager written in Rust.

Notice: dottie is under ACTIVE DEVELOPMENT and not ready for production.

## Concept

dottie provides easy approach to import dotfiles anywhere to your dotfiles project,
and connects the dotfiles with symblic links, which is also commonly used in dotfiles management softwares.

dottie is dotfile-agnostic therefore it is kind of a symbolic linker.
This might be used in any scenarios that requests similar functions.

## Usage

### Installation

Install with Homebrew:
```bash
brew install crispgm/dottie/homebrew
```

Install with Cargo:
```bash
cargo install dottie
```

### Getting Started

```bash
# create your dotfiles project and cd to it
mkdir /path/to/your/dottie_project
cd /path/to/your/dottie_project

# init with dottie
dottie init

# add your dotfiles
dottie add ~/.zshrc --name zsh
# => zsh is added
dottie add ~/.config/nvim
# => nvim is added

# link the files
dottie link
```

### Commands

- [ ] init
- [ ] clone
- [x] ls
- [x] info
- [ ] add
- [ ] rm
- [ ] link
- [ ] unlink

## License

Copyright &copy; MIT License, 2021.
