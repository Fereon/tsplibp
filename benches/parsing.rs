use criterion::{Criterion, criterion_group, criterion_main};
use std::{fs, path::Path, str::FromStr};
use tsplibp::Instance;

fn load_file_contents(path: &str) -> Vec<String> {
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

    file_paths
        .map(|file_path| {
            fs::read_to_string(&file_path)
                .map_err(|err| {
                    format!(
                        "could not read from file '{}': {}",
                        file_path.display(),
                        err
                    )
                })
                .unwrap()
        })
        .collect()
}

fn benchmark_tsp(c: &mut Criterion) {
    let contents = load_file_contents("instances/tsp");

    c.bench_function("tsp", |b| {
        b.iter(|| {
            for content in &contents {
                let instance = Instance::from_str(content).unwrap();
                drop(instance)
            }
        })
    });
}

fn benchmark_hcp(c: &mut Criterion) {
    let contents = load_file_contents("instances/hcp");

    c.bench_function("hcp", |b| {
        b.iter(|| {
            for content in &contents {
                let instance = Instance::from_str(content).unwrap();
                drop(instance)
            }
        })
    });
}

fn benchmark_atsp(c: &mut Criterion) {
    let contents = load_file_contents("instances/atsp");

    c.bench_function("atsp", |b| {
        b.iter(|| {
            for content in &contents {
                let instance = Instance::from_str(content).unwrap();
                drop(instance)
            }
        })
    });
}

fn benchmark_sop(c: &mut Criterion) {
    let contents = load_file_contents("instances/sop");

    c.bench_function("sop", |b| {
        b.iter(|| {
            for content in &contents {
                let instance = Instance::from_str(content).unwrap();
                drop(instance)
            }
        })
    });
}

fn benchmark_cvrp(c: &mut Criterion) {
    let contents = load_file_contents("instances/cvrp");

    c.bench_function("cvrp", |b| {
        b.iter(|| {
            for content in &contents {
                let instance = Instance::from_str(content).unwrap();
                drop(instance)
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_tsp,
    benchmark_hcp,
    benchmark_atsp,
    benchmark_sop,
    benchmark_cvrp
);
criterion_main!(benches);
