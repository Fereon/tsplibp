use std::collections::HashMap;

use crate::{
    datatypes::{
        EdgeDataSection, EdgeWeight, EdgeWeightFormat, EdgeWeightType, Instance, NodeCoordSection,
        NodeIdentifier, ThreeDimensionalNode, TwoDimensionalNode, Type,
    },
    error::TSPLibError,
};

/// Round half up to nearest integer according to TSPLIB95 specification: `(int) (x + 0.5)`.
#[inline]
pub fn nint(x: f64) -> EdgeWeight {
    (x + 0.5).floor() as EdgeWeight
}

/// Calculate 2D Euclidean distance according to TSPLIB95 EUC_2D:
/// `xd = x_i - x_j; yd = y_i - y_j; dij = nint(sqrt(xd*xd + yd*yd))`
pub fn distance_euc_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate) as f64;
    let yd = (n1.y_coordinate - n2.y_coordinate) as f64;
    nint((xd * xd + yd * yd).sqrt())
}

/// Calculate 3D Euclidean distance according to TSPLIB95 EUC_3D:
/// `xd = x_i - x_j; yd = y_i - y_j; zd = z_i - z_j; dij = nint(sqrt(xd*xd + yd*yd + zd*zd))`
pub fn distance_euc_3d(n1: &ThreeDimensionalNode, n2: &ThreeDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate) as f64;
    let yd = (n1.y_coordinate - n2.y_coordinate) as f64;
    let zd = (n1.z_coordinate - n2.z_coordinate) as f64;
    nint((xd * xd + yd * yd + zd * zd).sqrt())
}

/// Calculate 2D Manhattan distance according to TSPLIB95 MAN_2D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; dij = nint(xd) + nint(yd)`
pub fn distance_man_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = ((n1.x_coordinate - n2.x_coordinate) as f64).abs();
    let yd = ((n1.y_coordinate - n2.y_coordinate) as f64).abs();
    nint(xd) + nint(yd)
}

/// Calculate 3D Manhattan distance according to TSPLIB95 MAN_3D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; zd = |z_i - z_j|; dij = nint(xd) + nint(yd) + nint(zd)`
pub fn distance_man_3d(n1: &ThreeDimensionalNode, n2: &ThreeDimensionalNode) -> EdgeWeight {
    let xd = ((n1.x_coordinate - n2.x_coordinate) as f64).abs();
    let yd = ((n1.y_coordinate - n2.y_coordinate) as f64).abs();
    let zd = ((n1.z_coordinate - n2.z_coordinate) as f64).abs();
    nint(xd) + nint(yd) + nint(zd)
}

/// Calculate 2D Maximum distance according to TSPLIB95 MAX_2D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; dij = max(nint(xd), nint(yd))`
pub fn distance_max_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = ((n1.x_coordinate - n2.x_coordinate) as f64).abs();
    let yd = ((n1.y_coordinate - n2.y_coordinate) as f64).abs();
    nint(xd).max(nint(yd))
}

/// Calculate 3D Maximum distance according to TSPLIB95 MAX_3D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; zd = |z_i - z_j|; dij = max(nint(xd), max(nint(yd), nint(zd)))`
pub fn distance_max_3d(n1: &ThreeDimensionalNode, n2: &ThreeDimensionalNode) -> EdgeWeight {
    let xd = ((n1.x_coordinate - n2.x_coordinate) as f64).abs();
    let yd = ((n1.y_coordinate - n2.y_coordinate) as f64).abs();
    let zd = ((n1.z_coordinate - n2.z_coordinate) as f64).abs();
    nint(xd).max(nint(yd)).max(nint(zd))
}

/// Calculate 2D Ceil distance according to TSPLIB95 CEIL_2D:
/// `xd = x_i - x_j; yd = y_i - y_j; dij = (int) ceil(sqrt(xd*xd + yd*yd))`
pub fn distance_ceil_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate) as f64;
    let yd = (n1.y_coordinate - n2.y_coordinate) as f64;
    (xd * xd + yd * yd).sqrt().ceil() as EdgeWeight
}

/// Convert coordinate in degrees.minutes format to radians for TSPLIB95 GEO distance.
///
/// Note: TSPLIB95 explicitly specifies `PI = 3.141592`.
#[allow(clippy::approx_constant)]
fn geo_to_rad(coord: f64) -> f64 {
    let pi = 3.141592;
    let deg = coord as i32;
    let min = coord - deg as f64;
    pi * (deg as f64 + 5.0 * min / 3.0) / 180.0
}

