//! Example: Distance Matrix Export / Extraction
//!
//! Demonstrates how to extract a 2D distance matrix from a TSPLIB instance,
//! which is often needed when interfacing with external solvers (MIP, ILP, dynamic programming).
//!
//! Run with:
//! ```bash
//! cargo run --example distance_matrix instances/tsp/eil51.tsp
//! ```

use std::env;
use tsplibp::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("instances/tsp/eil51.tsp");

    println!("Loading instance: {}", file_path);
    let instance = Instance::from_file(file_path)?;
    let dim = instance.dimension()?;

    println!("Computing full {}x{} distance matrix...", dim, dim);
    let matrix = instance.distance_matrix()?;

    println!("Matrix preview (first 5x5 submatrix):");
    let preview_dim = dim.min(5);
    print!("     ");
    for j in 0..preview_dim {
        print!("{:6}", format!("N{}", j + 1));
    }
    println!();

    for (i, row) in matrix.iter().enumerate().take(preview_dim) {
        print!("{:4}:", format!("N{}", i + 1));
        for val in row.iter().take(preview_dim) {
            print!("{:6}", val);
        }
        println!();
    }

    Ok(())
}
