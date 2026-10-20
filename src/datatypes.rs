use std::{fmt::Display, io::Read, path::Path};

use crate::{error::TSPLibError, weights::WeightCalculator};

/// Integer type used for coordinates, counts, dimensions and weights in TSPLIB.
pub type Integer = i64;

/// Floating-point real type used for node coordinates.
pub type Real = f64;

/// Node identifier type. By convention in TSPLIB, node IDs are typically 1-indexed integers.
pub type NodeIdentifier = Integer;

/// An undirected or directed edge between two nodes `(from, to)`.
pub type Edge = (NodeIdentifier, NodeIdentifier);

/// Demand specification for CVRP: `(node_id, demand_quantity)`.
pub type Demand = (NodeIdentifier, Integer);

/// Weight / distance between two nodes.
pub type EdgeWeight = Integer;

/// Problem category / type specified in `TYPE:` header.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    /// Symmetric Traveling Salesperson Problem
    TSP,
    /// Asymmetric Traveling Salesperson Problem
    ATSP,
    /// Sequential Ordering Problem (TSP with precedence constraints)
    SOP,
    /// Hamiltonian Cycle Problem
    HCP,
    /// Capacitated Vehicle Routing Problem
    CVRP,
    /// Pre-computed tour / candidate solution
    Tour,
}

/// Distance metric specification in `EDGE_WEIGHT_TYPE:`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdgeWeightType {
    /// Explicit weights given in the `EDGE_WEIGHT_SECTION`
    Explicit,
    /// 2D Euclidean distance: `nint(sqrt(dx^2 + dy^2))`
    Euc2D,
    /// 3D Euclidean distance: `nint(sqrt(dx^2 + dy^2 + dz^2))`
    Euc3D,
    /// 2D Maximum metric: `max(nint(dx), nint(dy))`
    Max2D,
    /// 3D Maximum metric: `max(nint(dx), nint(dy), nint(dz))`
    Max3D,
    /// 2D Manhattan metric: `nint(dx) + nint(dy)`
    Man2D,
    /// 3D Manhattan metric: `nint(dx) + nint(dy) + nint(dz)`
    Man3D,
    /// 2D Ceiling Euclidean metric: `ceil(sqrt(dx^2 + dy^2))`
    Ceil2D,
    /// Geographic distance from latitude/longitude coordinates
    Geo,
    /// Pseudo-Euclidean distance metric used in ATT instances
    Att,
    /// Crystallography XRay1 metric
    XRay1,
    /// Crystallography XRay2 metric
    XRay2,
    /// Special user-defined metric
    Special,
}

/// Matrix storage format in `EDGE_WEIGHT_FORMAT:`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdgeWeightFormat {
    /// Weights are computed by a function from node coordinates
    Function,
    /// Full matrix of dimensions N x N
    FullMatrix,
    /// Upper triangular matrix row-wise without diagonal
    UpperRow,
    /// Lower triangular matrix row-wise without diagonal
    LowerRow,
    /// Upper triangular matrix row-wise with diagonal
    UpperDiagRow,
    /// Lower triangular matrix row-wise with diagonal
    LowerDiagRow,
    /// Upper triangular matrix column-wise without diagonal
    UpperCol,
    /// Lower triangular matrix column-wise without diagonal
    LowerCol,
    /// Upper triangular matrix column-wise with diagonal
    UpperDiagCol,
    /// Lower triangular matrix column-wise with diagonal
    LowerDiagCol,
}

/// Format of edge lists in `EDGE_DATA_FORMAT:`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdgeDataFormat {
    /// List of edge pairs (u, v) terminated by -1
    EdgeList,
    /// Adjacency list: node followed by neighbor list terminated by -1
    AdjList,
}

/// Coordinate dimensions in `NODE_COORD_TYPE:`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeCoordType {
    /// 2-dimensional coordinates
    TwodCoords,
    /// 3-dimensional coordinates
    ThreedCoords,
    /// Coordinates are not provided
    NoCoords,
}

/// Display data format in `DISPLAY_DATA_TYPE:`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayDataType {
    /// Display coordinates match node coordinates
    CoordDisplay,
    /// Explicit 2D display coordinates provided in `DISPLAY_DATA_SECTION`
    TwodDisplay,
    /// No graphical display coordinates
    NoDisplay,
}

