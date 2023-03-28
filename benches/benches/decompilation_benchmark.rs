use std::time::Duration;

use benches::{Invoke, RadixEngineToolkit};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use native_transaction::{
    manifest::{compile, decompile},
    model::NotarizedTransaction,
};
use radix_engine_toolkit::model::transaction::InstructionKind;
use radix_engine_toolkit::request::*;
use scrypto::{network::NetworkDefinition, prelude::manifest_decode};

fn decompile_intent_natively_benchmarks(c: &mut Criterion) {
    let compiled_transaction = hex::decode(include_str!("./transaction.hex")).unwrap();

    let mut group = c.benchmark_group("Decompile Intent Natively");
    group.sample_size(10);

    group.bench_function("SBOR Decode to NotarizedTransaction", |b| {
        b.iter(|| {
            black_box(manifest_decode::<NotarizedTransaction>(&compiled_transaction).unwrap())
        })
    });
    group.bench_function("SBOR Decode to NotarizedTransaction and Decompile", |b| {
        b.iter(|| {
            black_box({
                let transaction =
                    manifest_decode::<NotarizedTransaction>(&compiled_transaction).unwrap();
                decompile(
                    &transaction.signed_intent.intent.manifest.instructions,
                    &NetworkDefinition::simulator(),
                )
                .unwrap()
            })
        })
    });
    group.bench_function(
        "SBOR Decode to NotarizedTransaction, Decompile, then Recompile",
        |b| {
            b.iter(|| {
                black_box({
                    let transaction =
                        manifest_decode::<NotarizedTransaction>(&compiled_transaction).unwrap();
                    let manifest = decompile(
                        &transaction.signed_intent.intent.manifest.instructions,
                        &NetworkDefinition::simulator(),
                    )
                    .unwrap();
                    compile(&manifest, &NetworkDefinition::simulator(), vec![])
                })
            })
        },
    );

    group.finish();
}

fn decompile_intent_with_core_toolkit_benchmarks(c: &mut Criterion) {
    let compiled_transaction = hex::decode(include_str!("./transaction.hex")).unwrap();

    let mut group = c.benchmark_group("Decompile Intent with Toolkit Core");
    group.sample_size(10);

    group.bench_function("Decompile Unknown Intent to String", |b| {
        b.iter(|| {
            black_box({
                let request = DecompileUnknownTransactionIntentRequest {
                    compiled_unknown_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::String,
                };
                let response = DecompileUnknownTransactionIntentHandler::fulfill(request);
                response.unwrap()
            })
        })
    });
    group.bench_function("Decompile Unknown Intent to Parsed", |b| {
        b.iter(|| {
            black_box({
                let request = DecompileUnknownTransactionIntentRequest {
                    compiled_unknown_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::Parsed,
                };
                let response = DecompileUnknownTransactionIntentHandler::fulfill(request);
                response.unwrap()
            })
        })
    });
    group.bench_function("Decompile Notarized Intent to String", |b| {
        b.iter(|| {
            black_box({
                let request = DecompileNotarizedTransactionRequest {
                    compiled_notarized_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::String,
                };
                let response = DecompileNotarizedTransactionHandler::fulfill(request);
                response.unwrap()
            })
        })
    });
    group.bench_function("Decompile Notarized Intent to Parsed", |b| {
        b.iter(|| {
            black_box({
                let request = DecompileNotarizedTransactionRequest {
                    compiled_notarized_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::Parsed,
                };
                let response = DecompileNotarizedTransactionHandler::fulfill(request);
                response.unwrap()
            })
        })
    });

    group.finish();
}

fn decompile_intent_with_toolkit_wrapper_benchmarks(c: &mut Criterion) {
    let compiled_transaction = hex::decode(include_str!("./transaction.hex")).unwrap();

    let mut group = c.benchmark_group("Decompile Intent with Toolkit Wrapper");
    group.sample_size(10);

    group.bench_function("Decompile Unknown Intent to String", |b| {
        b.iter(|| {
            black_box(
                RadixEngineToolkit::invoke(DecompileUnknownTransactionIntentRequest {
                    compiled_unknown_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::String,
                })
                .unwrap(),
            )
        })
    });
    group.bench_function("Decompile Unknown Intent to Parsed", |b| {
        b.iter(|| {
            black_box(
                RadixEngineToolkit::invoke(DecompileUnknownTransactionIntentRequest {
                    compiled_unknown_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::Parsed,
                })
                .unwrap(),
            )
        })
    });
    group.bench_function("Decompile Notarized Intent to String", |b| {
        b.iter(|| {
            black_box(
                RadixEngineToolkit::invoke(DecompileNotarizedTransactionRequest {
                    compiled_notarized_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::String,
                })
                .unwrap(),
            )
        })
    });
    group.bench_function("Decompile Notarized Intent to Parsed", |b| {
        b.iter(|| {
            black_box(
                RadixEngineToolkit::invoke(DecompileNotarizedTransactionRequest {
                    compiled_notarized_intent: compiled_transaction.clone(),
                    instructions_output_kind: InstructionKind::Parsed,
                })
                .unwrap(),
            )
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    // config = Criterion::default();
    targets = decompile_intent_natively_benchmarks, decompile_intent_with_toolkit_wrapper_benchmarks, decompile_intent_with_core_toolkit_benchmarks
);
criterion_main!(benches);
