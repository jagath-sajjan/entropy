# entropy

> a text editor where the file deletes itself

you type. it deletes. you lose.

```
entropy eats your buffer one character at a time.
type faster than the void or watch your words disappear.
when the buffer hits zero game over.
```

![entropy demo]()

---

## install

### cargo (recommended)
```bash
cargo install entropy
```

### homebrew (mac/linux)
coming soon

### download binary

grab the latest release for your platform from [releases](https://github.com/jagath-sajjan/entropy/releases):

| platform | file |
|---|---|
| macOS (Apple Silicon) | `entropy-aarch64-apple-darwin` |
| macOS (Intel) | `entropy-x86_64-apple-darwin` |
| Linux (x86_64) | `entropy-x86_64-unknown-linux-gnu` |
| Windows | `entropy-x86_64-pc-windows-msvc.exe` |

then make it executable (mac/linux):
```bash
chmod +x entropy-*
mv entropy-* /usr/local/bin/entropy
```

---

## usage

```bash
entropy                  # open with empty buffer
entropy file.txt         # open a file (saves with ctrl+s)
entropy .                # browse current directory
entropy src/main.rs      # open a specific file
```

---

## controls

| key | action |
|---|---|
| type | add characters (raises your score) |
| `backspace` | delete character before cursor |
| `ctrl+s` | save to file (when opened with a filename) |
| `ctrl+c` | quit |

---

## how it works

entropy runs a background thread that deletes a random character from your buffer every few seconds. the deletion rate **accelerates** over time what starts as one delete every 3 seconds ramps up to one every 800ms.

the status bar shows:

- **chars** — current characters in buffer
- **deleted** — total characters consumed by the void
- **score** — total characters you've typed
- **▓▓▓** — danger level (fills red as deletion rate increases)

random glyphs flicker across your text. horror messages appear in the status bar, the border turns red, entropy wins eventually the question is how long you last.

---

## build from source

```bash
git clone https://github.com/jagath-sajjan/entropy.git
cd entropy
cargo build --release
./target/release/entropy
```

requires rust 1.70+

---

## cross compile

```bash
# install cross
cargo install cross

# mac apple silicon
cargo build --release --target aarch64-apple-darwin

# mac intel
cargo build --release --target x86_64-apple-darwin

# linux
cross build --release --target x86_64-unknown-linux-gnu

# windows
cross build --release --target x86_64-pc-windows-msvc
```

---

## project structure

```
src/
  main.rs        — entry point, terminal setup/teardown
  app.rs         — app state and logic
  ui.rs          — ratatui rendering
  events.rs      — keyboard input handling
  deletor.rs     — background delete thread (the villain)
  args.rs        — cli argument parsing
  filepicker.rs  — interactive directory browser
```

---

## why

because your work disappears anyway.
to crashes, to git resets, to `rm -rf`.
entropy just makes it honest.

---

built with [ratatui](https://ratatui.rs) · written in rust · made by [jagath-sajjan](https://github.com/jagath-sajjan)
