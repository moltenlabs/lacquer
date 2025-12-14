<p align="center">
  <img src="https://raw.githubusercontent.com/moltenlabs/lacquer/main/.github/assets/banner.png" alt="Lacquer" width="100%" />
</p>

<h1 align="center">💅 Lacquer</h1>

<p align="center">
  <strong>Style definitions for gorgeous terminal layouts.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/lacquer"><img src="https://img.shields.io/crates/v/lacquer.svg?style=flat-square&logo=rust" alt="Crates.io"></a>
  <a href="https://docs.rs/lacquer"><img src="https://img.shields.io/docsrs/lacquer?style=flat-square&logo=docs.rs" alt="Documentation"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square" alt="License"></a>
  <img src="https://img.shields.io/badge/status-coming%20soon-orange?style=flat-square" alt="Status">
</p>

<p align="center">
  <a href="#preview">Preview</a> •
  <a href="#features">Features</a> •
  <a href="#ecosystem">Ecosystem</a>
</p>

---

## 🚧 Coming Soon

**Lacquer** is the Rust equivalent of [lipgloss](https://github.com/charmbracelet/lipgloss) from Charmbracelet. It provides style definitions for building beautiful terminal layouts.

### Preview

```rust
use lacquer::{Style, Position};

// Create a styled box
let style = Style::new()
    .padding(1, 2)
    .margin(1)
    .border(Border::Rounded)
    .border_foreground(Color::from_hex("#7C3AED"))
    .foreground(Color::from_hex("#FAFAFA"))
    .background(Color::from_hex("#0F0F1A"))
    .bold()
    .align(Position::Center);

let output = style.render("🔥 Molten Labs");
println!("{}", output);
```

```
╭────────────────────────╮
│                        │
│     🔥 Molten Labs     │
│                        │
╰────────────────────────╯
```

---

## Features (Planned)

<table>
<tr>
<td width="50%">

### 📦 Box Model
- Padding & margin
- Width & height constraints
- Max/min dimensions

</td>
<td width="50%">

### 🎨 Colors & Styling
- Foreground & background
- Bold, italic, underline
- Inline & block rendering

</td>
</tr>
<tr>
<td width="50%">

### 🔲 Borders
- Multiple border styles
- Per-side customization
- Custom border characters

</td>
<td width="50%">

### 📐 Layout
- Horizontal & vertical joining
- Alignment (left, center, right)
- Table layouts

</td>
</tr>
</table>

---

## Why "Lacquer"?

**Lacquer** is a hard, protective finish applied to surfaces—like the industrial coating in a forge. It transforms raw materials into polished, beautiful outputs. Just like how this library transforms plain terminal output into gorgeous UIs. 💅

---

## Ecosystem

Lacquer is part of the **Molten Labs** open source ecosystem:

| Crate | Description | Status |
|-------|-------------|--------|
| **[molten-brand](https://github.com/moltenlabs/molten-brand)** | Design tokens & colors | ✅ Released |
| **[sigil](https://github.com/moltenlabs/sigil)** | ANSI escape sequences | ✅ Released |
| **[lacquer](https://github.com/moltenlabs/lacquer)** | Terminal styling (you are here) | 🚧 Coming Soon |
| **[cauldron](https://github.com/moltenlabs/cauldron)** | TUI framework (like bubbletea) | 📋 Planned |
| **[rune](https://github.com/moltenlabs/rune)** | Shell script tools (like gum) | 📋 Planned |
| **[ember](https://github.com/moltenlabs/ember)** | Markdown renderer (like glow) | 📋 Planned |

---

## Star & Watch

⭐ **Star this repo** to get notified when Lacquer is released!

👁️ **Watch releases** to be the first to know.

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

<p align="center">
  <sub>Built with 💅 by <a href="https://github.com/moltenlabs">Molten Labs</a></sub>
</p>

<p align="center">
  <sub><i>"Let them cook."</i></sub>
</p>
