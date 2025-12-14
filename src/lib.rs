//! # Lacquer
//!
//! Style definitions for gorgeous terminal layouts.
//!
//! Lacquer is the Rust equivalent of [lipgloss](https://github.com/charmbracelet/lipgloss)
//! from Charmbracelet. It provides a declarative way to style and layout terminal output.
//!
//! ## Quick Start
//!
//! ```rust
//! use lacquer::{Style, Border, Position, Color};
//!
//! let style = Style::new()
//!     .padding(1, 2)
//!     .border(Border::Rounded)
//!     .foreground(Color::from_hex("#F97316"))
//!     .bold();
//!
//! println!("{}", style.render("Hello, Lacquer!"));
//! ```
//!
//! ## Features
//!
//! - **Box Model**: Padding, margin, width, height constraints
//! - **Borders**: Multiple styles with per-side customization
//! - **Colors**: Full RGB/Hex support via sigil
//! - **Layout**: Horizontal/vertical joining, alignment
//! - **Brand Integration**: Optional molten_brand color support

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

mod border;
mod position;
mod style;

pub use border::{Border, BorderStyle};
pub use position::Position;
pub use style::Style;

// Re-export sigil's Color for convenience
pub use molten_sigil::Color;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{Border, BorderStyle, Color, Position, Style};
}
