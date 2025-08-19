use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::hint::black_box;
use wasmparser::Operator;
use wirm::{
    ir::{
        function::FunctionBuilder,
        types::{Body, Instruction},
    },
    DataType, Module,
};

fn module_roundtrip_benchmark(c: &mut Criterion) {
    // Read the user.wasm file
    let wasm_bytes = fs::read("user.wasm").expect("Failed to read user.wasm");

    c.bench_function("module_roundtrip", |b| {
        b.iter(|| {
            // Parse the module
            let mut module =
                Module::parse(black_box(&wasm_bytes), false).expect("Failed to parse module");

            let import_id = module
                .imports
                .find("ic0".to_string(), "time".to_string())
                .unwrap();
            let mut func = FunctionBuilder::new(&[], &[DataType::I64]);
            let instructions = vec![Instruction::new(Operator::I64Const { value: 1 })];
            func.body = Body {
                locals: vec![],
                num_locals: 0,
                num_instructions: instructions.len(),
                instructions,
                name: None,
            };
            func.replace_import_in_module(&mut module, import_id);
            // Emit the module back to bytes
            let emitted_bytes = module.encode();

            // Return the emitted bytes to prevent optimization
            black_box(emitted_bytes)
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = module_roundtrip_benchmark
}
criterion_main!(benches);