/// Header specification of a TSPLIB file describing metadata and problem format.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Specification {
    /// Name of the instance (e.g. "berlin52", "eil51")
    pub name: String,
    /// Problem type (e.g. TSP, CVRP, ATSP)
    pub dtype: Type,
    /// Comments and problem annotations
    pub comment: Vec<String>,
    /// Number of nodes in the problem
    pub dimension: Option<Integer>,
    /// Vehicle capacity (for CVRP instances)
    pub capacity: Option<Integer>,
    /// How edge weights are specified or computed
    pub edge_weight_type: Option<EdgeWeightType>,
    /// Layout format of the explicit edge weight matrix
    pub edge_weight_format: Option<EdgeWeightFormat>,
    /// Format of edges (for HCP or unweighted graphs)
    pub edge_data_format: Option<EdgeDataFormat>,
    /// Dimensionality of node coordinates
    pub node_coord_type: Option<NodeCoordType>,
    /// Graphical display type
    pub display_data_type: Option<DisplayDataType>,
}

impl Display for Specification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:?}, dimension: {})",
            self.name,
            self.dtype,
            self.dimension.map_or("None".into(), |d| d.to_string())
        )
    }
}

/// A node with 2-dimensional coordinates.
#[derive(Debug, PartialEq, Clone)]
pub struct TwoDimensionalNode {
    /// Node identifier (usually 1-indexed)
    pub identifier: NodeIdentifier,
    /// X coordinate (or latitude in GEO format)
    pub x_coordinate: Real,
    /// Y coordinate (or longitude in GEO format)
    pub y_coordinate: Real,
}

/// A node with 3-dimensional coordinates.
#[derive(Debug, PartialEq, Clone)]
pub struct ThreeDimensionalNode {
    /// Node identifier (usually 1-indexed)
    pub identifier: NodeIdentifier,
    /// X coordinate
    pub x_coordinate: Real,
    /// Y coordinate
    pub y_coordinate: Real,
    /// Z coordinate
    pub z_coordinate: Real,
}

/// Collection of node coordinates parsed from `NODE_COORD_SECTION`.
#[derive(Debug, PartialEq, Clone)]
pub enum NodeCoordSection {
    /// List of 2D coordinates
    TwodCoords(Vec<TwoDimensionalNode>),
    /// List of 3D coordinates
    ThreedCoords(Vec<ThreeDimensionalNode>),
}

/// Section containing unweighted graph edges (used for HCP).
#[derive(Debug, PartialEq, Clone)]
pub enum EdgeDataSection {
    /// Pairs of node identifiers
    EdgeList(Vec<Edge>),
    /// Adjacency list per node
    AdjList(Vec<Vec<Integer>>),
}

/// Data sections containing coordinates, weights, depots, demands, or tours.
#[derive(Debug, PartialEq, Clone)]
pub struct Data {
    /// Coordinates of nodes, if present
    pub node_coord_section: Option<NodeCoordSection>,
    /// Depot node identifiers for vehicle routing (CVRP)
    pub depot_section: Option<Vec<NodeIdentifier>>,
    /// Customer demands for vehicle routing (CVRP)
    pub demand_section: Option<Vec<Demand>>,
    /// Unweighted edge list or adjacency list (HCP)
    pub edge_data_section: Option<EdgeDataSection>,
    /// Fixed edges that must appear in solutions
    pub fixed_edges_section: Option<Vec<Edge>>,
    /// 2D display coordinates for visualization
    pub display_data_section: Option<Vec<TwoDimensionalNode>>,
    /// Node sequence of a tour
    pub tour_section: Option<Vec<NodeIdentifier>>,
    /// Raw explicit edge weights stored flat
    pub edge_weight_section: Option<Vec<EdgeWeight>>,
}

/// A complete TSPLIB problem instance consisting of metadata specification and data sections.
#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    /// Metadata header information
    pub specification: Specification,
    /// Concrete problem data (coordinates, weights, depots, etc.)
    pub data: Data,
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSPLIB Instance: {}", self.specification)
    }
}

