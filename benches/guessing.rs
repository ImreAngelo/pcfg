use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};

use pcfg::Grammar;

fn benchmark(c: &mut Criterion) {
    let grammar = Grammar::load(std::path::Path::new("tests/fixtures/model")).unwrap();

    let mut group = c.benchmark_group("guessing");
    group.throughput(Throughput::Elements(1_000_000));
    group.bench_function("rockyou-100k", |b| {
        b.iter_batched(
            || grammar.clone().into_guesses(),
            |g| std::hint::black_box(g.take(1_000_000).count()),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
