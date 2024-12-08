#![allow(dead_code)]


use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tree_collections::prelude::*;

fn insertion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RBTree Baseline");
    group.measurement_time(std::time::Duration::new(7, 0));
    let tree_sizes = [10000, 40000, 70000, 100000, 130000];

    for &size in &tree_sizes {
        group.bench_with_input(BenchmarkId::new("Insert", size), &size, |b, &size| {
            b.iter(|| {
                let mut tree = RBTree::new();
                for i in 0..size {
                    tree.insert(black_box(i)); // insert elements in increasing order
                }
            });
        });
    }

    group.finish();
}

fn search_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RBTree Baseline");
    group.measurement_time(std::time::Duration::new(7, 0));
    let tree_sizes = [10000, 40000, 70000, 100000, 130000];

    for &size in &tree_sizes {
        // Initialize your AVL tree here and fill it with `size` elements
        let mut tree = RBTree::new();
        for i in 0..size {
            tree.insert(i); // Assuming a worst-case insert scenario
        }

        group.bench_with_input(BenchmarkId::new("Search lowest", size/10), &size, |b, &size| {
            b.iter(|| {
                for i in 0..(size / 10) {
                    black_box(tree.contains(i)); // Search for the lowest `size / 10` elements
                }
            });
        });
    }

    group.finish();

}

criterion_group!(benches, insertion_benchmark, search_benchmark);
criterion_main!(benches);
