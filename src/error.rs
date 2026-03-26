use std::time::Duration;

use thiserror::Error;

#[derive(Debug, Error)]
/// Error type returned by `kicad-ipc-rs` operations.
pub enum KiCadError {
    /// Invalid local configuration or user input before IPC dispatch.
    #[error("invalid configuration: {reason}")]
    Config {
        /// Human-readable validation failure details.
        reason: String,
    },

    /// KiCad IPC socket could not be found at connect time.
    #[error("KiCad IPC socket not available at `{socket_uri}`. Open KiCad and open a project/board first.")]
    SocketUnavailable {
        /// Resolved socket URI/path that was checked.
        socket_uri: String,
    },

    /// IPC connection failed.
    #[error("connection failed for `{socket_uri}`: {reason}")]
    Connection {
        /// Socket URI/path the client attempted to connect to.
        socket_uri: String,
        /// Underlying connection failure details.
        reason: String,
    },

    /// Transport send path failed.
    #[error("transport send failed: {reason}")]
    TransportSend {
        /// Underlying transport error details.
        reason: String,
    },

    /// Transport receive path failed.
    #[error("transport receive failed: {reason}")]
    TransportReceive {
        /// Underlying transport error details.
        reason: String,
    },

    /// Background transport task has stopped.
    #[error("transport task is unavailable")]
    TransportClosed,

    /// Request exceeded configured timeout.
    #[error("request timed out after {timeout:?}")]
    Timeout {
        /// Timeout value that was exceeded.
        timeout: Duration,
    },

    /// KiCad returned a non-success API status.
    #[error("API status error `{code}`: {message}")]
    ApiStatus {
        /// Status code returned by KiCad.
        code: String,
        /// Human-readable status message.
        message: String,
    },

    /// KiCad returned a non-success per-item status.
    #[error("item request status error `{code}`")]
    ItemStatus {
        /// Status code returned for the specific item operation.
        code: String,
    },

    /// Response payload content was malformed or inconsistent.
    #[error("invalid API response: {reason}")]
    InvalidResponse {
        /// Human-readable description of the response problem.
        reason: String,
    },

    /// Response payload was missing when required.
    #[error("API response missing payload for `{expected_type_url}`")]
    MissingPayload {
        /// Expected protobuf type URL for the missing payload.
        expected_type_url: String,
    },

    /// Response payload type did not match expected protobuf type URL.
    #[error("unexpected payload type; expected `{expected_type_url}`, got `{actual_type_url}`")]
    UnexpectedPayloadType {
        /// Expected protobuf type URL.
        expected_type_url: String,
        /// Actual protobuf type URL returned by KiCad.
        actual_type_url: String,
    },

    /// Protobuf encoding failed.
    #[error("protobuf encode failed: {0}")]
    ProtobufEncode(String),

    /// Protobuf decoding failed.
    #[error("protobuf decode failed: {0}")]
    ProtobufDecode(String),

    /// Blocking runtime worker join failed.
    #[error("runtime task join failed: {0}")]
    RuntimeJoin(String),

    /// Blocking runtime worker is unavailable.
    #[error("blocking runtime is unavailable")]
    BlockingRuntimeClosed,

    /// Internal mutex poisoning detected.
    #[error("mutex poisoned")]
    InternalPoisoned,

    /// Operation requires an open PCB document.
    #[error("no open PCB document found; open a board in KiCad first")]
    BoardNotOpen,

    /// Multiple project paths were detected where a single path was required.
    #[error("multiple project paths found across open PCB docs: {paths:?}")]
    AmbiguousProjectPath {
        /// Candidate project paths returned by KiCad.
        paths: Vec<String>,
    },

    /// Multiple open PCB docs prevent choosing an implicit board context.
    #[error("multiple PCB documents are open; unable to choose one board context: {boards:?}")]
    AmbiguousBoardSelection {
        /// Candidate board names/paths that prevented implicit selection.
        boards: Vec<String>,
    },
}
