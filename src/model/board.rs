use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
/// KiCad net descriptor.
pub struct BoardNet {
    /// Numeric net code.
    pub code: i32,
    /// Net name.
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Board layer descriptor.
pub struct BoardLayerInfo {
    /// KiCad layer id.
    pub id: i32,
    /// Human-readable layer name.
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Enabled layer set for a board.
pub struct BoardEnabledLayers {
    /// Number of copper layers configured in the board stack.
    pub copper_layer_count: u32,
    /// Enabled board layers.
    pub layers: Vec<BoardLayerInfo>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Board origin kind.
pub enum BoardOriginKind {
    /// Grid origin.
    Grid,
    /// Drill/place origin.
    Drill,
}

impl FromStr for BoardOriginKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "grid" => Ok(Self::Grid),
            "drill" => Ok(Self::Drill),
            _ => Err(format!(
                "unknown board origin kind `{value}`; expected `grid` or `drill`"
            )),
        }
    }
}

impl std::fmt::Display for BoardOriginKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grid => write!(f, "grid"),
            Self::Drill => write!(f, "drill"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// 2D coordinate in nanometer units.
pub struct Vector2Nm {
    /// X coordinate in nm.
    pub x_nm: i64,
    /// Y coordinate in nm.
    pub y_nm: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Pad-to-net lookup row derived from footprint items.
pub struct PadNetEntry {
    /// Footprint reference (e.g. `U1`) when available.
    pub footprint_reference: Option<String>,
    /// Footprint id when available.
    pub footprint_id: Option<String>,
    /// Pad item id when available.
    pub pad_id: Option<String>,
    /// Pad number/text as shown in KiCad.
    pub pad_number: String,
    /// Net code when connected.
    pub net_code: Option<i32>,
    /// Net name when connected.
    pub net_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Arc geometry in nanometer units.
pub struct ArcStartMidEndNm {
    /// Arc start point.
    pub start: Vector2Nm,
    /// Arc midpoint.
    pub mid: Vector2Nm,
    /// Arc end point.
    pub end: Vector2Nm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Polyline node geometry.
pub enum PolyLineNodeGeometryNm {
    /// Straight segment point.
    Point(Vector2Nm),
    /// Arc segment node.
    Arc(ArcStartMidEndNm),
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Polyline geometry.
pub struct PolyLineNm {
    /// Ordered geometry nodes.
    pub nodes: Vec<PolyLineNodeGeometryNm>,
    /// Whether last node closes back to first.
    pub closed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Polygon with optional interior holes.
pub struct PolygonWithHolesNm {
    /// Outer outline polygon.
    pub outline: Option<PolyLineNm>,
    /// Interior holes.
    pub holes: Vec<PolyLineNm>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Polygonized pad shape for a specific layer.
pub struct PadShapeAsPolygonEntry {
    /// Pad item id.
    pub pad_id: String,
    /// Layer id used for the polygon conversion.
    pub layer_id: i32,
    /// Layer name used for the polygon conversion.
    pub layer_name: String,
    /// Polygonal representation of the pad on that layer.
    pub polygon: PolygonWithHolesNm,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Padstack presence result for a single item/layer pair.
pub struct PadstackPresenceEntry {
    /// Item id that was queried.
    pub item_id: String,
    /// Layer id that was checked.
    pub layer_id: i32,
    /// Human-readable layer name.
    pub layer_name: String,
    /// Presence state reported by KiCad.
    pub presence: PadstackPresenceState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Whether a padstack is present on a given layer.
pub enum PadstackPresenceState {
    /// The padstack is present on the layer.
    Present,
    /// The padstack is not present on the layer.
    NotPresent,
    /// KiCad returned an unrecognized presence code.
    Unknown(i32),
}

impl std::fmt::Display for PadstackPresenceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Present => write!(f, "PSP_PRESENT"),
            Self::NotPresent => write!(f, "PSP_NOT_PRESENT"),
            Self::Unknown(value) => write!(f, "UNKNOWN({value})"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// RGBA color using floating-point channel values.
pub struct ColorRgba {
    /// Red channel.
    pub r: f64,
    /// Green channel.
    pub g: f64,
    /// Blue channel.
    pub b: f64,
    /// Alpha channel.
    pub a: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Stackup layer category.
pub enum BoardStackupLayerType {
    /// Copper conducting layer.
    Copper,
    /// Dielectric insulating layer.
    Dielectric,
    /// Silkscreen layer.
    Silkscreen,
    /// Solder mask layer.
    SolderMask,
    /// Solder paste layer.
    SolderPaste,
    /// Layer type is explicitly undefined.
    Undefined,
    /// Unrecognized layer type code.
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq)]
/// Material properties for a dielectric slice in the stackup.
pub struct BoardStackupDielectricProperties {
    /// Relative permittivity.
    pub epsilon_r: f64,
    /// Loss tangent.
    pub loss_tangent: f64,
    /// Material name.
    pub material_name: String,
    /// Thickness in nm, when available.
    pub thickness_nm: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
/// One layer entry in the board stackup.
pub struct BoardStackupLayer {
    /// Layer metadata.
    pub layer: BoardLayerInfo,
    /// User-visible layer name override.
    pub user_name: String,
    /// Material name assigned to the layer.
    pub material_name: String,
    /// Whether the layer is enabled.
    pub enabled: bool,
    /// Layer thickness in nm, when available.
    pub thickness_nm: Option<i64>,
    /// Layer category.
    pub layer_type: BoardStackupLayerType,
    /// Optional display color.
    pub color: Option<ColorRgba>,
    /// Associated dielectric properties.
    pub dielectric_layers: Vec<BoardStackupDielectricProperties>,
}

#[derive(Clone, Debug, PartialEq)]
/// Full board stackup description.
pub struct BoardStackup {
    /// Surface finish description.
    pub finish_type_name: String,
    /// Whether impedance control is enabled.
    pub impedance_controlled: bool,
    /// Whether the board edge hosts a connector.
    pub edge_has_connector: bool,
    /// Whether the board edge has castellated pads.
    pub edge_has_castellated_pads: bool,
    /// Whether the board edge uses edge plating.
    pub edge_has_edge_plating: bool,
    /// Ordered stackup layers.
    pub layers: Vec<BoardStackupLayer>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Layer grouping used for graphics defaults.
pub enum BoardLayerClass {
    /// Silkscreen layers.
    Silkscreen,
    /// Copper layers.
    Copper,
    /// Edge cuts / outline layers.
    Edges,
    /// Courtyard layers.
    Courtyard,
    /// Fabrication layers.
    Fabrication,
    /// Other layer classes.
    Other,
    /// Unrecognized layer class code.
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq)]
/// Default graphics settings for a layer class.
pub struct BoardLayerGraphicsDefault {
    /// Target layer class.
    pub layer_class: BoardLayerClass,
    /// Default line thickness in nm.
    pub line_thickness_nm: Option<i64>,
    /// Default text font name.
    pub text_font_name: Option<String>,
    /// Default text size in nm.
    pub text_size_nm: Option<Vector2Nm>,
    /// Default text stroke width in nm.
    pub text_stroke_width_nm: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
/// Board-level graphics defaults grouped by layer class.
pub struct GraphicsDefaults {
    /// Per-class defaults.
    pub layers: Vec<BoardLayerGraphicsDefault>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// How inactive layers are displayed in the board editor.
pub enum InactiveLayerDisplayMode {
    /// Draw inactive layers normally.
    Normal,
    /// Dim inactive layers.
    Dimmed,
    /// Hide inactive layers.
    Hidden,
    /// Unrecognized mode code.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// How net colors are shown in the board editor.
pub enum NetColorDisplayMode {
    /// Show colors for all nets.
    All,
    /// Show colors for ratsnest only.
    Ratsnest,
    /// Disable net color display.
    Off,
    /// Unrecognized mode code.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Board flip mode in the editor.
pub enum BoardFlipMode {
    /// Normal top/bottom orientation.
    Normal,
    /// Flipped around the X axis.
    FlippedX,
    /// Unrecognized flip mode code.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Which ratsnest lines KiCad should display.
pub enum RatsnestDisplayMode {
    /// Show ratsnest on all layers.
    AllLayers,
    /// Show ratsnest only on visible layers.
    VisibleLayers,
    /// Unrecognized mode code.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Severity level for injected DRC markers.
pub enum DrcSeverity {
    /// Warning severity.
    Warning,
    /// Error severity.
    Error,
    /// Exclusion severity.
    Exclusion,
    /// Ignored severity.
    Ignore,
    /// Informational severity.
    Info,
    /// Action-item severity.
    Action,
    /// Debug severity.
    Debug,
    /// Undefined severity.
    Undefined,
}

impl std::fmt::Display for DrcSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Exclusion => "exclusion",
            Self::Ignore => "ignore",
            Self::Info => "info",
            Self::Action => "action",
            Self::Debug => "debug",
            Self::Undefined => "undefined",
        };
        write!(f, "{value}")
    }
}

impl FromStr for DrcSeverity {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "exclusion" => Ok(Self::Exclusion),
            "ignore" => Ok(Self::Ignore),
            "info" => Ok(Self::Info),
            "action" => Ok(Self::Action),
            "debug" => Ok(Self::Debug),
            "undefined" => Ok(Self::Undefined),
            _ => Err(format!(
                "unknown drc severity `{value}`; expected warning, error, exclusion, ignore, info, action, debug, or undefined"
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Board editor appearance preferences.
pub struct BoardEditorAppearanceSettings {
    /// Inactive layer display mode.
    pub inactive_layer_display: InactiveLayerDisplayMode,
    /// Net color display mode.
    pub net_color_display: NetColorDisplayMode,
    /// Board flip mode.
    pub board_flip: BoardFlipMode,
    /// Ratsnest display mode.
    pub ratsnest_display: RatsnestDisplayMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Whether a net class is explicit or derived.
pub enum NetClassType {
    /// User-defined net class.
    Explicit,
    /// Implicit/default net class.
    Implicit,
    /// Unrecognized class type code.
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq)]
/// Board-specific routing rules attached to a net class.
pub struct NetClassBoardSettings {
    /// Clearance in nm.
    pub clearance_nm: Option<i64>,
    /// Track width in nm.
    pub track_width_nm: Option<i64>,
    /// Differential-pair track width in nm.
    pub diff_pair_track_width_nm: Option<i64>,
    /// Differential-pair gap in nm.
    pub diff_pair_gap_nm: Option<i64>,
    /// Differential-pair via gap in nm.
    pub diff_pair_via_gap_nm: Option<i64>,
    /// Optional display color.
    pub color: Option<ColorRgba>,
    /// Named tuning profile, when configured.
    pub tuning_profile: Option<String>,
    /// Whether via stack settings are present.
    pub has_via_stack: bool,
    /// Whether microvia stack settings are present.
    pub has_microvia_stack: bool,
}

#[derive(Clone, Debug, PartialEq)]
/// Net class definition.
pub struct NetClassInfo {
    /// Net class name.
    pub name: String,
    /// Ordering priority, when available.
    pub priority: Option<i32>,
    /// Net class kind.
    pub class_type: NetClassType,
    /// Child or member class names.
    pub constituents: Vec<String>,
    /// Board-specific settings.
    pub board: Option<NetClassBoardSettings>,
}

#[derive(Clone, Debug, PartialEq)]
/// Mapping from a net to its resolved net class.
pub struct NetClassForNetEntry {
    /// Net name.
    pub net_name: String,
    /// Resolved net class.
    pub net_class: NetClassInfo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Via type classification.
pub enum PcbViaType {
    /// Through-hole via.
    Through,
    /// Combined blind/buried via.
    BlindBuried,
    /// Microvia.
    Micro,
    /// Blind via.
    Blind,
    /// Buried via.
    Buried,
    /// Unrecognized via type code.
    Unknown(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Layer information associated with a via.
pub struct PcbViaLayers {
    /// Layers included in the padstack.
    pub padstack_layers: Vec<BoardLayerInfo>,
    /// Drill start layer, when available.
    pub drill_start_layer: Option<BoardLayerInfo>,
    /// Drill end layer, when available.
    pub drill_end_layer: Option<BoardLayerInfo>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// PCB pad type.
pub enum PcbPadType {
    /// Plated through-hole pad.
    Pth,
    /// Surface-mount pad.
    Smd,
    /// Edge connector pad.
    EdgeConnector,
    /// Non-plated through-hole pad.
    Npth,
    /// Unrecognized pad type code.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Zone category.
pub enum PcbZoneType {
    /// Copper zone.
    Copper,
    /// Graphical zone.
    Graphical,
    /// Rule area.
    RuleArea,
    /// Teardrop zone.
    Teardrop,
    /// Unrecognized zone type code.
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Lock state for mutable board items.
pub enum ItemLockState {
    /// Item is not locked.
    Unlocked,
    /// Item is locked.
    Locked,
    /// Unrecognized lock-state code.
    Unknown(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Drill description for a padstack.
pub struct PcbPadstackDrill {
    /// Drill start layer.
    pub start_layer: BoardLayerInfo,
    /// Drill end layer.
    pub end_layer: BoardLayerInfo,
    /// Drill diameter in nm.
    pub diameter_nm: Option<Vector2Nm>,
    /// Drill shape description.
    pub shape: Option<String>,
    /// Drill capping state, when reported.
    pub capped: Option<String>,
    /// Drill filling state, when reported.
    pub filled: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Padstack description for pads and vias.
pub struct PcbPadStack {
    /// KiCad stack type label.
    pub stack_type: Option<String>,
    /// Layers participating in the padstack.
    pub layers: Vec<BoardLayerInfo>,
    /// Primary drill definition.
    pub drill: Option<PcbPadstackDrill>,
    /// Unconnected layer removal mode.
    pub unconnected_layer_removal: Option<String>,
    /// Number of copper layers in the stack.
    pub copper_layer_count: usize,
    /// Whether front outer layers are present.
    pub has_front_outer_layers: bool,
    /// Whether back outer layers are present.
    pub has_back_outer_layers: bool,
    /// Whether zone settings are present.
    pub has_zone_settings: bool,
    /// Secondary drill definition, when present.
    pub secondary_drill: Option<PcbPadstackDrill>,
    /// Tertiary drill definition, when present.
    pub tertiary_drill: Option<PcbPadstackDrill>,
    /// Whether front post-machining is present.
    pub has_front_post_machining: bool,
    /// Whether back post-machining is present.
    pub has_back_post_machining: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Symbol pin linkage attached to a footprint pad.
pub struct PcbSymbolPinInfo {
    /// Pin name or number.
    pub name: String,
    /// KiCad pin type string, when available.
    pub pin_type: Option<String>,
    /// Whether the pin is explicitly marked no-connect.
    pub no_connect: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Resolved text styling for board text items.
pub struct PcbTextAttributes {
    /// Font family name.
    pub font_name: Option<String>,
    /// Horizontal alignment string.
    pub horizontal_alignment: Option<String>,
    /// Vertical alignment string.
    pub vertical_alignment: Option<String>,
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
    /// Whether text should remain upright.
    pub keep_upright: bool,
    /// Text size in nm.
    pub size_nm: Option<Vector2Nm>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Geometry payload for board graphic shapes.
pub enum PcbGraphicShapeGeometry {
    /// Line segment geometry.
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
        /// Point on the radius in nm.
        radius_point_nm: Option<Vector2Nm>,
    },
    /// Polygon geometry summarized by polygon count.
    Polygon {
        /// Number of polygons represented by the shape.
        polygon_count: usize,
    },
    /// Bezier geometry.
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Per-layer zone settings.
pub struct PcbZoneLayerProperty {
    /// Layer metadata.
    pub layer: BoardLayerInfo,
    /// Hatching offset in nm, when configured.
    pub hatching_offset_nm: Option<Vector2Nm>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Geometry/style payload for a PCB dimension.
pub enum PcbDimensionStyle {
    /// Aligned dimension.
    Aligned {
        /// Start point in nm.
        start_nm: Option<Vector2Nm>,
        /// End point in nm.
        end_nm: Option<Vector2Nm>,
        /// Dimension height in nm.
        height_nm: Option<i64>,
        /// Extension height in nm.
        extension_height_nm: Option<i64>,
    },
    /// Orthogonal dimension.
    Orthogonal {
        /// Start point in nm.
        start_nm: Option<Vector2Nm>,
        /// End point in nm.
        end_nm: Option<Vector2Nm>,
        /// Dimension height in nm.
        height_nm: Option<i64>,
        /// Extension height in nm.
        extension_height_nm: Option<i64>,
        /// Alignment mode string.
        alignment: Option<String>,
    },
    /// Radial dimension.
    Radial {
        /// Center point in nm.
        center_nm: Option<Vector2Nm>,
        /// Radius point in nm.
        radius_point_nm: Option<Vector2Nm>,
        /// Leader length in nm.
        leader_length_nm: Option<i64>,
    },
    /// Leader dimension.
    Leader {
        /// Start point in nm.
        start_nm: Option<Vector2Nm>,
        /// End point in nm.
        end_nm: Option<Vector2Nm>,
        /// Border style string.
        border_style: Option<String>,
    },
    /// Center mark / centerline dimension.
    Center {
        /// Center point in nm.
        center_nm: Option<Vector2Nm>,
        /// End point in nm.
        end_nm: Option<Vector2Nm>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Schematic symbol link metadata for a footprint.
pub struct PcbFootprintSymbolLink {
    /// Whether a symbol path is present.
    pub has_symbol_path: bool,
    /// Sheet name, when available.
    pub sheet_name: Option<String>,
    /// Sheet filename, when available.
    pub sheet_filename: Option<String>,
    /// Footprint filter string, when available.
    pub footprint_filters: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB track item.
pub struct PcbTrack {
    /// Item id, when available.
    pub id: Option<String>,
    /// Start point in nm.
    pub start_nm: Option<Vector2Nm>,
    /// End point in nm.
    pub end_nm: Option<Vector2Nm>,
    /// Track width in nm.
    pub width_nm: Option<i64>,
    /// Lock state.
    pub locked: ItemLockState,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Attached net, when any.
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB arc item.
pub struct PcbArc {
    /// Item id, when available.
    pub id: Option<String>,
    /// Start point in nm.
    pub start_nm: Option<Vector2Nm>,
    /// Midpoint in nm.
    pub mid_nm: Option<Vector2Nm>,
    /// End point in nm.
    pub end_nm: Option<Vector2Nm>,
    /// Arc width in nm.
    pub width_nm: Option<i64>,
    /// Lock state.
    pub locked: ItemLockState,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Attached net, when any.
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB via item.
pub struct PcbVia {
    /// Item id, when available.
    pub id: Option<String>,
    /// Via center position in nm.
    pub position_nm: Option<Vector2Nm>,
    /// Via type.
    pub via_type: PcbViaType,
    /// Lock state.
    pub locked: ItemLockState,
    /// Layer information, when available.
    pub layers: Option<PcbViaLayers>,
    /// Padstack description, when available.
    pub pad_stack: Option<PcbPadStack>,
    /// Attached net, when any.
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, PartialEq)]
/// PCB footprint item.
pub struct PcbFootprint {
    /// Item id, when available.
    pub id: Option<String>,
    /// Reference designator.
    pub reference: Option<String>,
    /// Footprint position in nm.
    pub position_nm: Option<Vector2Nm>,
    /// Orientation in degrees.
    pub orientation_deg: Option<f64>,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Lock state.
    pub locked: ItemLockState,
    /// Value field, when available.
    pub value: Option<String>,
    /// Datasheet field, when available.
    pub datasheet: Option<String>,
    /// Description field, when available.
    pub description: Option<String>,
    /// Whether attributes are present.
    pub has_attributes: bool,
    /// Whether overrides are present.
    pub has_overrides: bool,
    /// Whether a linked definition is present.
    pub has_definition: bool,
    /// Number of items in the linked definition.
    pub definition_item_count: usize,
    /// Symbol link metadata, when available.
    pub symbol_link: Option<PcbFootprintSymbolLink>,
    /// Number of pads in the footprint.
    pub pad_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB pad item.
pub struct PcbPad {
    /// Item id, when available.
    pub id: Option<String>,
    /// Lock state.
    pub locked: ItemLockState,
    /// Pad number/name.
    pub number: String,
    /// Pad type.
    pub pad_type: PcbPadType,
    /// Pad position in nm.
    pub position_nm: Option<Vector2Nm>,
    /// Padstack description, when available.
    pub pad_stack: Option<PcbPadStack>,
    /// Copper clearance override in nm.
    pub copper_clearance_override_nm: Option<i64>,
    /// Pad-to-die length in nm.
    pub pad_to_die_length_nm: Option<i64>,
    /// Pad-to-die delay in attoseconds.
    pub pad_to_die_delay_as: Option<i64>,
    /// Linked schematic pin info, when available.
    pub symbol_pin: Option<PcbSymbolPinInfo>,
    /// Attached net, when any.
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Graphical shape item on a PCB.
pub struct PcbBoardGraphicShape {
    /// Item id, when available.
    pub id: Option<String>,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Lock state.
    pub locked: ItemLockState,
    /// Attached net, when any.
    pub net: Option<BoardNet>,
    /// KiCad geometry-kind string.
    pub geometry_kind: Option<String>,
    /// Decoded geometry payload.
    pub geometry: Option<PcbGraphicShapeGeometry>,
    /// Stroke width in nm.
    pub stroke_width_nm: Option<i64>,
    /// Stroke style string.
    pub stroke_style: Option<String>,
    /// Fill type string.
    pub fill_type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Board text item.
pub struct PcbBoardText {
    /// Item id, when available.
    pub id: Option<String>,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Text content.
    pub text: Option<String>,
    /// Text anchor position in nm.
    pub position_nm: Option<Vector2Nm>,
    /// Hyperlink, when available.
    pub hyperlink: Option<String>,
    /// Text styling, when available.
    pub attributes: Option<PcbTextAttributes>,
    /// Whether knockout text rendering is enabled.
    pub knockout: bool,
    /// Lock state.
    pub locked: ItemLockState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Board text-box item.
pub struct PcbBoardTextBox {
    /// Item id, when available.
    pub id: Option<String>,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Text content.
    pub text: Option<String>,
    /// Top-left corner in nm.
    pub top_left_nm: Option<Vector2Nm>,
    /// Bottom-right corner in nm.
    pub bottom_right_nm: Option<Vector2Nm>,
    /// Text styling, when available.
    pub attributes: Option<PcbTextAttributes>,
    /// Lock state.
    pub locked: ItemLockState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Named field attached to a PCB item.
pub struct PcbField {
    /// Field name.
    pub name: String,
    /// Whether the field is visible.
    pub visible: bool,
    /// Field text contents.
    pub text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB zone item.
pub struct PcbZone {
    /// Item id, when available.
    pub id: Option<String>,
    /// Zone name.
    pub name: String,
    /// Zone type.
    pub zone_type: PcbZoneType,
    /// Layers touched by the zone.
    pub layers: Vec<BoardLayerInfo>,
    /// Number of layers in the zone.
    pub layer_count: usize,
    /// Zone priority.
    pub priority: u32,
    /// Lock state.
    pub locked: ItemLockState,
    /// Whether the zone is currently filled.
    pub filled: bool,
    /// Number of polygons in the zone.
    pub polygon_count: usize,
    /// Number of outline polygons.
    pub outline_polygon_count: usize,
    /// Whether copper settings are present.
    pub has_copper_settings: bool,
    /// Whether rule-area settings are present.
    pub has_rule_area_settings: bool,
    /// Border style string.
    pub border_style: Option<String>,
    /// Border pitch in nm.
    pub border_pitch_nm: Option<i64>,
    /// Per-layer properties.
    pub layer_properties: Vec<PcbZoneLayerProperty>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB dimension item.
pub struct PcbDimension {
    /// Item id, when available.
    pub id: Option<String>,
    /// Owning layer.
    pub layer: BoardLayerInfo,
    /// Lock state.
    pub locked: ItemLockState,
    /// Rendered text, when available.
    pub text: Option<String>,
    /// KiCad style-kind string.
    pub style_kind: Option<String>,
    /// Decoded dimension style payload.
    pub style: Option<PcbDimensionStyle>,
    /// Whether override text is enabled.
    pub override_text_enabled: bool,
    /// Override text contents.
    pub override_text: Option<String>,
    /// Prefix text.
    pub prefix: Option<String>,
    /// Suffix text.
    pub suffix: Option<String>,
    /// Unit string.
    pub unit: Option<String>,
    /// Unit-format string.
    pub unit_format: Option<String>,
    /// Arrow direction string.
    pub arrow_direction: Option<String>,
    /// Precision string.
    pub precision: Option<String>,
    /// Whether trailing zeroes are suppressed.
    pub suppress_trailing_zeroes: bool,
    /// Line thickness in nm.
    pub line_thickness_nm: Option<i64>,
    /// Arrow length in nm.
    pub arrow_length_nm: Option<i64>,
    /// Extension offset in nm.
    pub extension_offset_nm: Option<i64>,
    /// Text-position string.
    pub text_position: Option<String>,
    /// Whether text stays aligned to the dimension.
    pub keep_text_aligned: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// PCB group item.
pub struct PcbGroup {
    /// Item id, when available.
    pub id: Option<String>,
    /// Group name.
    pub name: String,
    /// Number of items in the group.
    pub item_count: usize,
    /// Member item ids.
    pub item_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Fallback item for unsupported or unknown payloads.
pub struct PcbUnknownItem {
    /// Protobuf type URL.
    pub type_url: String,
    /// Raw payload length in bytes.
    pub raw_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
/// Sum type for decoded PCB items.
pub enum PcbItem {
    /// Track item.
    Track(PcbTrack),
    /// Arc item.
    Arc(PcbArc),
    /// Via item.
    Via(PcbVia),
    /// Footprint item.
    Footprint(PcbFootprint),
    /// Pad item.
    Pad(PcbPad),
    /// Board graphic shape item.
    BoardGraphicShape(PcbBoardGraphicShape),
    /// Board text item.
    BoardText(PcbBoardText),
    /// Board text-box item.
    BoardTextBox(PcbBoardTextBox),
    /// Field item.
    Field(PcbField),
    /// Zone item.
    Zone(PcbZone),
    /// Dimension item.
    Dimension(PcbDimension),
    /// Group item.
    Group(PcbGroup),
    /// Unknown or unsupported item.
    Unknown(PcbUnknownItem),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{BoardOriginKind, DrcSeverity};

    #[test]
    fn board_origin_kind_parses_known_values() {
        assert_eq!(
            BoardOriginKind::from_str("grid").expect("grid should parse"),
            BoardOriginKind::Grid
        );
        assert_eq!(
            BoardOriginKind::from_str("drill").expect("drill should parse"),
            BoardOriginKind::Drill
        );
    }

    #[test]
    fn board_origin_kind_rejects_unknown_values() {
        let result = BoardOriginKind::from_str("other");
        assert!(result.is_err());
    }

    #[test]
    fn drc_severity_parses_known_values() {
        assert_eq!(
            DrcSeverity::from_str("warning").expect("warning should parse"),
            DrcSeverity::Warning
        );
        assert_eq!(
            DrcSeverity::from_str("error").expect("error should parse"),
            DrcSeverity::Error
        );
    }

    #[test]
    fn drc_severity_rejects_unknown_values() {
        let result = DrcSeverity::from_str("fatal");
        assert!(result.is_err());
    }
}
