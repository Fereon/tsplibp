use crate::{
    datatypes::{
        DisplayDataType, EdgeDataFormat, EdgeWeightFormat, EdgeWeightType, Integer, NodeCoordType,
        Specification, Type,
    },
    error::TSPLibError,
    parser::{parse_integer, parse_string, split_once_colon, split_once_newline},
};

fn parse_name(input: &str) -> Result<(String, &str), TSPLibError> {
    let (name, rest) = parse_string(input);
    if name.is_empty() {
        return Err(TSPLibError::MissingName);
    }
    Ok((name, rest))
}

fn parse_dtype(input: &str) -> Result<(Type, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let value = line.split_whitespace().next().unwrap_or("");
    let value = match value {
        "TSP" => Type::TSP,
        "ATSP" => Type::ATSP,
        "SOP" => Type::SOP,
        "HCP" => Type::HCP,
        "CVRP" => Type::CVRP,
        "TOUR" => Type::Tour,
        value => return Err(TSPLibError::InvalidType(value.to_string())),
    };

    Ok((value, rest))
}

fn parse_comment(input: &str) -> Result<(String, &str), TSPLibError> {
    Ok(parse_string(input))
}

fn parse_dimension(input: &str) -> Result<(Integer, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let token = line.split_whitespace().next().unwrap_or("");
    let (dim, _) = parse_integer(token).map_err(|err| TSPLibError::InvalidDimension(err.into()))?;
    if dim <= 0 {
        return Err(TSPLibError::InvalidDimension(Box::new(
            TSPLibError::InvalidIntegerFormat(dim.to_string()),
        )));
    }
    Ok((dim, rest))
}

fn parse_capacity(input: &str) -> Result<(Integer, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let token = line.split_whitespace().next().unwrap_or("");
    let (cap, _) = parse_integer(token).map_err(|err| TSPLibError::InvalidCapacity(err.into()))?;
    if cap < 0 {
        return Err(TSPLibError::InvalidCapacity(Box::new(
            TSPLibError::InvalidIntegerFormat(cap.to_string()),
        )));
    }
    Ok((cap, rest))
}

fn parse_edge_weight_type(input: &str) -> Result<(EdgeWeightType, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let value = line.split_whitespace().next().unwrap_or("");
    let value = match value {
        "EXPLICIT" => EdgeWeightType::Explicit,
        "EUC_2D" => EdgeWeightType::Euc2D,
        "EUC_3D" => EdgeWeightType::Euc3D,
        "MAX_2D" => EdgeWeightType::Max2D,
        "MAX_3D" => EdgeWeightType::Max3D,
        "MAN_2D" => EdgeWeightType::Man2D,
        "MAN_3D" => EdgeWeightType::Man3D,
        "CEIL_2D" => EdgeWeightType::Ceil2D,
        "GEO" => EdgeWeightType::Geo,
        "ATT" => EdgeWeightType::Att,
        "XRAY1" => EdgeWeightType::XRay1,
        "XRAY2" => EdgeWeightType::XRay2,
        "SPECIAL" => EdgeWeightType::Special,
        value => return Err(TSPLibError::InvalidEdgeWeightType(value.to_string())),
    };
    Ok((value, rest))
}

fn parse_edge_weight_format(input: &str) -> Result<(EdgeWeightFormat, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let value = line.split_whitespace().next().unwrap_or("");
    let value = match value {
        "FUNCTION" => EdgeWeightFormat::Function,
        "FULL_MATRIX" => EdgeWeightFormat::FullMatrix,
        "UPPER_ROW" => EdgeWeightFormat::UpperRow,
        "LOWER_ROW" => EdgeWeightFormat::LowerRow,
        "UPPER_DIAG_ROW" => EdgeWeightFormat::UpperDiagRow,
        "LOWER_DIAG_ROW" => EdgeWeightFormat::LowerDiagRow,
        "UPPER_COL" => EdgeWeightFormat::UpperCol,
        "LOWER_COL" => EdgeWeightFormat::LowerCol,
        "UPPER_DIAG_COL" => EdgeWeightFormat::UpperDiagCol,
        "LOWER_DIAG_COL" => EdgeWeightFormat::LowerDiagCol,
        value => return Err(TSPLibError::InvalidEdgeWeightFormat(value.to_string())),
    };
    Ok((value, rest))
}

