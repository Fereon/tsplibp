use std::collections::{HashMap, HashSet};

use crate::{
    datatypes::{
        EdgeDataSection, EdgeWeight, EdgeWeightFormat, EdgeWeightType, Instance, NodeCoordSection,
        NodeIdentifier, ThreeDimensionalNode, TwoDimensionalNode, Type,
    },
    error::TSPLibError,
};

/// Weight returned for non-adjacent nodes in Hamiltonian Cycle Problem (`HCP`) instances.
pub const HCP_NON_EDGE_WEIGHT: EdgeWeight = i32::MAX as EdgeWeight / 2;

/// Round half up to nearest integer according to TSPLIB95 specification: `(int) (x + 0.5)`.
#[inline]
pub fn nint(x: f64) -> EdgeWeight {
    (x + 0.5).floor() as EdgeWeight
}

/// Calculate 2D Euclidean distance according to TSPLIB95 EUC_2D:
/// `xd = x_i - x_j; yd = y_i - y_j; dij = nint(sqrt(xd*xd + yd*yd))`
pub fn distance_euc_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = n1.x_coordinate - n2.x_coordinate;
    let yd = n1.y_coordinate - n2.y_coordinate;
    nint((xd * xd + yd * yd).sqrt())
}

/// Calculate 3D Euclidean distance according to TSPLIB95 EUC_3D:
/// `xd = x_i - x_j; yd = y_i - y_j; zd = z_i - z_j; dij = nint(sqrt(xd*xd + yd*yd + zd*zd))`
pub fn distance_euc_3d(n1: &ThreeDimensionalNode, n2: &ThreeDimensionalNode) -> EdgeWeight {
    let xd = n1.x_coordinate - n2.x_coordinate;
    let yd = n1.y_coordinate - n2.y_coordinate;
    let zd = n1.z_coordinate - n2.z_coordinate;
    nint((xd * xd + yd * yd + zd * zd).sqrt())
}

/// Calculate 2D Manhattan distance according to TSPLIB95 MAN_2D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; dij = nint(xd) + nint(yd)`
pub fn distance_man_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate).abs();
    let yd = (n1.y_coordinate - n2.y_coordinate).abs();
    nint(xd) + nint(yd)
}

/// Calculate 3D Manhattan distance according to TSPLIB95 MAN_3D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; zd = |z_i - z_j|; dij = nint(xd) + nint(yd) + nint(zd)`
pub fn distance_man_3d(n1: &ThreeDimensionalNode, n2: &ThreeDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate).abs();
    let yd = (n1.y_coordinate - n2.y_coordinate).abs();
    let zd = (n1.z_coordinate - n2.z_coordinate).abs();
    nint(xd) + nint(yd) + nint(zd)
}

/// Calculate 2D Maximum distance according to TSPLIB95 MAX_2D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; dij = max(nint(xd), nint(yd))`
pub fn distance_max_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate).abs();
    let yd = (n1.y_coordinate - n2.y_coordinate).abs();
    nint(xd).max(nint(yd))
}

/// Calculate 3D Maximum distance according to TSPLIB95 MAX_3D:
/// `xd = |x_i - x_j|; yd = |y_i - y_j|; zd = |z_i - z_j|; dij = max(nint(xd), max(nint(yd), nint(zd)))`
pub fn distance_max_3d(n1: &ThreeDimensionalNode, n2: &ThreeDimensionalNode) -> EdgeWeight {
    let xd = (n1.x_coordinate - n2.x_coordinate).abs();
    let yd = (n1.y_coordinate - n2.y_coordinate).abs();
    let zd = (n1.z_coordinate - n2.z_coordinate).abs();
    nint(xd).max(nint(yd)).max(nint(zd))
}

/// Calculate 2D Ceil distance according to TSPLIB95 CEIL_2D:
/// `xd = x_i - x_j; yd = y_i - y_j; dij = (int) ceil(sqrt(xd*xd + yd*yd))`
pub fn distance_ceil_2d(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = n1.x_coordinate - n2.x_coordinate;
    let yd = n1.y_coordinate - n2.y_coordinate;
    (xd * xd + yd * yd).sqrt().ceil() as EdgeWeight
}

