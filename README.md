[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg?style=flat-square)](https://opensource.org/licenses/)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/aaylafetzer/RustCloneGitProfile?style=flat-square)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/aaylafetzer/RustCloneGitProfile?style=flat-square)

# RustCloneGitProfile
A program to clone all public repositories on a GitHub or GitLab profile.

This is a Rust rewrite of [PyCloneGitProfile](https://github.com/aaylafetzer/PyCloneGitProfile).
## Usage
This information can also be accessed with ``./clonegitprofile -h``
```
CloneGitProfile 1.0
Aayla Fetzer
Clones the public repositories of a Github or GitLab profile into a directory

USAGE:
    clonegitprofile [FLAGS] [OPTIONS] <PLATFORM> <USERNAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    increase the program verbosity

OPTIONS:
    -p, --path <OUTPUT>    path to clone repositories into

ARGS:
    <PLATFORM>    gitlab or github
    <USERNAME>    username of the account to clone
```
### Examples
To clone my GitHub page to ``./Code/``, run:
``./clonegitprofile -p ./Code github aaylafetzer``
