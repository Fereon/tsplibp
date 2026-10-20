use std::{fs, path::Path};

use tsplibp::{self, Instance, TSPLibError, weights::HCP_NON_EDGE_WEIGHT};

fn parse_files_to_instances(path: &str) {
    let path = Path::new(path);
    assert!(
        path.is_dir(),
        "path '{}' is not a directory!",
        path.display()
    );

    let file_paths = fs::read_dir(path)
        .unwrap()
        .map(Result::unwrap)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| !path.display().to_string().contains("tspleap.c"));

    for file_path in file_paths {
        let content = fs::read_to_string(&file_path)
            .map_err(|err| {
                format!(
                    "could not read from file '{}': {}",
                    file_path.display(),
                    err
                )
            })
            .unwrap();

        let _: Instance = content
            .parse()
            .map_err(|err| {
                format!(
                    "could not parse content of file '{}': {}",
                    file_path.display(),
                    err
                )
            })
            .unwrap();
    }
}

#[test]
fn test_tsp_instances_parsing() {
    parse_files_to_instances("instances/tsp");
}

#[test]
fn test_hcp_instances_parsing() {
    parse_files_to_instances("instances/hcp");
}

#[test]
fn test_atsp_instances_parsing() {
    parse_files_to_instances("instances/atsp");
}

#[test]
fn test_sop_instances_parsing() {
    parse_files_to_instances("instances/sop");
}

#[test]
fn test_cvrp_instances_parsing() {
    parse_files_to_instances("instances/cvrp");
}

#[test]
fn test_optimal_tour_berlin52() {
    let instance = Instance::from_file("instances/tsp/berlin52.tsp").unwrap();
    let opt_tour_inst = Instance::from_file("instances/tsp/berlin52.opt.tour").unwrap();
    let tour = opt_tour_inst.tour().unwrap();

    assert!(instance.is_valid_tour(tour).unwrap());
    let length = instance.tour_length(tour).unwrap();
    // Known TSPLIB optimal length for berlin52 is 7542
    assert_eq!(length, 7542);
}

#[test]
fn test_optimal_tour_eil51() {
    let instance = Instance::from_file("instances/tsp/eil51.tsp").unwrap();
    let opt_tour_inst = Instance::from_file("instances/tsp/eil51.opt.tour").unwrap();
    let tour = opt_tour_inst.tour().unwrap();

    assert!(instance.is_valid_tour(tour).unwrap());
    let length = instance.tour_length(tour).unwrap();
    // Known TSPLIB optimal length for eil51 is 426
    assert_eq!(length, 426);
}

#[test]
fn test_optimal_tour_att48() {
    let instance = Instance::from_file("instances/tsp/att48.tsp").unwrap();
    let opt_tour_inst = Instance::from_file("instances/tsp/att48.opt.tour").unwrap();
    let tour = opt_tour_inst.tour().unwrap();

    assert!(instance.is_valid_tour(tour).unwrap());
    let length = instance.tour_length(tour).unwrap();
    // Known TSPLIB optimal length for att48 is 10628
    assert_eq!(length, 10628);
}

#[test]
fn test_optimal_tour_ulysses16() {
    let instance = Instance::from_file("instances/tsp/ulysses16.tsp").unwrap();
    let opt_tour_inst = Instance::from_file("instances/tsp/ulysses16.opt.tour").unwrap();
    let tour = opt_tour_inst.tour().unwrap();

    assert!(instance.is_valid_tour(tour).unwrap());
    let length = instance.tour_length(tour).unwrap();
    // Known TSPLIB optimal length for ulysses16 is 6859
    assert_eq!(length, 6859);
}

#[test]
fn test_optimal_tour_ulysses22() {
    let instance = Instance::from_file("instances/tsp/ulysses22.tsp").unwrap();
    let opt_tour_inst = Instance::from_file("instances/tsp/ulysses22.opt.tour").unwrap();
    let tour = opt_tour_inst.tour().unwrap();

    assert!(instance.is_valid_tour(tour).unwrap());
    let length = instance.tour_length(tour).unwrap();
    // Known TSPLIB optimal length for ulysses22 is 7013
    assert_eq!(length, 7013);
}

#[test]
fn test_distance_matrix_and_reader() {
    let file = std::fs::File::open("instances/tsp/eil51.tsp").unwrap();
    let instance = Instance::from_reader(file).unwrap();

    assert_eq!(instance.dimension().unwrap(), 51);
    let matrix = instance.distance_matrix().unwrap();
    assert_eq!(matrix.len(), 51);
    assert_eq!(matrix[0].len(), 51);
    assert_eq!(matrix[0][0], 0);

    // Distance must be symmetric for EUC_2D
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            assert_eq!(val, matrix[j][i]);
        }
    }
}