/// Convert coordinate in degrees.minutes format to radians for TSPLIB95 GEO distance.
///
/// Note: TSPLIB95 explicitly specifies `PI = 3.141592`.
#[allow(clippy::approx_constant)]
fn geo_to_rad(coord: f64) -> f64 {
    let pi = 3.141592;
    let deg = coord.trunc();
    let min = coord - deg;
    pi * (deg + 5.0 * min / 3.0) / 180.0
}

/// Calculate Geographic distance according to TSPLIB95 GEO:
pub fn distance_geo(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let rrr = 6378.388;
    let lat_i = geo_to_rad(n1.x_coordinate);
    let lon_i = geo_to_rad(n1.y_coordinate);
    let lat_j = geo_to_rad(n2.x_coordinate);
    let lon_j = geo_to_rad(n2.y_coordinate);

    let q1 = (lon_i - lon_j).cos();
    let q2 = (lat_i - lat_j).cos();
    let q3 = (lat_i + lat_j).cos();

    let arg = (0.5 * ((1.0 + q1) * q2 - (1.0 - q1) * q3)).clamp(-1.0, 1.0);
    (rrr * arg.acos() + 1.0) as EdgeWeight
}

/// Calculate Pseudo-Euclidean distance according to TSPLIB95 ATT:
pub fn distance_att(n1: &TwoDimensionalNode, n2: &TwoDimensionalNode) -> EdgeWeight {
    let xd = n1.x_coordinate - n2.x_coordinate;
    let yd = n1.y_coordinate - n2.y_coordinate;
    let rij = ((xd * xd + yd * yd) / 10.0).sqrt();
    let tij = nint(rij);
    if (tij as f64) < rij { tij + 1 } else { tij }
}

/// Fast single-pair weight calculation without allocating lookup maps when node IDs are standard 1-indexed.
pub(crate) fn weight_direct(
    instance: &Instance,
    u: NodeIdentifier,
    v: NodeIdentifier,
) -> Result<EdgeWeight, TSPLibError> {
    if u == v {
        return Ok(0);
    }

    let edge_weight_type = instance
        .specification
        .edge_weight_type
        .unwrap_or(EdgeWeightType::Explicit);

    match edge_weight_type {
        EdgeWeightType::Euc2D
        | EdgeWeightType::Man2D
        | EdgeWeightType::Max2D
        | EdgeWeightType::Ceil2D
        | EdgeWeightType::Geo
        | EdgeWeightType::Att => {
            let Some(NodeCoordSection::TwodCoords(nodes)) =
                instance.data.node_coord_section.as_ref()
            else {
                return Err(TSPLibError::WeightsNotAvailable(
                    "missing 2D coordinates".into(),
                ));
            };
            let n1 = find_2d_node(nodes, u)?;
            let n2 = find_2d_node(nodes, v)?;
            Ok(compute_2d_distance(edge_weight_type, n1, n2))
        }
        EdgeWeightType::Euc3D | EdgeWeightType::Man3D | EdgeWeightType::Max3D => {
            let Some(NodeCoordSection::ThreedCoords(nodes)) =
                instance.data.node_coord_section.as_ref()
            else {
                return Err(TSPLibError::WeightsNotAvailable(
                    "missing 3D coordinates".into(),
                ));
            };
            let n1 = find_3d_node(nodes, u)?;
            let n2 = find_3d_node(nodes, v)?;
            Ok(compute_3d_distance(edge_weight_type, n1, n2))
        }
        EdgeWeightType::Explicit => {
            let i = find_node_index(instance, u)?;
            let j = find_node_index(instance, v)?;
            explicit_weight_by_index(instance, u, v, i, j, None)
        }
        EdgeWeightType::Special | EdgeWeightType::XRay1 | EdgeWeightType::XRay2 => {
            if instance.data.edge_weight_section.is_some() {
                let i = find_node_index(instance, u)?;
                let j = find_node_index(instance, v)?;
                explicit_weight_by_index(instance, u, v, i, j, None)
            } else {
                Err(TSPLibError::UnsupportedWeightCalculation(format!(
                    "{:?}",
                    edge_weight_type
                )))
            }
        }
    }
}

