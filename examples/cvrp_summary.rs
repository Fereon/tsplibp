//! Example: Capacitated Vehicle Routing Problem (CVRP) Inspection
//!
//! Demonstrates how to access vehicle routing specific information:
//! truck capacity, depot location(s), and customer demands.
//!
//! Run with:
//! ```bash
//! cargo run --example cvrp_summary instances/cvrp/eil22.vrp
//! ```

use std::env;
use tsplibp::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("instances/cvrp/eil22.vrp");

    println!("Loading CVRP instance: {}", file_path);
    let instance = Instance::from_file(file_path)?;

    println!("Problem Name:     {}", instance.specification.name);
    println!("Total Customers:  {}", instance.dimension()?);
    println!(
        "Truck Capacity:   {}",
        instance
            .specification
            .capacity
            .map(|c| c.to_string())
            .unwrap_or_else(|| "Not specified".into())
    );

    if let Some(ref depots) = instance.data.depot_section {
        println!("Depot nodes:      {:?}", depots);
    }

    if let Some(ref demands) = instance.data.demand_section {
        let total_demand: i32 = demands.iter().map(|&(_, d)| d).sum();
        println!("Total Demand:     {}", total_demand);
        println!("Sample Demands (first 5):");
        for &(node, demand) in demands.iter().take(5) {
            println!("  Node {}: demand = {}", node, demand);
        }
    }

    Ok(())
}
