# tsplibp

[![Crates.io](https://img.shields.io/badge/crates.io-tsplibp-orange.svg)](https://crates.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-edition%202024-green.svg)](https://www.rust-lang.org)

**tsplibp** is a fast, ergonomic Rust parser and distance calculator for the standard [TSPLIB95](https://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) library of benchmark instances for the Traveling Salesperson Problem and related combinatorial optimization problems.

---

## ✨ Features

- **Full TSPLIB95 Problem Classes:**
  - `TSP`: Symmetric Traveling Salesperson Problem
  - `ATSP`: Asymmetric Traveling Salesperson Problem
  - `SOP`: Sequential Ordering Problem (TSP with precedence constraints)
  - `HCP`: Hamiltonian Cycle Problem
  - `CVRP`: Capacitated Vehicle Routing Problem
  - `TOUR`: Optimal and candidate solution tour files (`.opt.tour`)

- **Built-in Distance & Weight Computation:**
  - Avoids manual indexing and floating-point rounding bugs by implementing the exact TSPLIB95 distance formulas:
    - **`EUC_2D` & `EUC_3D`**: Euclidean metric with integer rounding `nint(d) = (int)(d + 0.5)`
    - **`GEO`**: Geographic distance from coordinates (latitude/longitude) via spherical trigonometry
    - **`ATT`**: Pseudo-Euclidean metric (used in ATT instances)
    - **`MAN_2D` & `MAN_3D`**: Manhattan / Grid distance
    - **`MAX_2D` & `MAX_3D`**: Maximum metric ($L_\infty$)
    - **`CEIL_2D`**: Ceiling Euclidean metric
    - **Explicit Matrices**: Automatically maps symmetric and asymmetric row/column formats (`FULL_MATRIX`, `UPPER_ROW`, `LOWER_ROW`, `UPPER_DIAG_ROW`, `LOWER_DIAG_ROW`, `UPPER_COL`, `LOWER_COL`, etc.)

- **Tour Evaluation & Validation:**
  - Compute total round-trip tour distance with `instance.tour_length(&tour)`.
  - Validate tours (check for complete Hamiltonian cycles without duplicates or missing nodes) with `instance.is_valid_tour(&tour)`.

- **Ergonomic Loading:**
  - Load directly from a file path: `Instance::from_file(...)`
  - Load from any reader: `Instance::from_reader(...)`
  - Parse from a string: `content.parse::<Instance>()`

---

## 📦 Installation

Add `tsplibp` to your `Cargo.toml`:

```toml
[dependencies]
tsplibp = "0.1.0"
```

Or from a local path / Git repository:

```toml
[dependencies]
tsplibp = { path = "../tsplibp" }
```

---

## 🚀 Quickstart

### 1. Load an Instance and Compute Distances

```rust
use tsplibp::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a TSPLIB file (e.g. 52 locations in Berlin)
    let instance = Instance::from_file("instances/tsp/berlin52.tsp")?;

    println!("Instance: {}", instance.specification.name);
    println!("Dimension: {}", instance.dimension()?);

    // Query distance between node 1 and node 2:
    let d_1_2 = instance.weight(1, 2)?;
    println!("Distance(1, 2) = {}", d_1_2);

    // Query all node IDs (typically 1..=N in TSPLIB):
    let nodes = instance.node_identifiers()?;
    println!("First node: {}, Last node: {}", nodes[0], nodes[nodes.len() - 1]);

    Ok(())
}
```

### 2. Verify a Tour and Calculate Its Total Length

```rust
use tsplibp::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = Instance::from_file("instances/tsp/berlin52.tsp")?;
    let opt_tour_inst = Instance::from_file("instances/tsp/berlin52.opt.tour")?;

    // Extract the official optimal tour:
    let opt_tour = opt_tour_inst.tour().expect("Expected tour section");

    // Validate that the tour visits every node exactly once:
    assert!(instance.is_valid_tour(opt_tour)?);

    // Calculate total round-trip length:
    let length = instance.tour_length(opt_tour)?;
    println!("Optimal tour length: {}", length); // Exactly 7542
    assert_eq!(length, 7542);

    Ok(())
}
```

---

## 💡 Guide for Students & Course Participants

### Node Indexing: 1-Based vs. 0-Based
In TSPLIB95 files, nodes are traditionally identified by **1-based integers** ($1, 2, \dots, N$).
* When calling `instance.weight(u, v)`, pass the actual **node identifiers** (`1` to `N`).
* If you extract a full 2D matrix using `instance.distance_matrix()?`, the resulting matrix is 0-indexed in Rust:
  ```rust
  let matrix = instance.distance_matrix()?;
  // matrix[i][j] corresponds to the i-th and j-th node in instance.node_identifiers()
  let dist = matrix[0][1]; // Distance between node 1 and node 2
  ```

### Working with Vehicle Routing (CVRP)
For Capacitated Vehicle Routing problems, `instance.specification.capacity`, `instance.data.depot_section`, and `instance.data.demand_section` provide vehicle and customer details:

```rust
let instance = Instance::from_file("instances/cvrp/eil22.vrp")?;
let capacity = instance.specification.capacity.unwrap();
let depots = instance.data.depot_section.as_ref().unwrap();
let demands = instance.data.demand_section.as_ref().unwrap();

println!("Truck Capacity: {}", capacity);
println!("Depot Node: {}", depots[0]);
for &(customer, demand) in demands {
    println!("Customer {}: demand {}", customer, demand);
}
```

---

## 📁 Runnable Examples

The repository includes ready-to-run examples in the `examples/` directory:

1. **Inspect an instance:**
   ```bash
   cargo run --example inspect_instance instances/tsp/berlin52.tsp
   ```

2. **Nearest-Neighbor TSP Heuristic:**
   ```bash
   cargo run --example nearest_neighbor instances/tsp/berlin52.tsp
   ```

3. **Export Full Distance Matrix:**
   ```bash
   cargo run --example distance_matrix instances/tsp/eil51.tsp
   ```

4. **CVRP Summary & Demand Inspection:**
   ```bash
   cargo run --example cvrp_summary instances/cvrp/eil22.vrp
   ```

---

## 🧪 Testing & Verification

Run the test suite:

```bash
# Run all unit tests, integration tests, and documentation tests
cargo test

# Run doc tests only
cargo test --doc

# Check code formatting and linter
cargo fmt --check
cargo clippy --all-targets
```

Run benchmarks:

```bash
cargo bench
```

---

## 📖 References

* **TSPLIB Documentation:** Gerhard Reinelt, *TSPLIB95*, Technical Report, Universität Heidelberg (1995).
* **Official Website:** [comopt.ifi.uni-heidelberg.de/software/TSPLIB95](https://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/)

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