#[inline]
fn find_2d_node(
    nodes: &[TwoDimensionalNode],
    id: NodeIdentifier,
) -> Result<&TwoDimensionalNode, TSPLibError> {
    if id >= 1
        && let Some(node) = nodes.get((id - 1) as usize)
        && node.identifier == id
    {
        return Ok(node);
    }
    nodes
        .iter()
        .find(|n| n.identifier == id)
        .ok_or(TSPLibError::NodeNotFound(id))
}

#[inline]
fn find_3d_node(
    nodes: &[ThreeDimensionalNode],
    id: NodeIdentifier,
) -> Result<&ThreeDimensionalNode, TSPLibError> {
    if id >= 1
        && let Some(node) = nodes.get((id - 1) as usize)
        && node.identifier == id
    {
        return Ok(node);
    }
    nodes
        .iter()
        .find(|n| n.identifier == id)
        .ok_or(TSPLibError::NodeNotFound(id))
}

#[inline]
fn find_node_index(instance: &Instance, id: NodeIdentifier) -> Result<usize, TSPLibError> {
    if let Some(ref section) = instance.data.node_coord_section {
        match section {
            NodeCoordSection::TwodCoords(nodes) => {
                if id >= 1
                    && let Some(node) = nodes.get((id - 1) as usize)
                    && node.identifier == id
                {
                    return Ok((id - 1) as usize);
                }
                return nodes
                    .iter()
                    .position(|n| n.identifier == id)
                    .ok_or(TSPLibError::NodeNotFound(id));
            }
            NodeCoordSection::ThreedCoords(nodes) => {
                if id >= 1
                    && let Some(node) = nodes.get((id - 1) as usize)
                    && node.identifier == id
                {
                    return Ok((id - 1) as usize);
                }
                return nodes
                    .iter()
                    .position(|n| n.identifier == id)
                    .ok_or(TSPLibError::NodeNotFound(id));
            }
        }
    }
    let dim = instance.dimension()?;
    if id >= 1 && (id as usize) <= dim {
        Ok((id - 1) as usize)
    } else {
        Err(TSPLibError::NodeNotFound(id))
    }
}

#[inline]
fn compute_2d_distance(
    edge_weight_type: EdgeWeightType,
    n1: &TwoDimensionalNode,
    n2: &TwoDimensionalNode,
) -> EdgeWeight {
    match edge_weight_type {
        EdgeWeightType::Euc2D => distance_euc_2d(n1, n2),
        EdgeWeightType::Man2D => distance_man_2d(n1, n2),
        EdgeWeightType::Max2D => distance_max_2d(n1, n2),
        EdgeWeightType::Ceil2D => distance_ceil_2d(n1, n2),
        EdgeWeightType::Geo => distance_geo(n1, n2),
        EdgeWeightType::Att => distance_att(n1, n2),
        _ => unreachable!(),
    }
}

#[inline]
fn compute_3d_distance(
    edge_weight_type: EdgeWeightType,
    n1: &ThreeDimensionalNode,
    n2: &ThreeDimensionalNode,
) -> EdgeWeight {
    match edge_weight_type {
        EdgeWeightType::Euc3D => distance_euc_3d(n1, n2),
        EdgeWeightType::Man3D => distance_man_3d(n1, n2),
        EdgeWeightType::Max3D => distance_max_3d(n1, n2),
        _ => unreachable!(),
    }
}

/// Helper structure for caching node lookups and matrix access.
pub struct WeightCalculator<'a> {
    instance: &'a Instance,
    /// True when node identifiers are contiguous `1..=dimension`.
    is_contiguous_one_based: bool,
    dimension: usize,
    twod_coords: Option<HashMap<NodeIdentifier, &'a TwoDimensionalNode>>,
    threed_coords: Option<HashMap<NodeIdentifier, &'a ThreeDimensionalNode>>,
    node_index_map: Option<HashMap<NodeIdentifier, usize>>,
    hcp_edges: Option<HashSet<(NodeIdentifier, NodeIdentifier)>>,
}

