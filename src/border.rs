//! Border definitions and styles.

use glyphs::Color;

/// Border style preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Border {
    /// No border.
    #[default]
    None,
    /// Normal box drawing characters.
    /// ```text
    /// ┌───┐
    /// │   │
    /// └───┘
    /// ```
    Normal,
    /// Rounded corners.
    /// ```text
    /// ╭───╮
    /// │   │
    /// ╰───╯
    /// ```
    Rounded,
    /// Bold/thick border.
    /// ```text
    /// ┏━━━┓
    /// ┃   ┃
    /// ┗━━━┛
    /// ```
    Thick,
    /// Double line border.
    /// ```text
    /// ╔═══╗
    /// ║   ║
    /// ╚═══╝
    /// ```
    Double,
    /// Simple ASCII border.
    /// ```text
    /// +---+
    /// |   |
    /// +---+
    /// ```
    Ascii,
    /// Block characters for a solid look.
    /// ```text
    /// ████
    /// █  █
    /// ████
    /// ```
    Block,
    /// Dashed border.
    /// ```text
    /// ┌╌╌╌┐
    /// ╎   ╎
    /// └╌╌╌┘
    /// ```
    Dashed,
    /// Custom border characters.
    Custom(BorderStyle),
}

impl Border {
    /// Get the border style characters.
    pub const fn style(self) -> BorderStyle {
        match self {
            Border::None => BorderStyle::NONE,
            Border::Normal => BorderStyle::NORMAL,
            Border::Rounded => BorderStyle::ROUNDED,
            Border::Thick => BorderStyle::THICK,
            Border::Double => BorderStyle::DOUBLE,
            Border::Ascii => BorderStyle::ASCII,
            Border::Block => BorderStyle::BLOCK,
            Border::Dashed => BorderStyle::DASHED,
            Border::Custom(style) => style,
        }
    }
}

/// Custom border characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderStyle {
    /// Top-left corner.
    pub top_left: char,
    /// Top edge.
    pub top: char,
    /// Top-right corner.
    pub top_right: char,
    /// Right edge.
    pub right: char,
    /// Bottom-right corner.
    pub bottom_right: char,
    /// Bottom edge.
    pub bottom: char,
    /// Bottom-left corner.
    pub bottom_left: char,
    /// Left edge.
    pub left: char,
}

impl BorderStyle {
    /// No border (empty characters).
    pub const NONE: Self = Self {
        top_left: ' ',
        top: ' ',
        top_right: ' ',
        right: ' ',
        bottom_right: ' ',
        bottom: ' ',
        bottom_left: ' ',
        left: ' ',
    };

    /// Normal box drawing border.
    pub const NORMAL: Self = Self {
        top_left: '┌',
        top: '─',
        top_right: '┐',
        right: '│',
        bottom_right: '┘',
        bottom: '─',
        bottom_left: '└',
        left: '│',
    };

    /// Rounded corner border.
    pub const ROUNDED: Self = Self {
        top_left: '╭',
        top: '─',
        top_right: '╮',
        right: '│',
        bottom_right: '╯',
        bottom: '─',
        bottom_left: '╰',
        left: '│',
    };

    /// Thick/bold border.
    pub const THICK: Self = Self {
        top_left: '┏',
        top: '━',
        top_right: '┓',
        right: '┃',
        bottom_right: '┛',
        bottom: '━',
        bottom_left: '┗',
        left: '┃',
    };

    /// Double line border.
    pub const DOUBLE: Self = Self {
        top_left: '╔',
        top: '═',
        top_right: '╗',
        right: '║',
        bottom_right: '╝',
        bottom: '═',
        bottom_left: '╚',
        left: '║',
    };

    /// ASCII border.
    pub const ASCII: Self = Self {
        top_left: '+',
        top: '-',
        top_right: '+',
        right: '|',
        bottom_right: '+',
        bottom: '-',
        bottom_left: '+',
        left: '|',
    };

    /// Block border.
    pub const BLOCK: Self = Self {
        top_left: '█',
        top: '█',
        top_right: '█',
        right: '█',
        bottom_right: '█',
        bottom: '█',
        bottom_left: '█',
        left: '█',
    };

    /// Dashed border.
    pub const DASHED: Self = Self {
        top_left: '┌',
        top: '╌',
        top_right: '┐',
        right: '╎',
        bottom_right: '┘',
        bottom: '╌',
        bottom_left: '└',
        left: '╎',
    };

    /// Create a custom border style.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        top_left: char,
        top: char,
        top_right: char,
        right: char,
        bottom_right: char,
        bottom: char,
        bottom_left: char,
        left: char,
    ) -> Self {
        Self {
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
            bottom_left,
            left,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::NONE
    }
}

/// Border colors for each side.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct BorderColors {
    /// Top border color.
    pub top: Option<Color>,
    /// Right border color.
    pub right: Option<Color>,
    /// Bottom border color.
    pub bottom: Option<Color>,
    /// Left border color.
    pub left: Option<Color>,
}

impl BorderColors {
    /// Create uniform border colors.
    pub fn all(color: Color) -> Self {
        Self {
            top: Some(color),
            right: Some(color),
            bottom: Some(color),
            left: Some(color),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_styles() {
        assert_eq!(Border::Rounded.style().top_left, '╭');
        assert_eq!(Border::Normal.style().top_left, '┌');
        assert_eq!(Border::Double.style().top_left, '╔');
        assert_eq!(Border::Ascii.style().top_left, '+');
    }

    #[test]
    fn test_custom_border() {
        let custom = BorderStyle::new('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H');
        assert_eq!(custom.top_left, 'A');
        assert_eq!(custom.bottom_right, 'E');
    }
}