/// Calculate Geographic distance according to TSPLIB95 GEO:
pub fn distance_geo(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let rrr = 6378.388;
    let lat_i = geo_to_rad(n1.x_coordinate as f64);
    let lon_i = geo_to_rad(n1.y_coordinate as f64);
    let lat_j = geo_to_rad(n2.x_coordinate as f64);
    let lon_j = geo_to_rad(n2.y_coordinate as f64);

    let q1 = (lon_i - lon_j).cos();
    let q2 = (lat_i - lat_j).cos();
    let q3 = (lat_i + lat_j).cos();

    (rrr * (0.5 * ((1.0 + q1) * q2 - (1.0 - q1) * q3)).acos() + 1.0) as EdgeWeight
}

/// Calculate Pseudo-Euclidean distance according to TSPLIB95 ATT:
pub fn distance_att(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate) as f64;
    let yd = (n1.y_coordinate - n2.y_coordinate) as f64;
    let rij = ((xd * xd + yd * yd) / 10.0).sqrt();
    let tij = nint(rij);
    if (tij as f64) < rij { tij + 1 } else { tij }
}

/// Helper structure for caching node lookups and matrix access.
pub struct WeightCalculator<'a> {
    instance: &'a Instance,
    twod_coords: Option<HashMap<NodeIdentifier, &'a TwoDimensionalNode>>,
    threed_coords: Option<HashMap<NodeIdentifier, &'a ThreeDimensionalNode>>,
    node_index_map: HashMap<NodeIdentifier, usize>,
}

impl<'a> WeightCalculator<'a> {
    pub fn new(instance: &'a Instance) -> Result<Self, TSPLibError> {
        let mut twod_coords = None;
        let mut threed_coords = None;
        let mut node_index_map = HashMap::new();

        if let Some(ref section) = instance.data.node_coord_section {
            match section {
                NodeCoordSection::TwodCoords(nodes) => {
                    let mut map = HashMap::with_capacity(nodes.len());
                    for (idx, node) in nodes.iter().enumerate() {
                        map.insert(node.identifier, node);
                        node_index_map.insert(node.identifier, idx);
                    }
                    twod_coords = Some(map);
                }
                NodeCoordSection::ThreedCoords(nodes) => {
                    let mut map = HashMap::with_capacity(nodes.len());
                    for (idx, node) in nodes.iter().enumerate() {
                        map.insert(node.identifier, node);
                        node_index_map.insert(node.identifier, idx);
                    }
                    threed_coords = Some(map);
                }
            }
        } else if let Some(dim) = instance.specification.dimension {
            // If there are no coordinates, default to 1..=dimension mapping
            for idx in 0..(dim as usize) {
                node_index_map.insert((idx + 1) as NodeIdentifier, idx);
            }
        }

        Ok(Self {
            instance,
            twod_coords,
            threed_coords,
            node_index_map,
        })
    }

    /// Retrieve the node index (0..dimension) for a given node identifier.
    pub fn node_index(&self, id: NodeIdentifier) -> Result<usize, TSPLibError> {
        self.node_index_map
            .get(&id)
            .copied()
            .ok_or(TSPLibError::NodeNotFound(id))
    }