impl Instance {
    /// Parse an instance directly from a file path.
    ///
    /// # Errors
    /// Returns a [`TSPLibError`] if the file cannot be read or if parsing fails.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, TSPLibError> {
        let content = std::fs::read_to_string(path)?;
        content.parse()
    }

    /// Parse an instance from any stream implementing [`std::io::Read`].
    ///
    /// # Errors
    /// Returns a [`TSPLibError`] on I/O error or if parsing fails.
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, TSPLibError> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        content.parse()
    }

    /// Return the dimension of the problem (number of nodes).
    ///
    /// # Errors
    /// Returns [`TSPLibError::MissingDimension`] if the dimension is not specified in the header,
    /// or [`TSPLibError::InvalidDimension`] if the dimension is not positive.
    pub fn dimension(&self) -> Result<usize, TSPLibError> {
        let dim = self
            .specification
            .dimension
            .ok_or(TSPLibError::MissingDimension)?;
        if dim <= 0 {
            return Err(TSPLibError::InvalidDimension(Box::new(
                TSPLibError::InvalidIntegerFormat(dim.to_string()),
            )));
        }
        usize::try_from(dim).map_err(|_| {
            TSPLibError::InvalidDimension(Box::new(TSPLibError::InvalidIntegerFormat(
                dim.to_string(),
            )))
        })
    }

    /// Return an ordered list of all node identifiers in this instance.
    ///
    /// If coordinates are provided, the identifiers are extracted in order.
    /// Otherwise, `1..=dimension` is assumed.
    pub fn node_identifiers(&self) -> Result<Vec<NodeIdentifier>, TSPLibError> {
        if let Some(ref section) = self.data.node_coord_section {
            match section {
                NodeCoordSection::TwodCoords(nodes) => {
                    return Ok(nodes.iter().map(|n| n.identifier).collect());
                }
                NodeCoordSection::ThreedCoords(nodes) => {
                    return Ok(nodes.iter().map(|n| n.identifier).collect());
                }
            }
        }
        let dim = self.dimension()?;
        Ok((1..=(dim as NodeIdentifier)).collect())
    }

    /// Create a [`WeightCalculator`] for repeated $O(1)$ distance queries on this instance.
    pub fn weight_calculator(&self) -> Result<WeightCalculator<'_>, TSPLibError> {
        WeightCalculator::new(self)
    }

    /// Calculate or look up the edge weight between two nodes `u` and `v`.
    ///
    /// Automatically handles both explicit weight matrices (in any standard TSPLIB format)
    /// and analytical distance functions (`EUC_2D`, `GEO`, `ATT`, etc.).
    ///
    /// # Errors
    /// Returns a [`TSPLibError`] if either node is not found, or if weights cannot be computed.
    pub fn weight(&self, u: NodeIdentifier, v: NodeIdentifier) -> Result<EdgeWeight, TSPLibError> {
        crate::weights::weight_direct(self, u, v)
    }

    /// Precompute and return a full `dimension x dimension` distance matrix.
    ///
    /// The matrix is indexed by node position (`0..dimension`).
    pub fn distance_matrix(&self) -> Result<Vec<Vec<EdgeWeight>>, TSPLibError> {
        let calculator = WeightCalculator::new(self)?;
        let nodes = self.node_identifiers()?;
        let mut matrix = Vec::with_capacity(nodes.len());
        for &u in &nodes {
            let mut row = Vec::with_capacity(nodes.len());
            for &v in &nodes {
                row.push(calculator.weight(u, v)?);
            }
            matrix.push(row);
        }
        Ok(matrix)
    }

    /// Calculate the total length of a TSP round-trip tour visiting all nodes in `tour`.
    ///
    /// The tour is assumed to form a cycle: `u_0 -> u_1 -> ... -> u_{k-1} -> u_0`.
    ///
    /// # Errors
    /// Returns a [`TSPLibError`] if the tour has fewer than 2 nodes or if edge weights cannot be computed.
    pub fn tour_length(&self, tour: &[NodeIdentifier]) -> Result<EdgeWeight, TSPLibError> {
        if tour.len() < 2 {
            return Err(TSPLibError::InvalidTour(
                "tour must contain at least 2 nodes".into(),
            ));
        }

        let calculator = WeightCalculator::new(self)?;
        let mut total_length: EdgeWeight = 0;

        for window in tour.windows(2) {
            total_length = total_length.saturating_add(calculator.weight(window[0], window[1])?);
        }
        // Return edge from last node to first node
        total_length =
            total_length.saturating_add(calculator.weight(tour[tour.len() - 1], tour[0])?);

        Ok(total_length)
    }

    /// Validate whether a given sequence of nodes forms a valid Hamiltonian cycle / TSP tour.
    ///
    /// Checks that the tour length matches `dimension`, contains no duplicates, and every node
    /// is a valid node identifier in the problem.
    pub fn is_valid_tour(&self, tour: &[NodeIdentifier]) -> Result<bool, TSPLibError> {
        let dim = self.dimension()?;
        if tour.len() != dim {
            return Ok(false);
        }

        let mut seen = std::collections::HashSet::with_capacity(dim);
        let valid_nodes: std::collections::HashSet<_> =
            self.node_identifiers()?.into_iter().collect();

        for &node in tour {
            if !valid_nodes.contains(&node) || !seen.insert(node) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// If this instance represents a tour file (`TYPE: TOUR`), returns the tour section slice.
    pub fn tour(&self) -> Option<&[NodeIdentifier]> {
        self.data.tour_section.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_data() -> Data {
        Data {
            node_coord_section: None,
            depot_section: None,
            demand_section: None,
            edge_data_section: None,
            fixed_edges_section: None,
            display_data_section: None,
            tour_section: None,
            edge_weight_section: None,
        }
    }

    fn base_spec(dimension: Option<Integer>) -> Specification {
        Specification {
            name: "unit".into(),
            dtype: Type::TSP,
            comment: vec![],
            dimension,
            capacity: None,
            edge_weight_type: Some(EdgeWeightType::Euc2D),
            edge_weight_format: None,
            edge_data_format: None,
            node_coord_type: None,
            display_data_type: None,
        }
    }

    #[test]
    fn test_dimension_validation() {
        let inst_missing = Instance {
            specification: base_spec(None),
            data: empty_data(),
        };
        assert_eq!(inst_missing.dimension(), Err(TSPLibError::MissingDimension));

        let inst_neg = Instance {
            specification: base_spec(Some(-3)),
            data: empty_data(),
        };
        assert!(matches!(
            inst_neg.dimension(),
            Err(TSPLibError::InvalidDimension(_))
        ));

        let inst_ok = Instance {
            specification: base_spec(Some(4)),
            data: empty_data(),
        };
        assert_eq!(inst_ok.dimension(), Ok(4));
    }

    #[test]
    fn test_node_identifiers_2d_3d_and_fallback() {
        // Fallback 1..=dimension
        let inst_fallback = Instance {
            specification: base_spec(Some(3)),
            data: empty_data(),
        };
        assert_eq!(inst_fallback.node_identifiers(), Ok(vec![1, 2, 3]));

        // 2D custom identifiers
        let mut data_2d = empty_data();
        data_2d.node_coord_section = Some(NodeCoordSection::TwodCoords(vec![
            TwoDimensionalNode {
                identifier: 10,
                x_coordinate: 0.0,
                y_coordinate: 0.0,
            },
            TwoDimensionalNode {
                identifier: 20,
                x_coordinate: 3.0,
                y_coordinate: 4.0,
            },
        ]));
        let inst_2d = Instance {
            specification: base_spec(Some(2)),
            data: data_2d,
        };
        assert_eq!(inst_2d.node_identifiers(), Ok(vec![10, 20]));

        // 3D custom identifiers
        let mut data_3d = empty_data();
        data_3d.node_coord_section = Some(NodeCoordSection::ThreedCoords(vec![
            ThreeDimensionalNode {
                identifier: 7,
                x_coordinate: 0.0,
                y_coordinate: 0.0,
                z_coordinate: 0.0,
            },
            ThreeDimensionalNode {
                identifier: 9,
                x_coordinate: 1.0,
                y_coordinate: 2.0,
                z_coordinate: 2.0,
            },
        ]));
        let inst_3d = Instance {
            specification: base_spec(Some(2)),
            data: data_3d,
        };
        assert_eq!(inst_3d.node_identifiers(), Ok(vec![7, 9]));
    }

    #[test]
    fn test_tour_validation_and_length_custom_ids() {
        let mut data = empty_data();
        data.node_coord_section = Some(NodeCoordSection::TwodCoords(vec![
            TwoDimensionalNode {
                identifier: 10,
                x_coordinate: 0.0,
                y_coordinate: 0.0,
            },
            TwoDimensionalNode {
                identifier: 20,
                x_coordinate: 3.0,
                y_coordinate: 4.0,
            },
            TwoDimensionalNode {
                identifier: 30,
                x_coordinate: 0.0,
                y_coordinate: 4.0,
            },
        ]));
        data.tour_section = Some(vec![10, 20, 30]);
        let inst = Instance {
            specification: base_spec(Some(3)),
            data,
        };

        assert_eq!(inst.tour(), Some(&[10, 20, 30][..]));
        assert_eq!(inst.is_valid_tour(&[10, 20, 30]), Ok(true));
        assert_eq!(inst.is_valid_tour(&[1, 2, 3]), Ok(false));
        assert_eq!(inst.is_valid_tour(&[10, 20, 20]), Ok(false));
        assert_eq!(inst.is_valid_tour(&[10, 20]), Ok(false));

        assert_eq!(inst.tour_length(&[10, 20, 30]), Ok(5 + 3 + 4));
        assert!(matches!(
            inst.tour_length(&[]),
            Err(TSPLibError::InvalidTour(_))
        ));
    }

    #[test]
    fn test_display_without_dimension() {
        let spec = base_spec(None);
        assert_eq!(spec.to_string(), "unit (TSP, dimension: None)");
    }

    #[test]
    fn test_from_reader_in_memory() {
        let content = b"NAME: mem\nTYPE: TOUR\nDIMENSION: 2\nTOUR_SECTION\n1 2 -1\nEOF";
        let inst = Instance::from_reader(&content[..]).unwrap();
        assert_eq!(inst.specification.name, "mem");
        assert_eq!(inst.tour(), Some(&[1, 2][..]));
    }
}
