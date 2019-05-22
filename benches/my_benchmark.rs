#[macro_use]
extern crate criterion;

use criterion::Criterion;

use gitchain::miner::Miner;
use gitchain::writer;
use gitchain::*;

// Benchmarking with six zeroes is extremely slow because criterion will run 5050 iterations which
// at an average of 2 seconds per solve results in a very long benchmark time.  That's why the
// seven zeroes test is not included in the benchmarks.  If you would like to test that, just add the
// function name to the bench group at the bottom of this file.
fn benchmark_mining_six_zeroes(c: &mut Criterion) {
    let tree = "TreeTest".to_string();
    let parent = None;
    let author = "AuthorTest <test@test.com>".to_string();
    let message = "MessageTest".to_string();
    let commit_time = time::strptime("2016-02-05 16:52:22", "%Y-%m-%d %H:%M:%S").unwrap();
    let blob = writer::generate_blob(tree, parent, author, message, commit_time).unwrap();

    let mut miner = Miner::new("000000".to_string(), blob);

    c.bench_function("proof of work solving with 6 zeros prefix.", move |b| {
        b.iter(|| miner.solve().unwrap())
    });
}

fn benchmark_mining_seven_zeroes(c: &mut Criterion) {
    let tree = "TreeTest".to_string();
    let parent = None;
    let author = "AuthorTest <test@test.com>".to_string();
    let message = "MessageTest".to_string();
    let commit_time = time::strptime("2016-02-05 16:52:22", "%Y-%m-%d %H:%M:%S").unwrap();
    let blob = writer::generate_blob(tree, parent, author, message, commit_time).unwrap();

    let mut miner = Miner::new("0000000".to_string(), blob);

    c.bench_function("proof of work solving with 7 zeros prefix.", move |b| {
        b.iter(|| miner.solve().unwrap())
    });
}

criterion_group!(benches, benchmark_mining_six_zeroes,);

criterion_main!(benches);
