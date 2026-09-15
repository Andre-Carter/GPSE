                 YOUR MACHINE
                     │
              ┌──────▼──────┐
              │ git commit  │
              │  checkpoint │
              └──────┬──────┘
                     │
                     ▼
                  GITHUB
              ┌─────────────┐
              │ git push    │
              │ remote copy │
              └─────────────┘

git status
cargo test
git diff
git add .
git commit -m "Describe the change"
git push
git status

KNOWN GOOD BASELINE
        ↓
remove duplicate modules
        ↓
cargo test 🟢
        ↓
cargo fmt 🟢
        ↓
commit
        ↓
push
        ↓
KNOWN GOOD REMOTE BASELINE