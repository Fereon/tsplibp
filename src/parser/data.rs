use crate::{
    datatypes::{
        Data, Demand, Edge, EdgeDataFormat, EdgeDataSection, EdgeWeight, EdgeWeightFormat,
        EdgeWeightType, Integer, NodeCoordSection, NodeCoordType, NodeIdentifier, Specification,
        ThreeDimensionalNode, TwoDimensionalNode,
    },
    error::TSPLibError,
    parser::{parse_integer, parse_real, split_once_newline, split_once_whitespace},
};

fn parse_positive_dimension(dimension: Option<Integer>) -> Result<usize, TSPLibError> {
    let dim = dimension.ok_or(TSPLibError::MissingDimension)?;
    if dim <= 0 {
        return Err(TSPLibError::InvalidDimension(Box::new(
            TSPLibError::InvalidIntegerFormat(dim.to_string()),
        )));
    }
    usize::try_from(dim).map_err(|_| {
        TSPLibError::InvalidDimension(Box::new(TSPLibError::InvalidIntegerFormat(dim.to_string())))
    })
}

fn parse_node_coord_section(
    input: &str,
    node_coord_type: Option<NodeCoordType>,
    dimension: Option<Integer>,
) -> Result<(NodeCoordSection, &str), TSPLibError> {
    let node_coord_type = if let Some(node_coord_type) = node_coord_type {
        node_coord_type
    } else {
        let (line, _) = split_once_newline(input);
        let count = line.split_whitespace().count();
        match count {
            3 => NodeCoordType::TwodCoords,
            4 => NodeCoordType::ThreedCoords,
            _ => return Err(TSPLibError::UnknownNodeCoordType),
        }
    };

    let dimension = parse_positive_dimension(dimension)?;

    let mut rest = input;
    match node_coord_type {
        NodeCoordType::TwodCoords => {
            let mut coords = Vec::with_capacity(dimension);
            for _ in 0..dimension {
                let (id, r) = parse_integer(rest)?;
                let (x, r) = parse_real(r)?;
                let (y, r) = parse_real(r)?;
                coords.push(TwoDimensionalNode {
                    identifier: id,
                    x_coordinate: x,
                    y_coordinate: y,
                });
                rest = r;
            }
            Ok((NodeCoordSection::TwodCoords(coords), rest))
        }
        NodeCoordType::ThreedCoords => {
            let mut coords = Vec::with_capacity(dimension);
            for _ in 0..dimension {
                let (id, r) = parse_integer(rest)?;
                let (x, r) = parse_real(r)?;
                let (y, r) = parse_real(r)?;
                let (z, r) = parse_real(r)?;
                coords.push(ThreeDimensionalNode {
                    identifier: id,
                    x_coordinate: x,
                    y_coordinate: y,
                    z_coordinate: z,
                });
                rest = r;
            }
            Ok((NodeCoordSection::ThreedCoords(coords), rest))
        }
        NodeCoordType::NoCoords => Err(TSPLibError::UnexpectedNodeCoordSection),
    }
}

fn parse_depot_section(input: &str) -> Result<(Vec<NodeIdentifier>, &str), TSPLibError> {
    let mut rest = input;
    let mut depots = Vec::new();
    loop {
        let (id, r) = parse_integer(rest)?;
        rest = r;
        if id == -1 {
            break;
        }
        depots.push(id);
    }
    Ok((depots, rest))
}

fn parse_demand_section(
    input: &str,
    dimension: Option<Integer>,
) -> Result<(Vec<Demand>, &str), TSPLibError> {
    let dimension = parse_positive_dimension(dimension)?;

    let mut rest = input;
    let mut demands = Vec::with_capacity(dimension);
    for _ in 0..dimension {
        let (id, r) = parse_integer(rest)?;
        let (demand, r) = parse_integer(r)?;
        demands.push((id, demand));
        rest = r;
    }
    Ok((demands, rest))
}

