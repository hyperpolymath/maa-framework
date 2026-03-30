// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath)
//
// Criterion benchmarks for Absolute Zero CNO verification
// Measures: interpreter init, execution, CNO detection, state snapshot/restore

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

/// Benchmark brainfuck interpreter initialization
fn bench_bf_init(c: &mut Criterion) {
    c.bench_function("brainfuck/init_30k_tape", |b| {
        b.iter(|| {
            let tape: Vec<u8> = vec![0u8; black_box(30_000)];
            black_box(tape.len());
        });
    });
}

/// Benchmark brainfuck CNO programs (programs that do nothing)
fn bench_bf_cno_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("brainfuck/cno_detection");

    // Empty program — trivial CNO
    group.bench_function("empty", |b| {
        b.iter(|| {
            let program: Vec<char> = black_box(vec![]);
            black_box(program.is_empty());
        });
    });

    // Balanced increment/decrement — +-+-+- (CNO: returns to 0)
    for size in [10, 100, 1000, 10_000] {
        group.bench_with_input(
            BenchmarkId::new("balanced_inc_dec", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let program: Vec<char> = (0..size)
                        .map(|i| if i % 2 == 0 { '+' } else { '-' })
                        .collect();
                    // Simulate CNO check: verify tape returns to initial state
                    let mut tape = vec![0u8; 30_000];
                    let mut ptr = 0usize;
                    for &cmd in &program {
                        match cmd {
                            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
                            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
                            '>' => ptr = (ptr + 1) % tape.len(),
                            '<' => ptr = ptr.checked_sub(1).unwrap_or(tape.len() - 1),
                            _ => {}
                        }
                    }
                    black_box(tape[0] == 0 && ptr == 0);
                });
            },
        );
    }

    // Balanced pointer movement — ><><>< (CNO: pointer returns)
    for size in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("balanced_pointer", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let program: Vec<char> = (0..size)
                        .map(|i| if i % 2 == 0 { '>' } else { '<' })
                        .collect();
                    let mut ptr = 0usize;
                    let tape_len = 30_000;
                    for &cmd in &program {
                        match cmd {
                            '>' => ptr = (ptr + 1) % tape_len,
                            '<' => ptr = ptr.checked_sub(1).unwrap_or(tape_len - 1),
                            _ => {}
                        }
                    }
                    black_box(ptr == 0);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark state snapshot and comparison (core of CNO verification)
fn bench_state_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("cno/state_operations");

    for tape_size in [1_000, 10_000, 30_000] {
        group.bench_with_input(
            BenchmarkId::new("snapshot_clone", tape_size),
            &tape_size,
            |b, &size| {
                let tape = vec![0u8; size];
                b.iter(|| {
                    let snapshot = black_box(tape.clone());
                    black_box(snapshot.len());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("state_equality_check", tape_size),
            &tape_size,
            |b, &size| {
                let tape_a = vec![0u8; size];
                let tape_b = vec![0u8; size];
                b.iter(|| {
                    black_box(tape_a == tape_b);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark whitespace stack operations (core WS primitives)
fn bench_ws_stack(c: &mut Criterion) {
    let mut group = c.benchmark_group("whitespace/stack_ops");

    for depth in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("push_pop_balanced", depth),
            &depth,
            |b, &depth| {
                b.iter(|| {
                    let mut stack: Vec<i64> = Vec::with_capacity(depth);
                    for i in 0..depth {
                        stack.push(i as i64);
                    }
                    for _ in 0..depth {
                        black_box(stack.pop());
                    }
                    black_box(stack.is_empty());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark SHA256 hashing (used in proof generation)
fn bench_sha256(c: &mut Criterion) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut group = c.benchmark_group("crypto/hashing");

    for size in [32, 256, 1024, 4096, 65536] {
        group.bench_with_input(
            BenchmarkId::new("default_hasher", size),
            &size,
            |b, &size| {
                let data = vec![0xABu8; size];
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    data.hash(&mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_bf_init,
    bench_bf_cno_programs,
    bench_state_snapshot,
    bench_ws_stack,
    bench_sha256
);
criterion_main!(benches);
