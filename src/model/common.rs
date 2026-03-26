use std::path::PathBuf;
use std::str::FromStr;

use crate::model::board::{ColorRgba, PcbItem, PolygonWithHolesNm, Vector2Nm};
use crate::proto::kiapi::common::types as common_types;

#[derive(Clone, Debug, Eq, PartialEq)]
/// KiCad semantic version returned by `GetVersion`.
pub struct VersionInfo {
    /// Major version component.
    pub major: u32,
    /// Minor version component.
    pub minor: u32,
    /// Patch version component.
    pub patch: u32,
    /// Full KiCad version string (includes prerelease/build details).
    pub full_version: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// KiCad top-level frame/editor targets used by API commands.
pub enum EditorFrameType {
    /// KiCad project manager frame.
    ProjectManager,
    /// Schematic editor frame.
    SchematicEditor,
    /// PCB editor frame.
    PcbEditor,
    /// Spice simulator frame.
    SpiceSimulator,
    /// Symbol editor frame.
    SymbolEditor,
    /// Footprint editor frame.
    FootprintEditor,
    /// Drawing-sheet editor frame.
    DrawingSheetEditor,
}

impl EditorFrameType {
    pub(crate) fn to_proto(self) -> i32 {
        match self {
            Self::ProjectManager => common_types::FrameType::FtProjectManager as i32,
            Self::SchematicEditor => common_types::FrameType::FtSchematicEditor as i32,
            Self::PcbEditor => common_types::FrameType::FtPcbEditor as i32,
            Self::SpiceSimulator => common_types::FrameType::FtSpiceSimulator as i32,
            Self::SymbolEditor => common_types::FrameType::FtSymbolEditor as i32,
            Self::FootprintEditor => common_types::FrameType::FtFootprintEditor as i32,
            Self::DrawingSheetEditor => common_types::FrameType::FtDrawingSheetEditor as i32,
        }
    }
}

impl std::fmt::Display for EditorFrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::ProjectManager => "project-manager",
            Self::SchematicEditor => "schematic",
            Self::PcbEditor => "pcb",
            Self::SpiceSimulator => "spice",
            Self::SymbolEditor => "symbol",
            Self::FootprintEditor => "footprint",
            Self::DrawingSheetEditor => "drawing-sheet",
        };
        write!(f, "{value}")
    }
}