fn parse_edge_data_format(input: &str) -> Result<(EdgeDataFormat, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let value = line.split_whitespace().next().unwrap_or("");
    let value = match value {
        "EDGE_LIST" => EdgeDataFormat::EdgeList,
        "ADJ_LIST" => EdgeDataFormat::AdjList,
        value => return Err(TSPLibError::InvalidEdgeDataFormat(value.to_string())),
    };
    Ok((value, rest))
}

fn parse_node_coord_type(input: &str) -> Result<(NodeCoordType, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let value = line.split_whitespace().next().unwrap_or("");
    let value = match value {
        "TWOD_COORDS" => NodeCoordType::TwodCoords,
        "THREED_COORDS" => NodeCoordType::ThreedCoords,
        "NO_COORDS" => NodeCoordType::NoCoords,
        value => return Err(TSPLibError::InvalidNodeCoordType(value.to_string())),
    };
    Ok((value, rest))
}

fn parse_display_data_type(input: &str) -> Result<(DisplayDataType, &str), TSPLibError> {
    let (line, rest) = split_once_newline(input);
    let value = line.split_whitespace().next().unwrap_or("");
    let value = match value {
        "COORD_DISPLAY" => DisplayDataType::CoordDisplay,
        "TWOD_DISPLAY" => DisplayDataType::TwodDisplay,
        "NO_DISPLAY" => DisplayDataType::NoDisplay,
        value => return Err(TSPLibError::InvalidDisplayDataType(value.to_string())),
    };
    Ok((value, rest))
}

