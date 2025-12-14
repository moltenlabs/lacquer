<h1 align="center">💅 Lacquer</h1>

<p align="center">
  <strong>Style definitions for gorgeous terminal layouts in Rust.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/lacquer"><img src="https://img.shields.io/crates/v/lacquer.svg?style=flat-square&logo=rust" alt="Crates.io"></a>
  <a href="https://docs.rs/lacquer"><img src="https://img.shields.io/docsrs/lacquer?style=flat-square&logo=docs.rs" alt="Documentation"></a>
  <a href="https://github.com/moltenlabs/lacquer/actions"><img src="https://img.shields.io/github/actions/workflow/status/moltenlabs/lacquer/ci.yml?style=flat-square&logo=github" alt="CI"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square" alt="License"></a>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#ecosystem">Ecosystem</a>
</p>

---

## What is Lacquer?

**Lacquer** is the Rust equivalent of [lipgloss](https://github.com/charmbracelet/lipgloss) from Charmbracelet. It provides declarative style definitions for building beautiful terminal layouts—with padding, margins, borders, colors, and alignment.

```rust
use lacquer::{Style, Border, Position, Color};

let style = Style::new()
    .padding(1, 2)
    .margin(1, 0)
    .border(Border::Rounded)
    .border_foreground(Color::from_hex("#7C3AED"))
    .foreground(Color::from_hex("#FAFAFA"))
    .bold()
    .align(Position::Center);

println!("{}", style.render("🔥 Molten Labs"));
```

```
╭────────────────────╮
│                    │
│   🔥 Molten Labs   │
│                    │
╰────────────────────╯
```

---

## Features

<table>
<tr>
<td width="50%">

### 📦 Box Model
- Padding (all sides or individual)
- Margins (all sides or individual)
- Width/height constraints
- Max width/height limits

</td>
<td width="50%">

### 🎨 Colors & Styling
- Foreground & background colors
- Full RGB/hex color support
- Bold, italic, underline, dim
- Strikethrough, reverse video

</td>
</tr>
<tr>
<td width="50%">

### 🔲 Borders
- 8 built-in styles (Rounded, Double, Thick...)
- Custom border characters
- Per-side border colors
- ASCII fallback option

</td>
<td width="50%">

### 📐 Layout & Alignment
- Horizontal alignment (left, center, right)
- Vertical alignment (top, center, bottom)
- Inline mode for no-wrap rendering
- Unicode-aware width calculation

</td>
</tr>
</table>

---

## Installation

```bash
cargo add lacquer
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
lacquer = "0.1"
```

---

## Quick Start

### Basic Styling

```rust
use lacquer::{Style, Color};

// Simple colored text
let styled = Style::new()
    .foreground(Color::from_hex("#F97316"))
    .bold()
    .render("Hello, Lacquer!");

println!("{}", styled);
```

### Box with Padding

```rust
use lacquer::{Style, Border};

let box_style = Style::new()
    .padding(1, 3)          // 1 vertical, 3 horizontal
    .border(Border::Rounded)
    .render("Content");

println!("{}", box_style);
// ╭─────────────╮
// │             │
// │   Content   │
// │             │
// ╰─────────────╯
```

### Centered Content

```rust
use lacquer::{Style, Border, Position};

let centered = Style::new()
    .width(30)
    .border(Border::Double)
    .align(Position::Center)
    .render("Centered!");

println!("{}", centered);
// ╔════════════════════════════╗
// ║          Centered!         ║
// ╚════════════════════════════╝
```

### Multiple Lines

```rust
use lacquer::{Style, Border, Color};

let content = "Line 1\nLine 2\nLine 3";

let multiline = Style::new()
    .padding(1, 2)
    .border(Border::Thick)
    .foreground(Color::Green)
    .render(content);

println!("{}", multiline);
```

---

## Border Styles

```rust
use lacquer::Border;

Border::None      // No border
Border::Normal    // ┌──┐ │ └──┘
Border::Rounded   // ╭──╮ │ ╰──╯
Border::Thick     // ┏━━┓ ┃ ┗━━┛
Border::Double    // ╔══╗ ║ ╚══╝
Border::Ascii     // +--+ | +--+
Border::Block     // ████ █ ████
Border::Dashed    // ┌╌╌┐ ╎ └╌╌┘
```

### Custom Borders

```rust
use lacquer::{Border, BorderStyle};

let custom = Border::Custom(BorderStyle::new(
    '╭', '─', '╮',  // top-left, top, top-right
    '│',            // right
    '╯', '─', '╰',  // bottom-right, bottom, bottom-left
    '│',            // left
));
```

---

## Inline Mode

For styling without block rendering (no borders, no padding):

```rust
use lacquer::{Style, Color};

let inline = Style::new()
    .foreground(Color::Red)
    .bold()
    .inline()
    .render("Error!");

println!("Status: {}", inline);
// Status: Error! (styled red and bold)
```

---

## Using with Molten Brand

Enable the `brand` feature for pre-defined colors:

```toml
[dependencies]
lacquer = { version = "0.1", features = ["brand"] }
```

```rust
use lacquer::{Style, Border};
use molten_brand::{colors, products};

let goblin_box = Style::new()
    .border(Border::Rounded)
    .border_foreground(products::lair::PRIMARY.into())
    .foreground(colors::text::PRIMARY.into())
    .render("Lair Terminal");
```

---

## Why "Lacquer"?

In the forge, **lacquer** is the hard, protective finish applied to metalwork—transforming raw iron into polished, beautiful artifacts. This library does the same for your terminal output. 💅

---

## Ecosystem

Lacquer is part of the **Molten Labs** open source ecosystem:

| Crate | Description | Status |
|-------|-------------|--------|
| **[molten_brand](https://crates.io/crates/molten_brand)** | Design tokens & colors | ✅ Published |
| **[glyphs](https://crates.io/crates/glyphs)** | ANSI escape sequences | ✅ Published |
| **[lacquer](https://crates.io/crates/lacquer)** | Terminal styling (you are here) | ✅ Published |
| **cauldron** | TUI framework (like bubbletea) | 📋 Planned |

---

## Documentation

- 📖 [API Documentation](https://docs.rs/lacquer)
- 💡 [Examples](https://github.com/moltenlabs/lacquer/tree/main/examples)
- 🎨 [Molten Brand Colors](https://github.com/moltenlabs/molten-brand)

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/moltenlabs/lacquer
cd lacquer
cargo test
```

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