fn parse_edge_data_section(
    input: &str,
    edge_data_format: Option<EdgeDataFormat>,
) -> Result<(EdgeDataSection, &str), TSPLibError> {
    let Some(edge_data_format) = edge_data_format else {
        return Err(TSPLibError::MissingEdgeDataFormat);
    };

    let mut rest = input;
    match edge_data_format {
        EdgeDataFormat::EdgeList => {
            let mut edges = Vec::new();
            loop {
                let (u, r) = parse_integer(rest)?;
                if u == -1 {
                    rest = r;
                    break;
                }
                let (v, r) = parse_integer(r)?;
                edges.push((u, v));
                rest = r;
            }
            Ok((EdgeDataSection::EdgeList(edges), rest))
        }
        EdgeDataFormat::AdjList => {
            let mut adj = Vec::new();
            loop {
                let (u, r) = parse_integer(rest)?;
                if u == -1 {
                    rest = r;
                    break;
                }
                let mut neighbors = vec![u];
                rest = r;
                loop {
                    let (v, r) = parse_integer(rest)?;
                    rest = r;
                    if v == -1 {
                        break;
                    }
                    neighbors.push(v);
                }
                adj.push(neighbors);
            }
            Ok((EdgeDataSection::AdjList(adj), rest))
        }
    }
}

fn parse_fixed_data_section(input: &str) -> Result<(Vec<Edge>, &str), TSPLibError> {
    let mut rest = input;
    let mut edges = Vec::new();
    loop {
        let (u, r) = parse_integer(rest)?;
        if u == -1 {
            rest = r;
            break;
        }
        let (v, r) = parse_integer(r)?;
        edges.push((u, v));
        rest = r;
    }
    Ok((edges, rest))
}

fn parse_display_data_section(
    input: &str,
    dimension: Option<Integer>,
) -> Result<(Vec<TwoDimensionalNode>, &str), TSPLibError> {
    let dimension = parse_positive_dimension(dimension)?;

    let mut rest = input;
    let mut display_data = Vec::with_capacity(dimension);
    for _ in 0..dimension {
        let (id, r) = parse_integer(rest)?;
        let (x, r) = parse_real(r)?;
        let (y, r) = parse_real(r)?;
        display_data.push(TwoDimensionalNode {
            identifier: id,
            x_coordinate: x,
            y_coordinate: y,
        });
        rest = r;
    }
    Ok((display_data, rest))
}

fn parse_tour_section(
    input: &str,
    dimension: Option<Integer>,
) -> Result<(Vec<Vec<NodeIdentifier>>, &str), TSPLibError> {
    let mut rest = input;
    let mut tours = Vec::new();
    let initial_capacity = parse_positive_dimension(dimension).unwrap_or(0);
    let mut current_tour = Vec::with_capacity(initial_capacity);

    loop {
        let trimmed = rest.trim_start();
        if trimmed.is_empty() || trimmed.starts_with("EOF") {
            if !current_tour.is_empty() {
                tours.push(current_tour);
            }
            rest = trimmed;
            break;
        }

        let (id, r) = parse_integer(trimmed)?;
        rest = r;

        if id == -1 {
            if current_tour.is_empty() {
                // An additional -1 terminates the entire section per TSPLIB95 Section 1.2.7
                break;
            } else {
                tours.push(current_tour);
                current_tour = Vec::new();
            }
        } else {
            current_tour.push(id);
        }
    }

    Ok((tours, rest))
}

