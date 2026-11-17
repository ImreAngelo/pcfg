use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
#[cfg(feature = "gzip")]
use criterion::{BenchmarkGroup, measurement::WallTime};
use std::{fs, io, time::Duration};

use pcfg::Grammar;

// train on gzipped file
#[cfg(feature = "gzip")]
fn train_gzip(group: &mut BenchmarkGroup<'_, WallTime>, path: &str, id: &str) {
    group.bench_function(id, |b| {
        b.iter_batched(
            || {
                let file = fs::File::open(path).unwrap();
                io::BufReader::new(flate2::read::GzDecoder::new(file))
            },
            |reader| pcfg::train(std::hint::black_box(reader)).unwrap(),
            BatchSize::PerIteration,
        );
    });
}

// guess
fn guess(group: &mut BenchmarkGroup<'_, WallTime>, path: &str, id: &str) {
    let grammar = Grammar::load(std::path::Path::new(path)).unwrap();

    group.bench_function(id, |b| {
        b.iter_batched(
            || grammar.clone().into_guesses(),
            |g| std::hint::black_box(g.take(1_000_000).count()),
            BatchSize::SmallInput,
        );
    });
}

fn benchmark(c: &mut Criterion) {
    {
        // Benchmark training scenarios
        let mut t = c.benchmark_group("training");

        t.measurement_time(Duration::from_secs(60));
        t.sample_size(10);

        #[cfg(feature = "gzip")]
        train_gzip(&mut t, "data/rockyou.txt.gz", "rockyou.txt.gz");

        t.finish();
    }
    {
        // Benchmark guessing scenarios
        let mut g = c.benchmark_group("guessing");

        g.measurement_time(Duration::from_secs(6));
        g.throughput(Throughput::Elements(1_000_000));

        guess(&mut g, "data/rockyou-100k", "rockyou-100k");

        g.finish();
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
