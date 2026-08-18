# Git & Rust Cheat Sheet

## Git — Setup
```
git init                          # start a new repo
git clone <url>                   # copy a remote repo
git config user.name "Name"
git config user.email "you@mail"
```

## Git — Daily Flow
```
git status                        # what changed
git add <file>                    # stage a file
git add .                         # stage everything
git commit -m "message"           # commit staged changes
git commit -am "message"          # stage tracked files + commit
git diff                          # unstaged changes
git diff --staged                 # staged changes
git log --oneline --graph         # compact history
```

## Git — Branching
```
git branch                        # list branches
git branch <name>                 # create branch
git checkout <name>                # switch branch
git checkout -b <name>             # create + switch
git switch <name>                  # switch (newer syntax)
git switch -c <name>                # create + switch (newer)
git merge <branch>                 # merge into current
git rebase <branch>                 # rebase current onto branch
git branch -d <name>                # delete branch (safe)
git branch -D <name>                # delete branch (force)
```

## Git — Remote
```
git remote -v                     # list remotes
git remote add origin <url>
git push -u origin <branch>       # push + set upstream
git push
git pull
git fetch
```

## Git — Undo / Fix
```
git restore <file>                # discard local changes
git restore --staged <file>       # unstage
git reset --soft HEAD~1           # undo last commit, keep changes staged
git reset --hard HEAD~1           # undo last commit, discard changes
git revert <commit>                # new commit that undoes one
git stash                          # shelve changes
git stash pop                      # reapply shelved changes
git commit --amend -m "new msg"    # edit last commit
```

## Git — Inspect
```
git log -p <file>                 # history of a file with diffs
git blame <file>                  # who changed each line
git show <commit>                 # show a specific commit
```
## Hard reset and clean (discards all uncommitted changes and untracked files to match HEAD):
git reset --hard HEAD
git clean -fd
---

## Rust — Cargo Project
```
cargo new <name>                  # new binary project
cargo new <name> --lib            # new library project
cargo init                        # init in existing dir
cargo build                       # compile (debug)
cargo build --release             # compile (optimized)
cargo run                         # build + run
cargo run --release
cargo check                       # fast type-check, no binary
```

## Rust — Testing & Quality
```
cargo test                        # run tests
cargo test <name>                 # run tests matching name
cargo fmt                         # auto-format code
cargo clippy                      # lint
cargo doc --open                  # build + view docs
```

## Rust — Dependencies
```
cargo add <crate>                 # add dependency
cargo add <crate>@<version>
cargo remove <crate>
cargo update                      # update Cargo.lock
cargo tree                        # dependency tree
```

## Rust — Misc
```
cargo clean                       # remove build artifacts
cargo publish                     # publish to crates.io
rustc --version                   # compiler version
rustup update                     # update toolchain
rustup component add rustfmt clippy
```
## Cargo Watch
```
cargo watch -c -q -i "static/css/main.css" -x "run --bin nour"
cargo watch -c -q -x "run --bin nour"
cargo watch -q -c -x "run --bin main"