fn parse_edge_weight_section(
    input: &str,
    dimension: Option<Integer>,
    edge_weight_type: Option<EdgeWeightType>,
    edge_weight_format: Option<EdgeWeightFormat>,
) -> Result<(Vec<EdgeWeight>, &str), TSPLibError> {
    let edge_weight_format = match edge_weight_format {
        Some(fmt) => fmt,
        None if edge_weight_type == Some(EdgeWeightType::Explicit) => EdgeWeightFormat::FullMatrix,
        None => return Err(TSPLibError::MissingEdgeWeightFormat),
    };

    let n = parse_positive_dimension(dimension)?;

    // Some SOP instances contain additional redundant dimension header on the line following EDGE_WEIGHT_SECTION
    let input = if edge_weight_format == EdgeWeightFormat::FullMatrix {
        let (v, r) = split_once_newline(input);
        match parse_integer(v) {
            Ok((val, "")) if val == n as Integer => r,
            _ => input,
        }
    } else {
        input
    };

    let count = match edge_weight_format {
        EdgeWeightFormat::FullMatrix => n * n,
        EdgeWeightFormat::UpperRow
        | EdgeWeightFormat::LowerRow
        | EdgeWeightFormat::UpperCol
        | EdgeWeightFormat::LowerCol => n * (n - 1) / 2,
        EdgeWeightFormat::UpperDiagRow
        | EdgeWeightFormat::LowerDiagRow
        | EdgeWeightFormat::UpperDiagCol
        | EdgeWeightFormat::LowerDiagCol => n * (n + 1) / 2,
        EdgeWeightFormat::Function => 0,
    };

    let mut rest = input;
    let mut weights = Vec::with_capacity(count);
    for _ in 0..count {
        let (w, r) = parse_integer(rest)?;
        weights.push(w);
        rest = r;
    }
    Ok((weights, rest))
}

