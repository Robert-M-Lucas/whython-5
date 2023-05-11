use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::atomic::{AtomicBool, Ordering};
extern crate whython_4;

static CTRLC: AtomicBool = AtomicBool::new(false);

fn criterion_benchmark(c: &mut Criterion) {
    ctrlc::set_handler(|| {
        CTRLC.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let mut memory = match whython_4::processing::processor::MemoryManagers::load_from_compiled(
        "Compiled - 64.cwhy".to_string(),
    ) {
        Err(e) => {
            println!("Loading precompiled file failed - {}", e);
            return;
        }
        Ok(value) => value,
    };
    c.bench_function("atomic", |b| {
        b.iter(|| whython_4::execution::execute(&mut memory, &CTRLC))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