    /// Calculate the weight between nodes `u` and `v`.
    pub fn weight(&self, u: NodeIdentifier, v: NodeIdentifier) -> Result<EdgeWeight, TSPLibError> {
        if u == v {
            return Ok(0);
        }

        let edge_weight_type = self
            .instance
            .specification
            .edge_weight_type
            .unwrap_or(EdgeWeightType::Explicit);

        match edge_weight_type {
            EdgeWeightType::Euc2D => {
                let coords = self.twod_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 2D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_euc_2d(n1, n2))
            }
            EdgeWeightType::Euc3D => {
                let coords = self.threed_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 3D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_euc_3d(n1, n2))
            }
            EdgeWeightType::Man2D => {
                let coords = self.twod_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 2D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_man_2d(n1, n2))
            }
            EdgeWeightType::Man3D => {
                let coords = self.threed_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 3D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_man_3d(n1, n2))
            }
            EdgeWeightType::Max2D => {
                let coords = self.twod_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 2D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_max_2d(n1, n2))
            }
            EdgeWeightType::Max3D => {
                let coords = self.threed_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 3D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_max_3d(n1, n2))
            }
            EdgeWeightType::Ceil2D => {
                let coords = self.twod_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 2D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_ceil_2d(n1, n2))
            }
            EdgeWeightType::Geo => {
                let coords = self.twod_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 2D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_geo(n1, n2))
            }
            EdgeWeightType::Att => {
                let coords = self.twod_coords.as_ref().ok_or_else(|| {
                    TSPLibError::WeightsNotAvailable("missing 2D coordinates".into())
                })?;
                let n1 = coords.get(&u).ok_or(TSPLibError::NodeNotFound(u))?;
                let n2 = coords.get(&v).ok_or(TSPLibError::NodeNotFound(v))?;
                Ok(distance_att(n1, n2))
            }
            EdgeWeightType::Explicit => self.explicit_weight(u, v),
            EdgeWeightType::Special | EdgeWeightType::XRay1 | EdgeWeightType::XRay2 => {
                // If explicit weights exist, fall back to them
                if self.instance.data.edge_weight_section.is_some() {
                    self.explicit_weight(u, v)
                } else {
                    Err(TSPLibError::UnsupportedWeightCalculation(format!(
                        "{:?}",
                        edge_weight_type
                    )))
                }
            }
        }
    }

    fn explicit_weight(
        &self,
        u: NodeIdentifier,
        v: NodeIdentifier,
    ) -> Result<EdgeWeight, TSPLibError> {
        let weights = match self.instance.data.edge_weight_section.as_ref() {
            Some(w) => w,
            None => {
                if self.instance.specification.dtype == Type::HCP {
                    return self.hcp_weight(u, v);
                }
                return Err(TSPLibError::WeightsNotAvailable(
                    "missing EDGE_WEIGHT_SECTION".into(),
                ));
            }
        };

        let dim = self
            .instance
            .specification
            .dimension
            .ok_or(TSPLibError::MissingDimension)? as usize;

        let format = self
            .instance
            .specification
            .edge_weight_format
            .unwrap_or(EdgeWeightFormat::FullMatrix);

        let i = self.node_index(u)?;
        let j = self.node_index(v)?;

        let index = match format {
            EdgeWeightFormat::FullMatrix => i * dim + j,
            EdgeWeightFormat::UpperRow => {
                let (r, c) = if i < j { (i, j) } else { (j, i) };
                r * dim - r * (r + 1) / 2 + (c - r - 1)
            }
            EdgeWeightFormat::LowerRow => {
                let (r, c) = if i > j { (i, j) } else { (j, i) };
                r * (r - 1) / 2 + c
            }
            EdgeWeightFormat::UpperDiagRow => {
                let (r, c) = if i <= j { (i, j) } else { (j, i) };
                r * dim - r * (r - 1) / 2 + (c - r)
            }
            EdgeWeightFormat::LowerDiagRow => {
                let (r, c) = if i >= j { (i, j) } else { (j, i) };
                r * (r + 1) / 2 + c
            }
            EdgeWeightFormat::UpperCol => {
                let (r, c) = if i < j { (i, j) } else { (j, i) };
                c * (c - 1) / 2 + r
            }
            EdgeWeightFormat::LowerCol => {
                let (r, c) = if i > j { (i, j) } else { (j, i) };
                c * dim - c * (c + 1) / 2 + (r - c - 1)
            }
            EdgeWeightFormat::UpperDiagCol => {
                let (r, c) = if i <= j { (i, j) } else { (j, i) };
                c * (c + 1) / 2 + r
            }
            EdgeWeightFormat::LowerDiagCol => {
                let (r, c) = if i >= j { (i, j) } else { (j, i) };
                c * dim - c * (c - 1) / 2 + (r - c)
            }
            EdgeWeightFormat::Function => {
                return Err(TSPLibError::UnsupportedWeightCalculation(
                    "EDGE_WEIGHT_FORMAT: FUNCTION specified without analytical coordinates".into(),
                ));
            }
        };

        weights.get(index).copied().ok_or_else(|| {
            TSPLibError::WeightsNotAvailable(format!("matrix index {index} out of bounds"))
        })
    }

    fn hcp_weight(&self, u: NodeIdentifier, v: NodeIdentifier) -> Result<EdgeWeight, TSPLibError> {
        if let Some(ref edge_data) = self.instance.data.edge_data_section {
            match edge_data {
                EdgeDataSection::EdgeList(edges) => {
                    let has_edge = edges
                        .iter()
                        .any(|&(a, b)| (a == u && b == v) || (a == v && b == u));
                    if has_edge {
                        Ok(1)
                    } else {
                        Ok(EdgeWeight::MAX / 2) // Represent infinity/unconnected
                    }
                }
                EdgeDataSection::AdjList(adj) => {
                    let has_edge = adj.iter().any(|list| {
                        if list.first() == Some(&u) {
                            list[1..].contains(&v)
                        } else if list.first() == Some(&v) {
                            list[1..].contains(&u)
                        } else {
                            false
                        }
                    });
                    if has_edge {
                        Ok(1)
                    } else {
                        Ok(EdgeWeight::MAX / 2)
                    }
                }
            }
        } else {
            Err(TSPLibError::WeightsNotAvailable(
                "no edge data found for HCP".into(),
            ))
        }
    }
}
