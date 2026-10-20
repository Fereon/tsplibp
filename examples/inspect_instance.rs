//! Example: Inspecting a TSPLIB file
//!
//! Run with:
//! ```bash
//! cargo run --example inspect_instance instances/tsp/berlin52.tsp
//! ```

use std::env;
use tsplibp::{Instance, NodeCoordSection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("instances/tsp/berlin52.tsp");

    println!("Loading instance from: {}", file_path);
    let instance = Instance::from_file(file_path)?;

    println!("==========================================");
    println!("Name:             {}", instance.specification.name);
    println!("Type:             {:?}", instance.specification.dtype);
    if let Ok(dim) = instance.dimension() {
        println!("Dimension:        {}", dim);
    } else {
        println!("Dimension:        Not specified");
    }
    if let Some(cap) = instance.specification.capacity {
        println!("Capacity:         {}", cap);
    }
    if let Some(ew_type) = instance.specification.edge_weight_type {
        println!("Edge Weight Type: {:?}", ew_type);
    }
    if let Some(ew_format) = instance.specification.edge_weight_format {
        println!("Edge Weight Fmt:  {:?}", ew_format);
    }
    if !instance.specification.comment.is_empty() {
        println!(
            "Comments:         {}",
            instance.specification.comment.join(" | ")
        );
    }
    println!("==========================================");

    // Preview coordinates if present
    if let Some(ref coords) = instance.data.node_coord_section {
        match coords {
            NodeCoordSection::TwodCoords(nodes) => {
                println!("First 3 nodes (2D):");
                for node in nodes.iter().take(3) {
                    println!(
                        "  Node {}: ({}, {})",
                        node.identifier, node.x_coordinate, node.y_coordinate
                    );
                }
            }
            NodeCoordSection::ThreedCoords(nodes) => {
                println!("First 3 nodes (3D):");
                for node in nodes.iter().take(3) {
                    println!(
                        "  Node {}: ({}, {}, {})",
                        node.identifier, node.x_coordinate, node.y_coordinate, node.z_coordinate
                    );
                }
            }
        }
    }

    // Preview distance between first two nodes if available
    if let Ok(nodes) = instance.node_identifiers()
        && nodes.len() >= 2
    {
        let u = nodes[0];
        let v = nodes[1];
        if let Ok(weight) = instance.weight(u, v) {
            println!("Distance between Node {} and Node {}: {}", u, v, weight);
        }
    }

    Ok(())
}
