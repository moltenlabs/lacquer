# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-14

### Added

- Initial release
- `Style` builder for terminal styling
- Box model: padding, margin
- Dimension constraints: width, height, max_width, max_height
- Border styles: None, Normal, Rounded, Thick, Double, Ascii, Block, Dashed
- Custom border character support
- Text colors: foreground, background
- Text modifiers: bold, italic, underline, strikethrough, dim, reverse
- Alignment: horizontal and vertical positioning
- Inline mode for non-block rendering
- `Position` enum for alignment
- `Border` and `BorderStyle` types
- Full integration with `sigil` for ANSI escape sequences
- Unicode width support via `unicode-width`
