//! Per-line decoration provider API.
//!
//! Consumers of the [`Input`](crate::input::Input) element in
//! [`CodeEditor`](crate::input::InputMode::CodeEditor) mode can attach a
//! [`LineDecorationProvider`] via
//! [`InputState::set_line_decoration_provider`](crate::input::InputState::set_line_decoration_provider)
//! to render full-line backgrounds and gutter glyphs alongside the
//! built-in syntax highlighter and diagnostic underlines.
//!
//! Decorations are queried per frame: the renderer asks the provider
//! which decorations apply to the current visible-row range, and paints
//! tints + glyphs accordingly. There is no caching or dispatch table —
//! the provider is responsible for filtering its data to the requested
//! range cheaply.

use std::ops::Range;

use gpui::{App, Hsla, SharedString};

use crate::IconName;

/// Source of per-line decorations for a [`CodeEditor`](crate::input::InputMode::CodeEditor)
/// input. Implementations must be cheap to call once per frame: the
/// renderer queries them with the current visible-row range and uses
/// the returned items to paint line tints and gutter glyphs.
pub trait LineDecorationProvider: Send + Sync {
    /// Return the decorations that fall in `visible_rows`. The renderer
    /// will discard items whose `line` is outside the range, but
    /// implementations should pre-filter when possible.
    fn decorations_for(&self, visible_rows: Range<u32>, cx: &App) -> Vec<LineDecorationItem>;
}

/// One per-line decoration entry. `line` is zero-based against the
/// underlying buffer.
///
/// `Debug` is intentionally not derived: [`LineDecorationGlyph`]
/// carries an [`IconName`] which only derives `Clone` upstream. Derive
/// it manually if needed.
#[derive(Clone)]
pub struct LineDecorationItem {
    /// Buffer line, zero-based.
    pub line: u32,
    /// Optional gutter glyph, painted in the gutter region.
    pub glyph: Option<LineDecorationGlyph>,
    /// Optional full-line background tint, painted under the line text.
    pub line_tint: Option<Hsla>,
    /// Optional tooltip text. Reserved for a future hover integration —
    /// not painted yet.
    pub tooltip: Option<SharedString>,
}

/// Glyph kind painted in the gutter beside a line. The fixed variants
/// exist for consumers that want canonical shapes the renderer can
/// pick optimised paints for; [`Self::Custom`] is the open hatch for
/// everything else.
#[derive(Clone)]
pub enum LineDecorationGlyph {
    /// A line added in the diff (green `+`).
    DiffAdded,
    /// A line removed in the diff (red `-`).
    DiffRemoved,
    /// A line modified in the diff (yellow `~`).
    DiffChanged,
    /// A conflict marker awaiting merge resolution.
    Conflict,
    /// A user-placed bookmark.
    Bookmark,
    /// A breakpoint (reserved for future debugger integration).
    Breakpoint,
    /// Caller-supplied icon and color. Use for any glyph the fixed
    /// variants don't already cover.
    Custom {
        /// Icon to render.
        icon: IconName,
        /// Paint color for the icon.
        color: Hsla,
    },
}
