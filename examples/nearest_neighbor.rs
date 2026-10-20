//! Example: Nearest Neighbor TSP Heuristic
//!
//! Demonstrates how to solve a TSP instance with the simple Nearest Neighbor greedy heuristic
//! and compare the result with the optimal tour if available.
//!
//! Run with:
//! ```bash
//! cargo run --example nearest_neighbor instances/tsp/berlin52.tsp
//! ```

use std::{collections::HashSet, env, path::Path};
use tsplibp::{EdgeWeight, Instance, NodeIdentifier, Type};

fn nearest_neighbor_tour(
    instance: &Instance,
) -> Result<Vec<NodeIdentifier>, Box<dyn std::error::Error>> {
    let nodes = instance.node_identifiers()?;
    let dim = instance.dimension()?;
    if dim < 2 {
        return Ok(nodes);
    }

    let calculator = instance.weight_calculator()?;
    let mut unvisited: HashSet<NodeIdentifier> = nodes.iter().copied().collect();
    let mut tour = Vec::with_capacity(dim);
    let is_sop = instance.specification.dtype == Type::SOP;

    // Start at the first node
    let mut current = nodes[0];
    tour.push(current);
    unvisited.remove(&current);

    while !unvisited.is_empty() {
        let mut best_next = None;
        let mut best_dist = EdgeWeight::MAX;

        for &candidate in &unvisited {
            let dist = calculator.weight(current, candidate)?;
            // In SOP instances, -1 encodes a precedence constraint (forbidden transition)
            if is_sop && dist < 0 {
                continue;
            }
            if dist < best_dist {
                best_dist = dist;
                best_next = Some(candidate);
            }
        }

        let next = best_next
            .or_else(|| unvisited.iter().copied().next())
            .expect("unvisited set is non-empty");
        tour.push(next);
        unvisited.remove(&next);
        current = next;
    }

    Ok(tour)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("instances/tsp/berlin52.tsp");

    println!("Loading TSP instance: {}", file_path);
    let instance = Instance::from_file(file_path)?;

    println!("Running Nearest Neighbor heuristic...");
    let tour = nearest_neighbor_tour(&instance)?;

    assert!(instance.is_valid_tour(&tour)?);
    let heuristic_length = instance.tour_length(&tour)?;
    println!("Nearest Neighbor Tour Length: {}", heuristic_length);

    // Check if an optimal tour file exists alongside the instance
    let opt_path = Path::new(file_path).with_extension("opt.tour");
    if opt_path.exists() {
        let opt_instance = Instance::from_file(&opt_path)?;
        if let Some(opt_tour) = opt_instance.tour() {
            let opt_length = instance.tour_length(opt_tour)?;
            println!("Official Optimal Tour Length: {}", opt_length);
            let gap = ((heuristic_length - opt_length) as f64 / opt_length as f64) * 100.0;
            println!("Optimality Gap: {:.2}%", gap);
        }
    }

    Ok(())
}
