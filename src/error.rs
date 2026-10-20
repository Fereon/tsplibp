use std::{error::Error, fmt::Display};

use crate::datatypes::NodeIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TSPLibError {
    InvalidIntegerFormat(String),
    InvalidRealFormat(String),
    InvalidType(String),
    InvalidDimension(Box<Self>),
    InvalidCapacity(Box<Self>),
    InvalidEdgeWeightType(String),
    InvalidEdgeWeightFormat(String),
    InvalidEdgeDataFormat(String),
    InvalidNodeCoordType(String),
    InvalidDisplayDataType(String),
    MissingName,
    MissingType,
    MissingDimension,
    MissingEdgeDataFormat,
    MissingEdgeWeightFormat,
    UnknownNodeCoordType,
    UnexpectedNodeCoordSection,
    UnexpectedKeyword(String),
    ParsingIncomplete(String),
    NodeNotFound(NodeIdentifier),
    WeightsNotAvailable(String),
    UnsupportedWeightCalculation(String),
    InvalidTour(String),
    IoError(String),
}

impl Display for TSPLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TSPLibError::InvalidIntegerFormat(s) => write!(f, "invalid INTEGER: {s}"),
            TSPLibError::InvalidRealFormat(s) => write!(f, "invalid REAL: {s}"),
            TSPLibError::InvalidType(s) => write!(f, "invalid TYPE: {s}"),
            TSPLibError::InvalidDimension(tsperror) => write!(f, "invalid DIMENSION: {tsperror}"),
            TSPLibError::InvalidCapacity(tsperror) => write!(f, "invalid CAPACITY: {tsperror}"),
            TSPLibError::InvalidEdgeWeightType(s) => write!(f, "invalid EDGE_WEIGHT_TYPE: {s}"),
            TSPLibError::InvalidEdgeWeightFormat(s) => write!(f, "invalid EDGE_WEIGHT_FORMAT: {s}"),
            TSPLibError::InvalidEdgeDataFormat(s) => write!(f, "invalid EDGE_DATA_FORMAT: {s}"),
            TSPLibError::InvalidNodeCoordType(s) => write!(f, "invalid NODE_COORD_TYPE: {s}"),
            TSPLibError::InvalidDisplayDataType(s) => write!(f, "invalid DISPLAY_DATA_TYPE: {s}"),
            TSPLibError::MissingName => write!(f, "NAME missing"),
            TSPLibError::MissingType => write!(f, "TYPE missing"),
            TSPLibError::MissingDimension => write!(f, "DIMENSION missing"),
            TSPLibError::MissingEdgeDataFormat => write!(f, "EDGE_DATA_FORMAT missing"),
            TSPLibError::MissingEdgeWeightFormat => write!(f, "EDGE_WEIGHT_FORMAT missing"),
            TSPLibError::UnknownNodeCoordType => {
                write!(f, "NODE_COORD_TYPE could not be determined")
            }
            TSPLibError::UnexpectedNodeCoordSection => {
                write!(f, "unexpected NODE_COORD_SECTION found")
            }
            TSPLibError::UnexpectedKeyword(s) => write!(f, "unexpected KEYWORD: {s}"),
            TSPLibError::ParsingIncomplete(s) => {
                write!(f, "parsing incomplete, remaining input: {s}")
            }
            TSPLibError::NodeNotFound(id) => write!(f, "node with identifier {id} not found"),
            TSPLibError::WeightsNotAvailable(s) => write!(f, "weights not available: {s}"),
            TSPLibError::UnsupportedWeightCalculation(s) => {
                write!(f, "unsupported weight calculation: {s}")
            }
            TSPLibError::InvalidTour(s) => write!(f, "invalid tour: {s}"),
            TSPLibError::IoError(s) => write!(f, "I/O error: {s}"),
        }
    }
}

impl Error for TSPLibError {}

impl From<std::io::Error> for TSPLibError {
    fn from(err: std::io::Error) -> Self {
        TSPLibError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let cases = [
            (
                TSPLibError::InvalidIntegerFormat("abc".into()),
                "invalid INTEGER: abc",
            ),
            (
                TSPLibError::InvalidRealFormat("xyz".into()),
                "invalid REAL: xyz",
            ),
            (TSPLibError::InvalidType("FOO".into()), "invalid TYPE: FOO"),
            (
                TSPLibError::InvalidDimension(Box::new(TSPLibError::InvalidIntegerFormat(
                    "-1".into(),
                ))),
                "invalid DIMENSION: invalid INTEGER: -1",
            ),
            (
                TSPLibError::InvalidCapacity(Box::new(TSPLibError::InvalidIntegerFormat(
                    "-5".into(),
                ))),
                "invalid CAPACITY: invalid INTEGER: -5",
            ),
            (
                TSPLibError::InvalidEdgeWeightType("BAD".into()),
                "invalid EDGE_WEIGHT_TYPE: BAD",
            ),
            (
                TSPLibError::InvalidEdgeWeightFormat("BAD".into()),
                "invalid EDGE_WEIGHT_FORMAT: BAD",
            ),
            (
                TSPLibError::InvalidEdgeDataFormat("BAD".into()),
                "invalid EDGE_DATA_FORMAT: BAD",
            ),
            (
                TSPLibError::InvalidNodeCoordType("BAD".into()),
                "invalid NODE_COORD_TYPE: BAD",
            ),
            (
                TSPLibError::InvalidDisplayDataType("BAD".into()),
                "invalid DISPLAY_DATA_TYPE: BAD",
            ),
            (TSPLibError::MissingName, "NAME missing"),
            (TSPLibError::MissingType, "TYPE missing"),
            (TSPLibError::MissingDimension, "DIMENSION missing"),
            (
                TSPLibError::MissingEdgeDataFormat,
                "EDGE_DATA_FORMAT missing",
            ),
            (
                TSPLibError::MissingEdgeWeightFormat,
                "EDGE_WEIGHT_FORMAT missing",
            ),
            (
                TSPLibError::UnknownNodeCoordType,
                "NODE_COORD_TYPE could not be determined",
            ),
            (
                TSPLibError::UnexpectedNodeCoordSection,
                "unexpected NODE_COORD_SECTION found",
            ),
            (
                TSPLibError::UnexpectedKeyword("BOGUS".into()),
                "unexpected KEYWORD: BOGUS",
            ),
            (
                TSPLibError::ParsingIncomplete("leftover".into()),
                "parsing incomplete, remaining input: leftover",
            ),
            (
                TSPLibError::NodeNotFound(42),
                "node with identifier 42 not found",
            ),
            (
                TSPLibError::WeightsNotAvailable("none".into()),
                "weights not available: none",
            ),
            (
                TSPLibError::UnsupportedWeightCalculation("XRAY1".into()),
                "unsupported weight calculation: XRAY1",
            ),
            (
                TSPLibError::InvalidTour("too short".into()),
                "invalid tour: too short",
            ),
            (
                TSPLibError::IoError("not found".into()),
                "I/O error: not found",
            ),
        ];

        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected);
        }
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing file");
        let tsp_err: TSPLibError = io_err.into();
        assert_eq!(tsp_err, TSPLibError::IoError("missing file".into()));
    }
}
