# Git Status All

Find subdirectories where untracked or uncommitted files exist.

The output will show the directory name along with the status using the same annotation as `$ git status --porcelain=v1` (see `man git-status`).

```bash
$ gitsa 
git-rcall {"": 1, "??": 2}
litho {"??": 1}
cult {"??": 1}
```

## Installation


## Usage

```bash
Checks for dirty Git repositories

Usage: gitsa [ROOT]

Arguments:
  [ROOT]  Optional root dir, otherwise uses current working dir

Options:
  -h, --help     Print help
  -V, --version  Print version
```