pub fn parse_specification(input: &str) -> Result<(Specification, &str), TSPLibError> {
    let mut name = None;
    let mut dtype = None;
    let mut comment = Vec::new();
    let mut dimension = None;
    let mut capacity = None;
    let mut edge_weight_type = None;
    let mut edge_weight_format = None;
    let mut edge_data_format = None;
    let mut node_coord_type = None;
    let mut display_data_type = None;

    let mut rest = input.trim_start();
    loop {
        let old_rest = rest;
        let (line, _) = split_once_newline(rest);
        let (keyword, _) = split_once_colon(line);
        let keyword = keyword.trim();

        if line.is_empty() || keyword == "EOF" {
            if keyword == "EOF" {
                let (_, after_eof) = split_once_newline(rest);
                rest = after_eof;
            }
            break;
        }

        if matches!(
            keyword,
            "NAME"
                | "TYPE"
                | "COMMENT"
                | "DIMENSION"
                | "CAPACITY"
                | "EDGE_WEIGHT_TYPE"
                | "EDGE_WEIGHT_FORMAT"
                | "EDGE_DATA_FORMAT"
                | "NODE_COORD_TYPE"
                | "DISPLAY_DATA_TYPE"
        ) {
            let (_, after_colon) = split_once_colon(rest);
            rest = after_colon;
        }

        match keyword {
            "NAME" => (name, rest) = parse_name(rest).map(|(v, r)| (Some(v), r))?,
            "TYPE" => (dtype, rest) = parse_dtype(rest).map(|(v, r)| (Some(v), r))?,
            "COMMENT" => {
                rest = parse_comment(rest).map(|(v, r)| {
                    comment.push(v);
                    r
                })?
            }
            "DIMENSION" => (dimension, rest) = parse_dimension(rest).map(|(v, r)| (Some(v), r))?,
            "CAPACITY" => (capacity, rest) = parse_capacity(rest).map(|(v, r)| (Some(v), r))?,
            "EDGE_WEIGHT_TYPE" => {
                (edge_weight_type, rest) =
                    parse_edge_weight_type(rest).map(|(v, r)| (Some(v), r))?;
            }
            "EDGE_WEIGHT_FORMAT" => {
                (edge_weight_format, rest) =
                    parse_edge_weight_format(rest).map(|(v, r)| (Some(v), r))?;
            }
            "EDGE_DATA_FORMAT" => {
                (edge_data_format, rest) =
                    parse_edge_data_format(rest).map(|(v, r)| (Some(v), r))?;
            }
            "NODE_COORD_TYPE" => {
                (node_coord_type, rest) = parse_node_coord_type(rest).map(|(v, r)| (Some(v), r))?;
            }
            "DISPLAY_DATA_TYPE" => {
                (display_data_type, rest) =
                    parse_display_data_type(rest).map(|(v, r)| (Some(v), r))?;
            }
            _ => {
                rest = old_rest;
                break;
            }
        }
    }

    let specification = Specification {
        name: name.ok_or(TSPLibError::MissingName)?,
        dtype: dtype.ok_or(TSPLibError::MissingType)?,
        comment,
        dimension,
        capacity,
        edge_weight_type,
        edge_weight_format,
        edge_data_format,
        node_coord_type,
        display_data_type,
    };

    Ok((specification, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        assert_eq!(parse_name("test").unwrap(), ("test".to_string(), ""));
        assert_eq!(parse_name("\nTYPE: TSP"), Err(TSPLibError::MissingName));
    }

    #[test]
    fn test_parse_dtype() {
        assert_eq!(parse_dtype("TSP").unwrap(), (Type::TSP, ""));
        assert_eq!(
            parse_dtype("TSP (M.~Hofmeister)\nrest").unwrap(),
            (Type::TSP, "rest")
        );
        assert_eq!(parse_dtype("ATSP").unwrap(), (Type::ATSP, ""));
        assert_eq!(parse_dtype("SOP").unwrap(), (Type::SOP, ""));
        assert_eq!(parse_dtype("HCP").unwrap(), (Type::HCP, ""));
        assert_eq!(parse_dtype("CVRP").unwrap(), (Type::CVRP, ""));
        assert_eq!(parse_dtype("TOUR").unwrap(), (Type::Tour, ""));
        assert!(parse_dtype("INVALID").is_err());
    }

    #[test]
    fn test_parse_dimension() {
        assert_eq!(parse_dimension("100").unwrap(), (100, ""));
        assert_eq!(
            parse_dimension("100 (cities)\nrest").unwrap(),
            (100, "rest")
        );
        assert!(parse_dimension("0").is_err());
        assert!(parse_dimension("-5").is_err());
        assert!(parse_dimension("abc").is_err());
    }

    #[test]
    fn test_parse_capacity() {
        assert_eq!(parse_capacity("100").unwrap(), (100, ""));
        assert_eq!(parse_capacity("0").unwrap(), (0, ""));
        assert!(parse_capacity("-1").is_err());
        assert!(parse_capacity("abc").is_err());
    }

    #[test]
    fn test_parse_edge_weight_type() {
        let cases = [
            ("EXPLICIT", EdgeWeightType::Explicit),
            ("EUC_2D", EdgeWeightType::Euc2D),
            ("EUC_3D", EdgeWeightType::Euc3D),
            ("MAX_2D", EdgeWeightType::Max2D),
            ("MAX_3D", EdgeWeightType::Max3D),
            ("MAN_2D", EdgeWeightType::Man2D),
            ("MAN_3D", EdgeWeightType::Man3D),
            ("CEIL_2D", EdgeWeightType::Ceil2D),
            ("GEO", EdgeWeightType::Geo),
            ("ATT", EdgeWeightType::Att),
            ("XRAY1", EdgeWeightType::XRay1),
            ("XRAY2", EdgeWeightType::XRay2),
            ("SPECIAL", EdgeWeightType::Special),
        ];
        for (s, expected) in cases {
            assert_eq!(parse_edge_weight_type(s).unwrap(), (expected, ""));
        }
        assert!(parse_edge_weight_type("INVALID").is_err());
    }

    #[test]
    fn test_parse_edge_weight_format() {
        let cases = [
            ("FUNCTION", EdgeWeightFormat::Function),
            ("FULL_MATRIX", EdgeWeightFormat::FullMatrix),
            ("UPPER_ROW", EdgeWeightFormat::UpperRow),
            ("LOWER_ROW", EdgeWeightFormat::LowerRow),
            ("UPPER_DIAG_ROW", EdgeWeightFormat::UpperDiagRow),
            ("LOWER_DIAG_ROW", EdgeWeightFormat::LowerDiagRow),
            ("UPPER_COL", EdgeWeightFormat::UpperCol),
            ("LOWER_COL", EdgeWeightFormat::LowerCol),
            ("UPPER_DIAG_COL", EdgeWeightFormat::UpperDiagCol),
            ("LOWER_DIAG_COL", EdgeWeightFormat::LowerDiagCol),
        ];
        for (s, expected) in cases {
            assert_eq!(parse_edge_weight_format(s).unwrap(), (expected, ""));
        }
        assert!(parse_edge_weight_format("INVALID").is_err());
    }

    #[test]
    fn test_parse_edge_data_format() {
        assert_eq!(
            parse_edge_data_format("EDGE_LIST").unwrap(),
            (EdgeDataFormat::EdgeList, "")
        );
        assert_eq!(
            parse_edge_data_format("ADJ_LIST").unwrap(),
            (EdgeDataFormat::AdjList, "")
        );
        assert!(parse_edge_data_format("INVALID").is_err());
    }

    #[test]
    fn test_parse_node_coord_type() {
        assert_eq!(
            parse_node_coord_type("TWOD_COORDS").unwrap(),
            (NodeCoordType::TwodCoords, "")
        );
        assert_eq!(
            parse_node_coord_type("THREED_COORDS").unwrap(),
            (NodeCoordType::ThreedCoords, "")
        );
        assert_eq!(
            parse_node_coord_type("NO_COORDS").unwrap(),
            (NodeCoordType::NoCoords, "")
        );
        assert!(parse_node_coord_type("INVALID").is_err());
    }

    #[test]
    fn test_parse_display_data_type() {
        assert_eq!(
            parse_display_data_type("COORD_DISPLAY").unwrap(),
            (DisplayDataType::CoordDisplay, "")
        );
        assert_eq!(
            parse_display_data_type("TWOD_DISPLAY").unwrap(),
            (DisplayDataType::TwodDisplay, "")
        );
        assert_eq!(
            parse_display_data_type("NO_DISPLAY").unwrap(),
            (DisplayDataType::NoDisplay, "")
        );
        assert!(parse_display_data_type("INVALID").is_err());
    }

    #[test]
    fn test_parse_specification() {
        let input = "NAME: test\nTYPE: TSP\nCOMMENT: test comment\nDIMENSION: 10\nCAPACITY: 0\nEDGE_WEIGHT_TYPE: EUC_2D\nEDGE_WEIGHT_FORMAT: FUNCTION\nEDGE_DATA_FORMAT: EDGE_LIST\nNODE_COORD_TYPE: TWOD_COORDS\nDISPLAY_DATA_TYPE: NO_DISPLAY\nEOF\n";
        let (spec, rest) = parse_specification(input).unwrap();
        assert_eq!(spec.name, "test");
        assert_eq!(spec.dtype, Type::TSP);
        assert_eq!(spec.comment, vec!["test comment"]);
        assert_eq!(spec.dimension, Some(10));
        assert_eq!(spec.capacity, Some(0));
        assert_eq!(spec.edge_weight_type, Some(EdgeWeightType::Euc2D));
        assert_eq!(spec.edge_weight_format, Some(EdgeWeightFormat::Function));
        assert_eq!(spec.edge_data_format, Some(EdgeDataFormat::EdgeList));
        assert_eq!(spec.node_coord_type, Some(NodeCoordType::TwodCoords));
        assert_eq!(spec.display_data_type, Some(DisplayDataType::NoDisplay));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_specification_empty_comment_and_eof_colon() {
        let input = "NAME: test\nCOMMENT:\nCOMMENT : \nTYPE: TSP\nDIMENSION: 5\nEOF :\n";
        let (spec, rest) = parse_specification(input).unwrap();
        assert_eq!(spec.name, "test");
        assert_eq!(spec.dtype, Type::TSP);
        assert_eq!(spec.comment, vec!["", ""]);
        assert_eq!(spec.dimension, Some(5));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_specification_missing_required_fields() {
        assert_eq!(
            parse_specification("TYPE: TSP\nEOF"),
            Err(TSPLibError::MissingName)
        );
        assert_eq!(
            parse_specification("NAME:\nTYPE: TSP\nEOF"),
            Err(TSPLibError::MissingName)
        );
        assert_eq!(
            parse_specification("NAME: only_name\nEOF"),
            Err(TSPLibError::MissingType)
        );
    }
}