pub fn parse_data_section<'a>(
    input: &'a str,
    specification: &Specification,
) -> Result<(Data, &'a str), TSPLibError> {
    let mut node_coord_section = None;
    let mut depot_section = None;
    let mut demand_section = None;
    let mut edge_data_section = None;
    let mut fixed_edges_section = None;
    let mut display_data_section = None;
    let mut tour_section = None;
    let mut edge_weight_section = None;

    let mut rest = input;
    let mut keyword;
    loop {
        (keyword, rest) = split_once_whitespace(rest);
        match keyword {
            "NODE_COORD_SECTION" => {
                let (value, r) = parse_node_coord_section(
                    rest,
                    specification.node_coord_type,
                    specification.dimension,
                )?;
                node_coord_section = Some(value);
                rest = r;
            }
            "DEPOT_SECTION" => {
                let (value, r) = parse_depot_section(rest)?;
                depot_section = Some(value);
                rest = r;
            }
            "DEMAND_SECTION" => {
                let (value, r) = parse_demand_section(rest, specification.dimension)?;
                demand_section = Some(value);
                rest = r;
            }
            "EDGE_DATA_SECTION" => {
                let (value, r) = parse_edge_data_section(rest, specification.edge_data_format)?;
                edge_data_section = Some(value);
                rest = r;
            }
            "FIXED_EDGES_SECTION" | "FIXED_EDGES" => {
                // Some examples contain erroneous notation
                let (value, r) = if let (":", r) = split_once_whitespace(rest) {
                    parse_fixed_data_section(r)?
                } else {
                    parse_fixed_data_section(rest)?
                };

                fixed_edges_section = Some(value);
                rest = r;
            }
            "DISPLAY_DATA_SECTION" => {
                let (value, r) = parse_display_data_section(rest, specification.dimension)?;
                display_data_section = Some(value);
                rest = r;
            }
            "TOUR_SECTION" => {
                let (value, r) = parse_tour_section(rest, specification.dimension)?;
                tour_section = Some(value);
                rest = r;
            }
            "EDGE_WEIGHT_SECTION" => {
                let (value, r) = parse_edge_weight_section(
                    rest,
                    specification.dimension,
                    specification.edge_weight_type,
                    specification.edge_weight_format,
                )?;
                edge_weight_section = Some(value);
                rest = r;
            }
            "EOF" | "" => break,
            s => {
                return Err(TSPLibError::UnexpectedKeyword(s.to_string()));
            }
        }
    }

    let data = Data {
        node_coord_section,
        depot_section,
        demand_section,
        edge_data_section,
        fixed_edges_section,
        display_data_section,
        tour_section,
        edge_weight_section,
    };

    Ok((data, rest))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::datatypes::{DisplayDataType, EdgeWeightFormat, EdgeWeightType, Type};

    #[test]
    fn test_parse_node_coord_section() {
        let input = "1 1.0 2.0\n2 3.0 4.0";
        let (v, rest) =
            parse_node_coord_section(input, Some(NodeCoordType::TwodCoords), Some(2)).unwrap();
        if let NodeCoordSection::TwodCoords(coords) = v {
            assert_eq!(
                coords,
                vec![
                    TwoDimensionalNode {
                        identifier: 1,
                        x_coordinate: 1.0,
                        y_coordinate: 2.0
                    },
                    TwoDimensionalNode {
                        identifier: 2,
                        x_coordinate: 3.0,
                        y_coordinate: 4.0
                    }
                ]
            );
        } else {
            panic!("wrong variant");
        }
        assert_eq!(rest, "");

        let input = "1 1.0 2.0 3.0\n2 4.0 5.0 6.0";
        let (v, rest) =
            parse_node_coord_section(input, Some(NodeCoordType::ThreedCoords), Some(2)).unwrap();
        if let NodeCoordSection::ThreedCoords(coords) = v {
            assert_eq!(
                coords,
                vec![
                    ThreeDimensionalNode {
                        identifier: 1,
                        x_coordinate: 1.0,
                        y_coordinate: 2.0,
                        z_coordinate: 3.0
                    },
                    ThreeDimensionalNode {
                        identifier: 2,
                        x_coordinate: 4.0,
                        y_coordinate: 5.0,
                        z_coordinate: 6.0
                    }
                ]
            );
        } else {
            panic!("wrong variant");
        }
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_depot_section() {
        let input = "1 2 3 -1";
        let (v, rest) = parse_depot_section(input).unwrap();
        assert_eq!(v, vec![1, 2, 3]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_demand_section() {
        let input = "1 10\n2 20";
        let (v, rest) = parse_demand_section(input, Some(2)).unwrap();
        assert_eq!(v, vec![(1, 10), (2, 20)]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_edge_data_section() {
        let input = "1 2\n3 4\n-1";
        let (v, rest) = parse_edge_data_section(input, Some(EdgeDataFormat::EdgeList)).unwrap();
        if let EdgeDataSection::EdgeList(edges) = v {
            assert_eq!(edges, vec![(1, 2), (3, 4)]);
        } else {
            panic!("wrong variant");
        }
        assert_eq!(rest, "");

        let input = "1 2 3 -1\n4 5 -1\n-1";
        let (v, rest) = parse_edge_data_section(input, Some(EdgeDataFormat::AdjList)).unwrap();
        if let EdgeDataSection::AdjList(adj) = v {
            assert_eq!(adj, vec![vec![1, 2, 3], vec![4, 5]]);
        } else {
            panic!("wrong variant");
        }
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_fixed_data_section() {
        let input = "1 2\n3 4\n-1";
        let (v, rest) = parse_fixed_data_section(input).unwrap();
        assert_eq!(v, vec![(1, 2), (3, 4)]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_display_data_section() {
        let input = "1 1.0 2.0\n2 3.0 4.0";
        let (v, rest) = parse_display_data_section(input, Some(2)).unwrap();
        assert_eq!(
            v,
            vec![
                TwoDimensionalNode {
                    identifier: 1,
                    x_coordinate: 1.0,
                    y_coordinate: 2.0
                },
                TwoDimensionalNode {
                    identifier: 2,
                    x_coordinate: 3.0,
                    y_coordinate: 4.0
                }
            ]
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_tour_section() {
        // Single tour with terminator -1
        let input = "1 2 3 4 5 -1";
        let (v, rest) = parse_tour_section(input, Some(5)).unwrap();
        assert_eq!(v, vec![vec![1, 2, 3, 4, 5]]);
        assert_eq!(rest, "");

        // Single tour with section terminator -1 (TSPLIB95 Section 1.2.7)
        let input_double_term = "1 2 3 4 5 -1\n-1\nEOF";
        let (v, rest) = parse_tour_section(input_double_term, Some(5)).unwrap();
        assert_eq!(v, vec![vec![1, 2, 3, 4, 5]]);
        assert_eq!(rest.trim_start(), "EOF");

        // Collection of multiple tours
        let input_multi = "1 2 -1\n3 4 5 -1\n-1\nEOF";
        let (v, rest) = parse_tour_section(input_multi, Some(5)).unwrap();
        assert_eq!(v, vec![vec![1, 2], vec![3, 4, 5]]);
        assert_eq!(rest.trim_start(), "EOF");

        // Tour terminated by EOF
        let input_eof = "1 2 3\nEOF";
        let (v, rest) = parse_tour_section(input_eof, None).unwrap();
        assert_eq!(v, vec![vec![1, 2, 3]]);
        assert_eq!(rest, "EOF");
    }

    #[test]
    fn test_parse_edge_weight_section() {
        let input = "1 2 3 4";
        let (v, rest) = parse_edge_weight_section(
            input,
            Some(2),
            Some(EdgeWeightType::Explicit),
            Some(EdgeWeightFormat::FullMatrix),
        )
        .unwrap();
        assert_eq!(v, vec![1, 2, 3, 4]);
        assert_eq!(rest, "");

        // Default to FullMatrix when EdgeWeightType::Explicit and format is None
        let (v, rest) =
            parse_edge_weight_section(input, Some(2), Some(EdgeWeightType::Explicit), None)
                .unwrap();
        assert_eq!(v, vec![1, 2, 3, 4]);
        assert_eq!(rest, "");

        // Missing format when not Explicit
        assert_eq!(
            parse_edge_weight_section(input, Some(2), None, None),
            Err(TSPLibError::MissingEdgeWeightFormat)
        );

        let input = "1 2 3";
        let (v, rest) = parse_edge_weight_section(
            input,
            Some(3),
            Some(EdgeWeightType::Explicit),
            Some(EdgeWeightFormat::UpperRow),
        )
        .unwrap();
        // n=3, n*(n-1)/2 = 3*2/2 = 3
        assert_eq!(v, vec![1, 2, 3]);
        assert_eq!(rest, "");

        let input_diag = "1 2 3 4 5 6";
        let (v, rest) = parse_edge_weight_section(
            input_diag,
            Some(3),
            Some(EdgeWeightType::Explicit),
            Some(EdgeWeightFormat::LowerDiagRow),
        )
        .unwrap();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(rest, "");

        let (v, rest) = parse_edge_weight_section(
            "",
            Some(3),
            Some(EdgeWeightType::Euc2D),
            Some(EdgeWeightFormat::Function),
        )
        .unwrap();
        assert!(v.is_empty());
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_node_coord_section_inference() {
        let input = "1 1.0 2.0\n2 3.0 4.0";
        let (v, _) = parse_node_coord_section(input, None, Some(2)).unwrap();
        if let NodeCoordSection::TwodCoords(coords) = v {
            assert_eq!(
                coords,
                vec![
                    TwoDimensionalNode {
                        identifier: 1,
                        x_coordinate: 1.0,
                        y_coordinate: 2.0
                    },
                    TwoDimensionalNode {
                        identifier: 2,
                        x_coordinate: 3.0,
                        y_coordinate: 4.0
                    }
                ]
            );
        } else {
            panic!("should have inferred TwodCoords");
        }

        let input = "1 1.0 2.0 3.0\n2 4.0 5.0 6.0";
        let (v, _) = parse_node_coord_section(input, None, Some(2)).unwrap();
        if let NodeCoordSection::ThreedCoords(coords) = v {
            assert_eq!(
                coords,
                vec![
                    ThreeDimensionalNode {
                        identifier: 1,
                        x_coordinate: 1.0,
                        y_coordinate: 2.0,
                        z_coordinate: 3.0
                    },
                    ThreeDimensionalNode {
                        identifier: 2,
                        x_coordinate: 4.0,
                        y_coordinate: 5.0,
                        z_coordinate: 6.0
                    }
                ]
            );
        } else {
            panic!("should have inferred ThreedCoords");
        }
    }

    #[test]
    fn test_parse_node_coord_section_errors() {
        let input = "1 1.0 2.0";
        assert!(parse_node_coord_section(input, Some(NodeCoordType::TwodCoords), None).is_err());
        assert!(
            parse_node_coord_section(input, Some(NodeCoordType::TwodCoords), Some(-1)).is_err()
        );
        assert!(parse_node_coord_section(input, Some(NodeCoordType::NoCoords), Some(1)).is_err());
        let bad_input = "1 1.0";
        assert_eq!(
            parse_node_coord_section(bad_input, None, Some(1)),
            Err(TSPLibError::UnknownNodeCoordType)
        );
    }

    #[test]
    fn test_parse_demand_and_display_section_errors() {
        let input = "1 10";
        assert!(parse_demand_section(input, None).is_err());
        assert!(parse_display_data_section("1 1.0 2.0", None).is_err());
    }

    #[test]
    fn test_parse_edge_data_section_error() {
        let input = "1 2 -1";
        assert!(parse_edge_data_section(input, None).is_err());
    }

    #[test]
    fn test_parse_data_section_all_sections() {
        let spec = Specification {
            name: "test".into(),
            dtype: Type::TSP,
            comment: vec![],
            dimension: Some(2),
            capacity: Some(10),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::FullMatrix),
            edge_data_format: Some(EdgeDataFormat::EdgeList),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            display_data_type: Some(DisplayDataType::TwodDisplay),
        };
        let input = "\
NODE_COORD_SECTION\n1 1.0 2.0\n2 3.0 4.0\n\
DISPLAY_DATA_SECTION\n1 10.0 20.0\n2 30.0 40.0\n\
DEMAND_SECTION\n1 0\n2 5\n\
DEPOT_SECTION\n1 -1\n\
EDGE_DATA_SECTION\n1 2 -1\n\
FIXED_EDGES_SECTION\n1 2 -1\n\
EDGE_WEIGHT_SECTION\n0 7 7 0\n\
TOUR_SECTION\n1 2 -1\n\
EOF";
        let (data, rest) = parse_data_section(input, &spec).unwrap();
        assert!(data.node_coord_section.is_some());
        assert!(data.display_data_section.is_some());
        assert!(data.demand_section.is_some());
        assert!(data.depot_section.is_some());
        assert!(data.edge_data_section.is_some());
        assert!(data.fixed_edges_section.is_some());
        assert_eq!(data.edge_weight_section, Some(vec![0, 7, 7, 0]));
        assert_eq!(data.tour_section, Some(vec![vec![1, 2]]));
        assert_eq!(rest, "");

        // Test erroneous notation for FIXED_EDGES_SECTION
        let input = "FIXED_EDGES : \n1 2\n-1\nEOF";
        let (data, _) = parse_data_section(input, &spec).unwrap();
        assert_eq!(data.fixed_edges_section.unwrap(), vec![(1, 2)]);
    }

    #[test]
    fn test_parse_data_section_unexpected_keyword() {
        let spec = Specification {
            name: "test".into(),
            dtype: Type::TSP,
            comment: vec![],
            dimension: Some(2),
            capacity: Some(0),
            edge_weight_type: Some(EdgeWeightType::Euc2D),
            edge_weight_format: None,
            edge_data_format: None,
            node_coord_type: None,
            display_data_type: None,
        };
        let input = "UNKNOWN_SECTION\n1 2 3\nEOF";
        assert!(parse_data_section(input, &spec).is_err());
    }
}
