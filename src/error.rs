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
    UnkownNodeCoordType,
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
            TSPLibError::UnkownNodeCoordType => {
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
