# universe-idle
Idle/Incremental game

# Build
```sh
cargo run
```
This is only currently tested on linux systems.

# Deploy
Ensure that the following is run once you have cloned this repository, before
building the project:
```sh
git worktree add deploy gh-pages
```

Then, to deploy, run `deploy.sh`.  (Will need to be made executable with `chmod`)
