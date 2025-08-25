use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::hint::black_box;

fn basic(c: &mut Criterion) {
    // Read the user.wasm file
    let wasm_bytes = fs::read("user.wasm").expect("Failed to read user.wasm");

    c.bench_function("roundtrip_wirm", |b| {
        b.iter(|| {
            let _instant = std::time::Instant::now();
            // Parse the module
            let mut module =
                wirm::Module::parse(black_box(&wasm_bytes), false).expect("Failed to parse module");

            // println!("wirm parse: {:?}", instant.elapsed());
            // let import_id = module
            //     .imports
            //     .find("ic0".to_string(), "time".to_string())
            //     .unwrap();
            // let mut func = FunctionBuilder::new(&[], &[DataType::I64]);
            // let instructions = vec![Instruction::new(Operator::I64Const { value: 1 })];
            // func.body = Body {
            //     locals: vec![],
            //     num_locals: 0,
            //     num_instructions: instructions.len(),
            //     instructions,
            //     name: None,
            // };
            // func.replace_import_in_module(&mut module, import_id);
            // Emit the module back to bytes
            let emitted_bytes = module.encode();

            // Return the emitted bytes to prevent optimization
            black_box(emitted_bytes)
        });
    });

    c.bench_function("roundtrip_transform", |b| {
        b.iter(|| {
            let _instant = std::time::Instant::now();
            // Parse the module
            let module = wirm::wasm_transform::Module::parse(black_box(&wasm_bytes), false)
                .expect("Failed to parse module");
            // println!("transform parse: {:?}", instant.elapsed());

            // let import_id = module
            //     .imports
            //     .find("ic0".to_string(), "time".to_string())
            //     .unwrap();
            // let mut func = FunctionBuilder::new(&[], &[DataType::I64]);
            // let instructions = vec![Instruction::new(Operator::I64Const { value: 1 })];
            // func.body = Body {
            //     locals: vec![],
            //     num_locals: 0,
            //     num_instructions: instructions.len(),
            //     instructions,
            //     name: None,
            // };
            // func.replace_import_in_module(&mut module, import_id);
            // Emit the module back to bytes
            let emitted_bytes = module.encode().unwrap();

            // Return the emitted bytes to prevent optimization
            black_box(emitted_bytes)
        });
    });

    c.bench_function("parse_wirm", |b| {
        b.iter_with_large_drop(|| {
            // Parse the module
            let module =
                wirm::Module::parse(black_box(&wasm_bytes), false).expect("Failed to parse module");
            // println!("transform parse: {:?}", instant.elapsed());
            black_box(module)
        });
    });

    c.bench_function("parse_transform", |b| {
        b.iter_with_large_drop(|| {
            // Parse the module
            let module = wirm::wasm_transform::Module::parse(black_box(&wasm_bytes), false)
                .expect("Failed to parse module");
            // println!("transform parse: {:?}", instant.elapsed());
            black_box(module)
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = basic,
}
criterion_main!(benches);