impl<'a> WeightCalculator<'a> {
    pub fn new(instance: &'a Instance) -> Result<Self, TSPLibError> {
        let mut twod_coords = None;
        let mut threed_coords = None;
        let mut node_index_map = None;
        let mut is_contiguous_one_based = true;
        let mut dimension = instance
            .specification
            .dimension
            .and_then(|d| usize::try_from(d).ok())
            .unwrap_or(0);

        if let Some(ref section) = instance.data.node_coord_section {
            match section {
                NodeCoordSection::TwodCoords(nodes) => {
                    dimension = nodes.len();
                    is_contiguous_one_based = nodes
                        .iter()
                        .enumerate()
                        .all(|(idx, n)| n.identifier == (idx + 1) as NodeIdentifier);
                    if !is_contiguous_one_based {
                        let mut map = HashMap::with_capacity(nodes.len());
                        let mut idx_map = HashMap::with_capacity(nodes.len());
                        for (idx, node) in nodes.iter().enumerate() {
                            map.insert(node.identifier, node);
                            idx_map.insert(node.identifier, idx);
                        }
                        twod_coords = Some(map);
                        node_index_map = Some(idx_map);
                    }
                }
                NodeCoordSection::ThreedCoords(nodes) => {
                    dimension = nodes.len();
                    is_contiguous_one_based = nodes
                        .iter()
                        .enumerate()
                        .all(|(idx, n)| n.identifier == (idx + 1) as NodeIdentifier);
                    if !is_contiguous_one_based {
                        let mut map = HashMap::with_capacity(nodes.len());
                        let mut idx_map = HashMap::with_capacity(nodes.len());
                        for (idx, node) in nodes.iter().enumerate() {
                            map.insert(node.identifier, node);
                            idx_map.insert(node.identifier, idx);
                        }
                        threed_coords = Some(map);
                        node_index_map = Some(idx_map);
                    }
                }
            }
        }

        let hcp_edges = if let Some(ref edge_data) = instance.data.edge_data_section {
            let mut set = HashSet::new();
            match edge_data {
                EdgeDataSection::EdgeList(edges) => {
                    set.reserve(edges.len());
                    for &(u, v) in edges {
                        let pair = if u <= v { (u, v) } else { (v, u) };
                        set.insert(pair);
                    }
                }
                EdgeDataSection::AdjList(adj) => {
                    for list in adj {
                        if let Some(&u) = list.first() {
                            for &v in &list[1..] {
                                let pair = if u <= v { (u, v) } else { (v, u) };
                                set.insert(pair);
                            }
                        }
                    }
                }
            }
            Some(set)
        } else {
            None
        };

        Ok(Self {
            instance,
            is_contiguous_one_based,
            dimension,
            twod_coords,
            threed_coords,
            node_index_map,
            hcp_edges,
        })
    }

    /// Retrieve the node index (0..dimension) for a given node identifier.
    #[inline]
    pub fn node_index(&self, id: NodeIdentifier) -> Result<usize, TSPLibError> {
        if self.is_contiguous_one_based {
            if id >= 1 && (id as usize) <= self.dimension {
                return Ok((id - 1) as usize);
            } else {
                return Err(TSPLibError::NodeNotFound(id));
            }
        }
        self.node_index_map
            .as_ref()
            .and_then(|m| m.get(&id).copied())
            .ok_or(TSPLibError::NodeNotFound(id))
    }

