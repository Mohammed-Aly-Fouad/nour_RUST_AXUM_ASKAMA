# Project Commands Cheatsheet

1. Cargo Watch (Development)

- Full project watch (ignores CSS changes to prevent unnecessary reloads):
cargo watch -c -q -i "static/css/main.css" -x "run --bin nour"

- Main source watch:
cargo watch -c -q -x "run --bin nour"

- Specific binary watch (replace main with your target binary name):
cargo watch -q -c -x "run --bin main"


2. Database Management (SQLx)

- Run pending database migrations:
cargo sqlx migrate run

- Prepare offline queries (generates/updates sqlx-data.json for compilation without a live database):
cargo sqlx prepare


3. Git Workflow

- Quick commit and push to GitHub:
git status
git add .
git commit -m "notes"
git push

- Hard reset and clean (discards all uncommitted changes and untracked files to match HEAD):
git reset --hard HEAD
git clean -fd