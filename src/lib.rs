//! # tsplibp
//!
//! `tsplibp` is a fast, ergonomic parser and distance calculator for the
//! [TSPLIB95](https://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) benchmark library
//! written in Rust.
//!
//! It supports standard problem classes:
//! - **TSP** (Symmetric Traveling Salesperson Problem)
//! - **ATSP** (Asymmetric Traveling Salesperson Problem)
//! - **SOP** (Sequential Ordering Problem)
//! - **HCP** (Hamiltonian Cycle Problem)
//! - **CVRP** (Capacitated Vehicle Routing Problem)
//! - **TOUR** (Pre-computed / optimal solution tours)
//!
//! ## Key Features
//!
//! - **Parse from file, reader or string:** Load instances directly with [`Instance::from_file`],
//!   [`Instance::from_reader`], or standard [`std::str::FromStr`].
//! - **Automatic Distance & Weight Computation:** Calculates edge weights according to official
//!   TSPLIB95 specification formulas:
//!   - `EUC_2D`, `EUC_3D` (Euclidean distance with TSPLIB rounding `(int)(d + 0.5)`)
//!   - `MAN_2D`, `MAN_3D` (Manhattan distance)
//!   - `MAX_2D`, `MAX_3D` (Maximum coordinate distance)
//!   - `CEIL_2D` (Ceiling Euclidean distance)
//!   - `GEO` (Geographic distance from latitude/longitude)
//!   - `ATT` (Pseudo-Euclidean distance)
//!   - Explicit matrices in all standard formats (`FULL_MATRIX`, `UPPER_ROW`, `LOWER_DIAG_ROW`, etc.)
//! - **Tour Validation & Length Evaluation:** Verify candidate tours and compute their total round-trip distance.
//!
//! ## Quickstart Example
//!
//! ```rust
//! use tsplibp::Instance;
//!
//! let content = "NAME: mini3
//! TYPE: TSP
//! DIMENSION: 3
//! EDGE_WEIGHT_TYPE: EUC_2D
//! NODE_COORD_SECTION
//! 1 0.0 0.0
//! 2 30.0 40.0
//! 3 0.0 40.0
//! EOF";
//!
//! let instance: Instance = content.parse().expect("Failed to parse instance");
//! assert_eq!(instance.specification.name, "mini3");
//! assert_eq!(instance.dimension().unwrap(), 3);
//!
//! // Query distance between node 1 and node 2 (3-4-5 triangle: sqrt(30^2 + 40^2) = 50)
//! let dist_1_2 = instance.weight(1, 2).unwrap();
//! assert_eq!(dist_1_2, 50);
//!
//! // Calculate total round-trip length of a tour 1 -> 2 -> 3 -> 1
//! let tour = vec![1, 2, 3];
//! assert!(instance.is_valid_tour(&tour).unwrap());
//! let total = instance.tour_length(&tour).unwrap();
//! assert_eq!(total, 50 + 30 + 40); // 120
//! ```

pub mod datatypes;
pub mod error;
mod parser;
pub mod weights;

pub use datatypes::{
    DisplayDataType, Edge, EdgeDataFormat, EdgeDataSection, EdgeWeight, EdgeWeightFormat,
    EdgeWeightType, Instance, NodeCoordSection, NodeCoordType, NodeIdentifier, Specification, Type,
};
pub use error::TSPLibError;
pub use weights::WeightCalculator;
