Terminal Countdown Timer (Rust implementation)
===

Forked from https://github.com/zenito9970/countdown-rs
This one does not use fancy ASCII characters to draw the remaining time.

Usage
---

Specify duration in go format `1h2m3s` .

```
countdown-rs 25s
```

Add command with `&&` to run after countdown.

```
countdown-rs 1m30s && say "Hello, world"
```

Press `Esc` or `Ctrl+C` to stop countdown without running next command.

Install
---

```
git clone https://github.com/savchenko/countdown
cd countdown
cargo install --path .
```

License
---

MIT