    #[inline]
    fn get_2d_node(&self, id: NodeIdentifier) -> Result<&'a TwoDimensionalNode, TSPLibError> {
        let Some(NodeCoordSection::TwodCoords(nodes)) =
            self.instance.data.node_coord_section.as_ref()
        else {
            return Err(TSPLibError::WeightsNotAvailable(
                "missing 2D coordinates".into(),
            ));
        };
        if self.is_contiguous_one_based {
            if id >= 1
                && let Some(node) = nodes.get((id - 1) as usize)
            {
                return Ok(node);
            }
            return Err(TSPLibError::NodeNotFound(id));
        }
        self.twod_coords
            .as_ref()
            .and_then(|m| m.get(&id).copied())
            .ok_or(TSPLibError::NodeNotFound(id))
    }

    #[inline]
    fn get_3d_node(&self, id: NodeIdentifier) -> Result<&'a ThreeDimensionalNode, TSPLibError> {
        let Some(NodeCoordSection::ThreedCoords(nodes)) =
            self.instance.data.node_coord_section.as_ref()
        else {
            return Err(TSPLibError::WeightsNotAvailable(
                "missing 3D coordinates".into(),
            ));
        };
        if self.is_contiguous_one_based {
            if id >= 1
                && let Some(node) = nodes.get((id - 1) as usize)
            {
                return Ok(node);
            }
            return Err(TSPLibError::NodeNotFound(id));
        }
        self.threed_coords
            .as_ref()
            .and_then(|m| m.get(&id).copied())
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
            EdgeWeightType::Euc2D
            | EdgeWeightType::Man2D
            | EdgeWeightType::Max2D
            | EdgeWeightType::Ceil2D
            | EdgeWeightType::Geo
            | EdgeWeightType::Att => {
                let n1 = self.get_2d_node(u)?;
                let n2 = self.get_2d_node(v)?;
                Ok(compute_2d_distance(edge_weight_type, n1, n2))
            }
            EdgeWeightType::Euc3D | EdgeWeightType::Man3D | EdgeWeightType::Max3D => {
                let n1 = self.get_3d_node(u)?;
                let n2 = self.get_3d_node(v)?;
                Ok(compute_3d_distance(edge_weight_type, n1, n2))
            }
            EdgeWeightType::Explicit => {
                let i = self.node_index(u)?;
                let j = self.node_index(v)?;
                explicit_weight_by_index(self.instance, u, v, i, j, self.hcp_edges.as_ref())
            }
            EdgeWeightType::Special | EdgeWeightType::XRay1 | EdgeWeightType::XRay2 => {
                // If explicit weights exist, fall back to them
                if self.instance.data.edge_weight_section.is_some() {
                    let i = self.node_index(u)?;
                    let j = self.node_index(v)?;
                    explicit_weight_by_index(self.instance, u, v, i, j, self.hcp_edges.as_ref())
                } else {
                    Err(TSPLibError::UnsupportedWeightCalculation(format!(
                        "{:?}",
                        edge_weight_type
                    )))
                }
            }
        }
    }
}

fn explicit_weight_by_index(
    instance: &Instance,
    u: NodeIdentifier,
    v: NodeIdentifier,
    i: usize,
    j: usize,
    hcp_edges: Option<&HashSet<(NodeIdentifier, NodeIdentifier)>>,
) -> Result<EdgeWeight, TSPLibError> {
    let weights = match instance.data.edge_weight_section.as_ref() {
        Some(w) => w,
        None => {
            if instance.specification.dtype == Type::HCP {
                return hcp_weight(instance, u, v, hcp_edges);
            }
            return Err(TSPLibError::WeightsNotAvailable(
                "missing EDGE_WEIGHT_SECTION".into(),
            ));
        }
    };

    let dim = instance.dimension()?;

    let format = instance
        .specification
        .edge_weight_format
        .unwrap_or(EdgeWeightFormat::FullMatrix);

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

fn hcp_weight(
    instance: &Instance,
    u: NodeIdentifier,
    v: NodeIdentifier,
    hcp_edges: Option<&HashSet<(NodeIdentifier, NodeIdentifier)>>,
) -> Result<EdgeWeight, TSPLibError> {
    if let Some(set) = hcp_edges {
        let pair = if u <= v { (u, v) } else { (v, u) };
        return if set.contains(&pair) {
            Ok(1)
        } else {
            Ok(HCP_NON_EDGE_WEIGHT)
        };
    }

    if let Some(ref edge_data) = instance.data.edge_data_section {
        match edge_data {
            EdgeDataSection::EdgeList(edges) => {
                let has_edge = edges
                    .iter()
                    .any(|&(a, b)| (a == u && b == v) || (a == v && b == u));
                if has_edge {
                    Ok(1)
                } else {
                    Ok(HCP_NON_EDGE_WEIGHT)
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
                    Ok(HCP_NON_EDGE_WEIGHT)
                }
            }
        }
    } else {
        Err(TSPLibError::WeightsNotAvailable(
            "no edge data found for HCP".into(),
        ))
    }
}
