//! Style definitions for terminal rendering.

use crate::border::{Border, BorderColors};
use crate::position::Position;
use sigil::Color;
use unicode_width::UnicodeWidthStr;

/// A style definition for rendering terminal content.
///
/// Style uses a builder pattern for easy chaining:
///
/// ```rust
/// use lacquer::{Style, Border, Position, Color};
///
/// let style = Style::new()
///     .padding(1, 2)
///     .margin(0, 1)
///     .border(Border::Rounded)
///     .foreground(Color::from_hex("#F97316"))
///     .background(Color::from_hex("#0A0A0A"))
///     .bold()
///     .width(40)
///     .align(Position::Center);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Style {
    // Box model
    padding_top: u16,
    padding_right: u16,
    padding_bottom: u16,
    padding_left: u16,

    margin_top: u16,
    margin_right: u16,
    margin_bottom: u16,
    margin_left: u16,

    // Dimensions
    width: Option<u16>,
    height: Option<u16>,
    max_width: Option<u16>,
    max_height: Option<u16>,

    // Border
    border: Border,
    border_colors: BorderColors,

    // Colors
    foreground: Option<Color>,
    background: Option<Color>,

    // Text styling
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    dim: bool,
    reverse: bool,

    // Alignment
    horizontal_align: Position,
    vertical_align: Position,

    // Inline mode (no block rendering)
    inline: bool,
}

impl Style {
    /// Create a new empty style.
    pub const fn new() -> Self {
        Self {
            padding_top: 0,
            padding_right: 0,
            padding_bottom: 0,
            padding_left: 0,
            margin_top: 0,
            margin_right: 0,
            margin_bottom: 0,
            margin_left: 0,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            border: Border::None,
            border_colors: BorderColors {
                top: None,
                right: None,
                bottom: None,
                left: None,
            },
            foreground: None,
            background: None,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            dim: false,
            reverse: false,
            horizontal_align: Position::Start,
            vertical_align: Position::Start,
            inline: false,
        }
    }

    // ─────────────────────────────────────────────────────────────
    // Padding
    // ─────────────────────────────────────────────────────────────

    /// Set uniform padding on all sides.
    pub const fn padding_all(mut self, padding: u16) -> Self {
        self.padding_top = padding;
        self.padding_right = padding;
        self.padding_bottom = padding;
        self.padding_left = padding;
        self
    }

    /// Set vertical and horizontal padding.
    pub const fn padding(mut self, vertical: u16, horizontal: u16) -> Self {
        self.padding_top = vertical;
        self.padding_bottom = vertical;
        self.padding_left = horizontal;
        self.padding_right = horizontal;
        self
    }

    /// Set padding for each side individually.
    pub const fn padding_sides(
        mut self,
        top: u16,
        right: u16,
        bottom: u16,
        left: u16,
    ) -> Self {
        self.padding_top = top;
        self.padding_right = right;
        self.padding_bottom = bottom;
        self.padding_left = left;
        self
    }

    /// Set top padding.
    pub const fn padding_top(mut self, padding: u16) -> Self {
        self.padding_top = padding;
        self
    }

    /// Set right padding.
    pub const fn padding_right(mut self, padding: u16) -> Self {
        self.padding_right = padding;
        self
    }

    /// Set bottom padding.
    pub const fn padding_bottom(mut self, padding: u16) -> Self {
        self.padding_bottom = padding;
        self
    }

    /// Set left padding.
    pub const fn padding_left(mut self, padding: u16) -> Self {
        self.padding_left = padding;
        self
    }

    // ─────────────────────────────────────────────────────────────
    // Margin
    // ─────────────────────────────────────────────────────────────

    /// Set uniform margin on all sides.
    pub const fn margin_all(mut self, margin: u16) -> Self {
        self.margin_top = margin;
        self.margin_right = margin;
        self.margin_bottom = margin;
        self.margin_left = margin;
        self
    }

    /// Set vertical and horizontal margin.
    pub const fn margin(mut self, vertical: u16, horizontal: u16) -> Self {
        self.margin_top = vertical;
        self.margin_bottom = vertical;
        self.margin_left = horizontal;
        self.margin_right = horizontal;
        self
    }

    /// Set margin for each side individually.
    pub const fn margin_sides(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        self.margin_top = top;
        self.margin_right = right;
        self.margin_bottom = bottom;
        self.margin_left = left;
        self
    }

    // ─────────────────────────────────────────────────────────────
    // Dimensions
    // ─────────────────────────────────────────────────────────────

    /// Set fixed width.
    pub const fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height.
    pub const fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Set maximum width.
    pub const fn max_width(mut self, max: u16) -> Self {
        self.max_width = Some(max);
        self
    }