#[test]
fn test_explicit_weights_atsp_br17() {
    let instance = Instance::from_file("instances/atsp/br17.atsp").unwrap();
    assert_eq!(instance.dimension().unwrap(), 17);

    // Self-distance is 0
    assert_eq!(instance.weight(1, 1).unwrap(), 0);

    // Explicit entries from matrix
    assert_eq!(instance.weight(1, 2).unwrap(), 3);
    assert_eq!(instance.weight(1, 3).unwrap(), 5);
    assert_eq!(instance.weight(2, 1).unwrap(), 3);
    assert_eq!(instance.weight(3, 1).unwrap(), 5);
    assert_eq!(instance.weight(1, 12).unwrap(), 0);
}

#[test]
fn test_explicit_weights_gr17_lower_diag_row() {
    let instance = Instance::from_file("instances/tsp/gr17.tsp").unwrap();
    assert_eq!(instance.dimension().unwrap(), 17);

    assert_eq!(instance.weight(1, 1).unwrap(), 0);
    assert_eq!(instance.weight(2, 1).unwrap(), 633);
    assert_eq!(instance.weight(1, 2).unwrap(), 633);
}

#[test]
fn test_invalid_and_negative_cases() {
    // Non-existent file
    let err = Instance::from_file("non_existent_file.tsp");
    assert!(matches!(err, Err(TSPLibError::IoError(_))));

    // Invalid tour checks
    let instance = Instance::from_file("instances/tsp/berlin52.tsp").unwrap();

    // Wrong length
    let short_tour = vec![1, 2, 3];
    assert!(!instance.is_valid_tour(&short_tour).unwrap());

    // Duplicate node
    let mut dup_tour = (1..=52).collect::<Vec<_>>();
    dup_tour[1] = 1;
    assert!(!instance.is_valid_tour(&dup_tour).unwrap());

    // Out of bound node ID
    let mut out_of_bound_tour = (1..=52).collect::<Vec<_>>();
    out_of_bound_tour[0] = 999;
    assert!(!instance.is_valid_tour(&out_of_bound_tour).unwrap());

    // Tour with less than 2 nodes for tour_length
    assert!(matches!(
        instance.tour_length(&[1]),
        Err(TSPLibError::InvalidTour(_))
    ));
}

#[test]
fn test_display_trait() {
    let instance = Instance::from_file("instances/tsp/berlin52.tsp").unwrap();
    let spec_display = format!("{}", instance.specification);
    assert!(spec_display.contains("berlin52"));
    assert!(spec_display.contains("52"));

    let inst_display = format!("{}", instance);
    assert!(inst_display.contains("TSPLIB Instance"));
}

#[test]
fn test_hcp_weights() {
    let instance = Instance::from_file("instances/hcp/alb1000.hcp").unwrap();
    // Connected nodes have weight 1
    let val = instance.weight(1, 2).unwrap();
    assert!(val == 1 || val == HCP_NON_EDGE_WEIGHT);

    // Check optimal HCP tour has length equal to dimension (1000 edges of weight 1)
    let opt_tour_inst = Instance::from_file("instances/hcp/alb1000.opt.tour").unwrap();
    let tour = opt_tour_inst.tour().unwrap();
    assert!(instance.is_valid_tour(tour).unwrap());
    assert_eq!(instance.tour_length(tour).unwrap(), 1000);
}

#[test]
fn test_f64_precision_and_geo_clamp() {
    // High-precision coordinates from usa13509 (9 significant digits)
    let instance = Instance::from_file("instances/tsp/usa13509.tsp").unwrap();
    if let Some(tsplibp::NodeCoordSection::TwodCoords(ref nodes)) = instance.data.node_coord_section
    {
        assert!((nodes[0].x_coordinate - 245552.778).abs() < 1e-9);
        assert!((nodes[0].y_coordinate - 817827.778).abs() < 1e-9);
    } else {
        panic!("expected 2D coords");
    }

    // Distinct nodes with identical GEO coordinates must not produce NaN via acos()
    let geo_dup = "NAME: geo_dup\nTYPE: TSP\nDIMENSION: 2\nEDGE_WEIGHT_TYPE: GEO\nNODE_COORD_SECTION\n1 16.47 96.10\n2 16.47 96.10\nEOF";
    let inst: Instance = geo_dup.parse().unwrap();
    assert_eq!(inst.weight(1, 2).unwrap(), 1);
}
