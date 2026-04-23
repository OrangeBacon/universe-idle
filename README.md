# universe-idle
Idle/Incremental game

# Build
```sh
cargo run
```
This is only currently tested on linux systems.  The build depends on typescript
being available in the path as `npx tsc`, `npm install` will get the right version
for you.

# Deploy
Ensure that the following is run once you have cloned this repository, before
building the project:
```sh
git worktree add deploy gh-pages
```

Then, to deploy, commit and push from inside the work tree.  Make sure you run the
build command first!
