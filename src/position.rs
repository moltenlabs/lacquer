//! Position and alignment definitions.

/// Horizontal and vertical positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// Align to the start (left/top).
    #[default]
    Start,
    /// Align to center.
    Center,
    /// Align to the end (right/bottom).
    End,
}

impl Position {
    /// Left alignment (alias for Start).
    pub const LEFT: Self = Self::Start;
    /// Right alignment (alias for End).
    pub const RIGHT: Self = Self::End;
    /// Top alignment (alias for Start).
    pub const TOP: Self = Self::Start;
    /// Bottom alignment (alias for End).
    pub const BOTTOM: Self = Self::End;

    /// Calculate offset for content of given size within container.
    pub fn offset(self, content_size: usize, container_size: usize) -> usize {
        if content_size >= container_size {
            return 0;
        }
        let space = container_size - content_size;
        match self {
            Position::Start => 0,
            Position::Center => space / 2,
            Position::End => space,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_offset() {
        // Content smaller than container
        assert_eq!(Position::Start.offset(5, 10), 0);
        assert_eq!(Position::Center.offset(5, 10), 2); // (10-5)/2 = 2
        assert_eq!(Position::End.offset(5, 10), 5);

        // Content equal to container
        assert_eq!(Position::Center.offset(10, 10), 0);

        // Content larger than container
        assert_eq!(Position::Center.offset(15, 10), 0);
    }

    #[test]
    fn test_position_aliases() {
        assert_eq!(Position::LEFT, Position::Start);
        assert_eq!(Position::RIGHT, Position::End);
        assert_eq!(Position::TOP, Position::Start);
        assert_eq!(Position::BOTTOM, Position::End);
    }
}