    /// Set maximum height.
    pub const fn max_height(mut self, max: u16) -> Self {
        self.max_height = Some(max);
        self
    }

    // ─────────────────────────────────────────────────────────────
    // Border
    // ─────────────────────────────────────────────────────────────

    /// Set border style.
    pub const fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Set uniform border color.
    pub fn border_foreground(mut self, color: Color) -> Self {
        self.border_colors = BorderColors::all(color);
        self
    }

    /// Set border color from a hex string.
    pub fn border_foreground_hex(self, hex: &str) -> Self {
        self.border_foreground(Color::from_hex(hex))
    }

    // ─────────────────────────────────────────────────────────────
    // Colors
    // ─────────────────────────────────────────────────────────────

    /// Set foreground (text) color.
    ///
    /// Accepts a `Color`, hex string, or RGB values.
    ///
    /// ```rust
    /// use lacquer::{Style, Color};
    ///
    /// let s1 = Style::new().foreground(Color::Red);
    /// let s2 = Style::new().foreground(Color::from_hex("#F97316"));
    /// let s3 = Style::new().foreground(Color::rgb(249, 115, 22));
    /// ```
    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Set foreground color from a hex string.
    pub fn foreground_hex(self, hex: &str) -> Self {
        self.foreground(Color::from_hex(hex))
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set background color from a hex string.
    pub fn background_hex(self, hex: &str) -> Self {
        self.background(Color::from_hex(hex))
    }

    // ─────────────────────────────────────────────────────────────
    // Text Styling
    // ─────────────────────────────────────────────────────────────

    /// Enable bold text.
    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enable italic text.
    pub const fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Enable underlined text.
    pub const fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Enable strikethrough text.
    pub const fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Enable dim/faint text.
    pub const fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Enable reverse video (swap fg/bg).
    pub const fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    // ─────────────────────────────────────────────────────────────
    // Alignment
    // ─────────────────────────────────────────────────────────────

    /// Set horizontal alignment.
    pub const fn align(mut self, position: Position) -> Self {
        self.horizontal_align = position;
        self
    }

    /// Set vertical alignment.
    pub const fn vertical_align(mut self, position: Position) -> Self {
        self.vertical_align = position;
        self
    }

    // ─────────────────────────────────────────────────────────────
    // Mode
    // ─────────────────────────────────────────────────────────────

    /// Enable inline mode (no block wrapping).
    pub const fn inline(mut self) -> Self {
        self.inline = true;
        self
    }

    // ─────────────────────────────────────────────────────────────
    // Rendering
    // ─────────────────────────────────────────────────────────────

    /// Render content with this style applied.
    pub fn render(&self, content: &str) -> String {
        if self.inline {
            return self.render_inline(content);
        }

        let lines: Vec<&str> = content.lines().collect();
        let content_width = self.calculate_content_width(&lines);
        let content_height = lines.len();

        // Calculate total dimensions
        let inner_width = content_width + self.padding_left as usize + self.padding_right as usize;
        let inner_height =
            content_height + self.padding_top as usize + self.padding_bottom as usize;

        let has_border = !matches!(self.border, Border::None);
        let border_width = if has_border { 2 } else { 0 };

        let _total_width = inner_width + border_width;
        let _total_height = inner_height + border_width;

        // Build output
        let mut output = String::new();

        // Top margin
        for _ in 0..self.margin_top {
            output.push('\n');
        }

        let left_margin = " ".repeat(self.margin_left as usize);
        let border_style = self.border.style();

        // Top border
        if has_border {
            output.push_str(&left_margin);
            output.push(border_style.top_left);
            output.push_str(&border_style.top.to_string().repeat(inner_width));
            output.push(border_style.top_right);
            output.push('\n');
        }

        // Content rows
        let total_content_rows = inner_height;
        let vertical_offset = self
            .vertical_align
            .offset(content_height, total_content_rows);

        for row in 0..total_content_rows {
            output.push_str(&left_margin);

            // Left border
            if has_border {
                output.push(border_style.left);
            }

            // Row content
            let content_row = row
                .checked_sub(self.padding_top as usize)
                .and_then(|r| {
                    if r >= vertical_offset && r < vertical_offset + content_height {
                        Some(r - vertical_offset)
                    } else {
                        None
                    }
                })
                .and_then(|r| lines.get(r).copied());

            let row_str = self.render_row(content_row, inner_width);
            output.push_str(&row_str);

            // Right border
            if has_border {
                output.push(border_style.right);
            }

            output.push('\n');
        }

        // Bottom border
        if has_border {
            output.push_str(&left_margin);
            output.push(border_style.bottom_left);
            output.push_str(&border_style.bottom.to_string().repeat(inner_width));
            output.push(border_style.bottom_right);
            output.push('\n');
        }

        // Bottom margin
        for _ in 0..self.margin_bottom {
            output.push('\n');
        }

        // Trim trailing newline if present
        if output.ends_with('\n') {
            output.pop();
        }

        output
    }

    fn render_inline(&self, content: &str) -> String {
        let mut styled = sigil::style(content);

        if let Some(fg) = &self.foreground {
            styled = styled.fg(*fg);
        }
        if let Some(bg) = &self.background {
            styled = styled.bg(*bg);
        }
        if self.bold {
            styled = styled.bold();
        }
        if self.italic {
            styled = styled.italic();
        }
        if self.underline {
            styled = styled.underline();
        }
        if self.strikethrough {
            styled = styled.strikethrough();
        }
        if self.dim {
            styled = styled.dim();
        }
        if self.reverse {
            styled = styled.reverse();
        }

        styled.to_string()
    }

    fn render_row(&self, content: Option<&str>, width: usize) -> String {
        let text = content.unwrap_or("");
        let text_width = UnicodeWidthStr::width(text);

        let available_width = width
            .saturating_sub(self.padding_left as usize)
            .saturating_sub(self.padding_right as usize);

        let horizontal_offset = self.horizontal_align.offset(text_width, available_width);

        let mut row = String::new();

        // Left padding
        row.push_str(&" ".repeat(self.padding_left as usize));

        // Left alignment space
        row.push_str(&" ".repeat(horizontal_offset));

        // Apply text styling
        if content.is_some() {
            let mut styled = sigil::style(text);

            if let Some(fg) = &self.foreground {
                styled = styled.fg(*fg);
            }
            if let Some(bg) = &self.background {
                styled = styled.bg(*bg);
            }
            if self.bold {
                styled = styled.bold();
            }
            if self.italic {
                styled = styled.italic();
            }
            if self.underline {
                styled = styled.underline();
            }
            if self.strikethrough {
                styled = styled.strikethrough();
            }

            row.push_str(&styled.to_string());
        }

        // Right alignment space
        let used = horizontal_offset + text_width;
        let remaining = available_width.saturating_sub(used);
        row.push_str(&" ".repeat(remaining));

        // Right padding
        row.push_str(&" ".repeat(self.padding_right as usize));

        row
    }

    fn calculate_content_width(&self, lines: &[&str]) -> usize {
        let max_line_width = lines
            .iter()
            .map(|line| UnicodeWidthStr::width(*line))
            .max()
            .unwrap_or(0);

        if let Some(width) = self.width {
            let border_width = if matches!(self.border, Border::None) {
                0
            } else {
                2
            };
            let padding_width = self.padding_left as usize + self.padding_right as usize;
            (width as usize)
                .saturating_sub(border_width)
                .saturating_sub(padding_width)
        } else if let Some(max_width) = self.max_width {
            max_line_width.min(max_width as usize)
        } else {
            max_line_width
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_render() {
        let style = Style::new();
        let output = style.render("Hello");
        assert_eq!(output, "Hello");
    }

    #[test]
    fn test_padding() {
        let style = Style::new().padding(1, 2);
        let output = style.render("Hi");
        let lines: Vec<&str> = output.lines().collect();

        assert_eq!(lines.len(), 3); // 1 top + content + 1 bottom
        assert!(lines[1].contains("Hi"));
    }

    #[test]
    fn test_border_rounded() {
        let style = Style::new().border(Border::Rounded);
        let output = style.render("X");

        assert!(output.contains('╭'));
        assert!(output.contains('╰'));
        assert!(output.contains('│'));
    }

    #[test]
    fn test_border_double() {
        let style = Style::new().border(Border::Double);
        let output = style.render("X");

        assert!(output.contains('╔'));
        assert!(output.contains('╚'));
        assert!(output.contains('║'));
    }

    #[test]
    fn test_inline_mode() {
        let style = Style::new().inline().bold();
        let output = style.render("Bold");

        // Should contain ANSI codes but no newlines/borders
        assert!(output.contains("\x1b["));
        assert!(!output.contains('\n'));
    }

    #[test]
    fn test_alignment_center() {
        let style = Style::new().width(20).align(Position::Center);
        let output = style.render("Hi");

        // "Hi" is 2 chars, width is 20, so ~9 spaces on each side
        let trimmed = output.trim();
        assert_eq!(trimmed, "Hi");
    }

    #[test]
    fn test_multiline() {
        let style = Style::new().border(Border::Normal);
        let output = style.render("Line 1\nLine 2");

        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 4); // top border + 2 lines + bottom border
    }
}