impl FromStr for EditorFrameType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "project-manager" => Ok(Self::ProjectManager),
            "schematic" => Ok(Self::SchematicEditor),
            "pcb" => Ok(Self::PcbEditor),
            "spice" => Ok(Self::SpiceSimulator),
            "symbol" => Ok(Self::SymbolEditor),
            "footprint" => Ok(Self::FootprintEditor),
            "drawing-sheet" => Ok(Self::DrawingSheetEditor),
            _ => Err(format!(
                "unknown frame `{value}`; expected one of: project-manager, schematic, pcb, spice, symbol, footprint, drawing-sheet"
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// KiCad document type selector used by document-scoped APIs.
pub enum DocumentType {
    /// Schematic document.
    Schematic,
    /// Symbol document.
    Symbol,
    /// PCB document.
    Pcb,
    /// Footprint document.
    Footprint,
    /// Drawing-sheet document.
    DrawingSheet,
    /// Project-level document.
    Project,
}

impl DocumentType {
    pub(crate) fn to_proto(self) -> i32 {
        match self {
            Self::Schematic => common_types::DocumentType::DoctypeSchematic as i32,
            Self::Symbol => common_types::DocumentType::DoctypeSymbol as i32,
            Self::Pcb => common_types::DocumentType::DoctypePcb as i32,
            Self::Footprint => common_types::DocumentType::DoctypeFootprint as i32,
            Self::DrawingSheet => common_types::DocumentType::DoctypeDrawingSheet as i32,
            Self::Project => common_types::DocumentType::DoctypeProject as i32,
        }
    }

    pub(crate) fn from_proto(value: i32) -> Option<Self> {
        let ty = common_types::DocumentType::try_from(value).ok()?;
        match ty {
            common_types::DocumentType::DoctypeSchematic => Some(Self::Schematic),
            common_types::DocumentType::DoctypeSymbol => Some(Self::Symbol),
            common_types::DocumentType::DoctypePcb => Some(Self::Pcb),
            common_types::DocumentType::DoctypeFootprint => Some(Self::Footprint),
            common_types::DocumentType::DoctypeDrawingSheet => Some(Self::DrawingSheet),
            common_types::DocumentType::DoctypeProject => Some(Self::Project),
            common_types::DocumentType::DoctypeUnknown => None,
        }
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Schematic => "schematic",
            Self::Symbol => "symbol",
            Self::Pcb => "pcb",
            Self::Footprint => "footprint",
            Self::DrawingSheet => "drawing-sheet",
            Self::Project => "project",
        };

        write!(f, "{value}")
    }
}

impl FromStr for DocumentType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "schematic" => Ok(Self::Schematic),
            "symbol" => Ok(Self::Symbol),
            "pcb" => Ok(Self::Pcb),
            "footprint" => Ok(Self::Footprint),
            "drawing-sheet" => Ok(Self::DrawingSheet),
            "project" => Ok(Self::Project),
            _ => Err(format!(
                "unknown document type `{value}`; expected one of: schematic, symbol, pcb, footprint, drawing-sheet, project"
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Minimal project information attached to open-document responses.
pub struct ProjectInfo {
    /// Project display name, if provided by KiCad.
    pub name: Option<String>,
    /// Project filesystem path, if available.
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Descriptor for an open KiCad document.
pub struct DocumentSpecifier {
    /// KiCad document type.
    pub document_type: DocumentType,
    /// Board filename when relevant.
    pub board_filename: Option<String>,
    /// Owning project metadata.
    pub project: ProjectInfo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Count of selected items for a specific protobuf type URL.
pub struct SelectionTypeCount {
    /// Protobuf type URL for the selected item type.
    pub type_url: String,
    /// Number of selected items of this type.
    pub count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Summary of current selection composition.
pub struct SelectionSummary {
    /// Total selected item count.
    pub total_items: usize,
    /// Per-type counts by protobuf type URL.
    pub type_url_counts: Vec<SelectionTypeCount>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Human/debug-friendly selection entry detail.
pub struct SelectionItemDetail {
    /// Protobuf type URL.
    pub type_url: String,
    /// Decoded/debug string detail.
    pub detail: String,
    /// Raw payload length in bytes.
    pub raw_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Selection dump returned by `get_selection_as_string`.
pub struct SelectionStringDump {
    /// Ordered ids included in the serialized selection payload.
    pub ids: Vec<String>,
    /// Selection serialized as KiCad s-expression text.
    pub contents: String,
}

#[derive(Clone, Debug, PartialEq)]
/// Result of add/remove/clear selection mutations.
pub struct SelectionMutationResult {
    /// Decoded selected items after mutation.
    pub items: Vec<PcbItem>,
    /// Compact composition summary for the same selection state.
    pub summary: SelectionSummary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Opaque commit session identifier returned by `begin_commit`.
pub struct CommitSession {
    /// KiCad commit session id.
    pub id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Final action to apply when ending a commit session.
pub enum CommitAction {
    /// Persist commit changes.
    Commit,
    /// Discard commit changes.
    Drop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Status result returned by `run_action`.
pub enum RunActionStatus {
    /// Action succeeded.
    Ok,
    /// Action name or payload was invalid.
    Invalid,
    /// Target editor frame was not open.
    FrameNotOpen,
    /// Unrecognized status code from KiCad.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Merge strategy for map-like update APIs.
pub enum MapMergeMode {
    /// Merge provided entries into existing map.
    Merge,
    /// Replace existing map with provided entries.
    Replace,
}

impl std::fmt::Display for MapMergeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Merge => write!(f, "merge"),
            Self::Replace => write!(f, "replace"),
        }
    }
}

impl FromStr for MapMergeMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "merge" => Ok(Self::Merge),
            "replace" => Ok(Self::Replace),
            _ => Err(format!(
                "unknown merge mode `{value}`; expected `merge` or `replace`"
            )),
        }
    }
}

impl std::fmt::Display for CommitAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Commit => write!(f, "commit"),
            Self::Drop => write!(f, "drop"),
        }
    }
}

impl FromStr for CommitAction {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "commit" => Ok(Self::Commit),
            "drop" => Ok(Self::Drop),
            _ => Err(format!(
                "unknown commit action `{value}`; expected `commit` or `drop`"
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Title block fields from the active document.
pub struct TitleBlockInfo {
    /// Title block title.
    pub title: String,
    /// Title block date.
    pub date: String,
    /// Revision string.
    pub revision: String,
    /// Company field.
    pub company: String,
    /// Non-empty comment fields.
    pub comments: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Axis-aligned bounding box for a board item.
pub struct ItemBoundingBox {
    /// Item id this box belongs to.
    pub item_id: String,
    /// Left/top X coordinate in nm.
    pub x_nm: i64,
    /// Left/top Y coordinate in nm.
    pub y_nm: i64,
    /// Box width in nm.
    pub width_nm: i64,
    /// Box height in nm.
    pub height_nm: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Hit-test result for a single item query.
pub enum ItemHitTestResult {
    /// KiCad returned an unrecognized state.
    Unknown,
    /// The test point did not hit the item.
    NoHit,
    /// The test point hit the item.
    Hit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Stable `(code, name)` pair for a KiCad PCB object type.
pub struct PcbObjectTypeCode {
    /// Numeric KiCad object type code.
    pub code: i32,
    /// Symbolic name for the type code.
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Horizontal alignment used by text rendering APIs.
pub enum TextHorizontalAlignment {
    /// Alignment is not specified or not known.
    Unknown,
    /// Left-aligned text.
    Left,
    /// Center-aligned text.
    Center,
    /// Right-aligned text.
    Right,
    /// Mixed or indeterminate alignment.
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Vertical alignment used by text rendering APIs.
pub enum TextVerticalAlignment {
    /// Alignment is not specified or not known.
    Unknown,
    /// Top-aligned text.
    Top,
    /// Center-aligned text.
    Center,
    /// Bottom-aligned text.
    Bottom,
    /// Mixed or indeterminate alignment.
    Indeterminate,
}

#[derive(Clone, Debug, PartialEq)]
/// Text appearance settings accepted by KiCad text APIs.
pub struct TextAttributesSpec {
    /// Font family name, when explicitly selected.
    pub font_name: Option<String>,
    /// Requested horizontal alignment.
    pub horizontal_alignment: TextHorizontalAlignment,
    /// Requested vertical alignment.
    pub vertical_alignment: TextVerticalAlignment,
    /// Rotation angle in degrees.
    pub angle_degrees: Option<f64>,
    /// Relative line spacing value.
    pub line_spacing: Option<f64>,
    /// Stroke width in nm.
    pub stroke_width_nm: Option<i64>,
    /// Whether italic styling is enabled.
    pub italic: bool,
    /// Whether bold styling is enabled.
    pub bold: bool,
    /// Whether underlining is enabled.
    pub underlined: bool,
    /// Whether the text is mirrored.
    pub mirrored: bool,
    /// Whether multiline layout is enabled.
    pub multiline: bool,
    /// Whether text should remain upright when rotated.
    pub keep_upright: bool,
    /// Text size in nm.
    pub size_nm: Option<Vector2Nm>,
}

impl Default for TextAttributesSpec {
    fn default() -> Self {
        Self {
            font_name: None,
            horizontal_alignment: TextHorizontalAlignment::Unknown,
            vertical_alignment: TextVerticalAlignment::Unknown,
            angle_degrees: None,
            line_spacing: None,
            stroke_width_nm: None,
            italic: false,
            bold: false,
            underlined: false,
            mirrored: false,
            multiline: false,
            keep_upright: false,
            size_nm: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Single-line text object specification.
pub struct TextSpec {
    /// Text content to render.
    pub text: String,
    /// Anchor position in nm.
    pub position_nm: Option<Vector2Nm>,
    /// Optional text styling attributes.
    pub attributes: Option<TextAttributesSpec>,
    /// Optional hyperlink attached to the text object.
    pub hyperlink: Option<String>,
}

impl TextSpec {
    /// Creates a plain text spec with default positioning and styling.
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position_nm: None,
            attributes: None,
            hyperlink: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bounding box returned by `get_text_extents`.
pub struct TextExtents {
    /// Left/top X coordinate in nm.
    pub x_nm: i64,
    /// Left/top Y coordinate in nm.
    pub y_nm: i64,
    /// Width in nm.
    pub width_nm: i64,
    /// Height in nm.
    pub height_nm: i64,
}

#[derive(Clone, Debug, PartialEq)]
/// Text-box object specification.
pub struct TextBoxSpec {
    /// Text content inside the box.
    pub text: String,
    /// Top-left corner in nm.
    pub top_left_nm: Option<Vector2Nm>,
    /// Bottom-right corner in nm.
    pub bottom_right_nm: Option<Vector2Nm>,
    /// Optional text styling attributes.
    pub attributes: Option<TextAttributesSpec>,
}

#[derive(Clone, Debug, PartialEq)]
/// Text object input accepted by `get_text_as_shapes`.
pub enum TextObjectSpec {
    /// A single text item.
    Text(TextSpec),
    /// A text box item.
    TextBox(TextBoxSpec),
}

#[derive(Clone, Debug, PartialEq)]
/// Shape geometry produced when text is converted to drawable primitives.
pub enum TextShapeGeometry {
    /// Straight segment geometry.
    Segment {
        /// Segment start point in nm.
        start_nm: Option<Vector2Nm>,
        /// Segment end point in nm.
        end_nm: Option<Vector2Nm>,
    },
    /// Rectangle geometry.
    Rectangle {
        /// Top-left corner in nm.
        top_left_nm: Option<Vector2Nm>,
        /// Bottom-right corner in nm.
        bottom_right_nm: Option<Vector2Nm>,
        /// Corner radius in nm.
        corner_radius_nm: Option<i64>,
    },
    /// Arc geometry.
    Arc {
        /// Arc start point in nm.
        start_nm: Option<Vector2Nm>,
        /// Arc midpoint in nm.
        mid_nm: Option<Vector2Nm>,
        /// Arc end point in nm.
        end_nm: Option<Vector2Nm>,
    },
    /// Circle geometry.
    Circle {
        /// Circle center in nm.
        center_nm: Option<Vector2Nm>,
        /// Point on the circle radius in nm.
        radius_point_nm: Option<Vector2Nm>,
    },
    /// Polygon geometry.
    Polygon {
        /// One or more polygons representing the filled outline.
        polygons: Vec<PolygonWithHolesNm>,
    },
    /// Cubic Bezier geometry.
    Bezier {
        /// Curve start point in nm.
        start_nm: Option<Vector2Nm>,
        /// First control point in nm.
        control1_nm: Option<Vector2Nm>,
        /// Second control point in nm.
        control2_nm: Option<Vector2Nm>,
        /// Curve end point in nm.
        end_nm: Option<Vector2Nm>,
    },
    /// Unrecognized geometry returned by KiCad.
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
/// Drawable shape emitted for a text object.
pub struct TextShape {
    /// Geometric primitive for the shape.
    pub geometry: TextShapeGeometry,
    /// Stroke width in nm.
    pub stroke_width_nm: Option<i64>,
    /// KiCad stroke style code.
    pub stroke_style: Option<i32>,
    /// Stroke color.
    pub stroke_color: Option<ColorRgba>,
    /// KiCad fill type code.
    pub fill_type: Option<i32>,
    /// Fill color.
    pub fill_color: Option<ColorRgba>,
}

#[derive(Clone, Debug, PartialEq)]
/// Result row returned by `get_text_as_shapes`.
pub struct TextAsShapesEntry {
    /// Original source object, when echoed back by KiCad.
    pub source: Option<TextObjectSpec>,
    /// Shapes generated from the source object.
    pub shapes: Vec<TextShape>,
}

impl std::fmt::Display for ItemHitTestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Unknown => "unknown",
            Self::NoHit => "no-hit",
            Self::Hit => "hit",
        };

        write!(f, "{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::{CommitAction, EditorFrameType, MapMergeMode};
    use std::str::FromStr;

    #[test]
    fn commit_action_parses_known_values() {
        assert_eq!(CommitAction::from_str("commit"), Ok(CommitAction::Commit));
        assert_eq!(CommitAction::from_str("drop"), Ok(CommitAction::Drop));
    }

    #[test]
    fn commit_action_rejects_unknown_values() {
        assert!(CommitAction::from_str("rollback").is_err());
    }

    #[test]
    fn editor_frame_type_parses_known_values() {
        assert_eq!(
            EditorFrameType::from_str("pcb"),
            Ok(EditorFrameType::PcbEditor)
        );
        assert_eq!(
            EditorFrameType::from_str("project-manager"),
            Ok(EditorFrameType::ProjectManager)
        );
    }

    #[test]
    fn editor_frame_type_rejects_unknown_values() {
        assert!(EditorFrameType::from_str("layout").is_err());
    }

    #[test]
    fn map_merge_mode_parses_known_values() {
        assert_eq!(MapMergeMode::from_str("merge"), Ok(MapMergeMode::Merge));
        assert_eq!(MapMergeMode::from_str("replace"), Ok(MapMergeMode::Replace));
    }

    #[test]
    fn map_merge_mode_rejects_unknown_values() {
        assert!(MapMergeMode::from_str("upsert").is_err());
    }
}
