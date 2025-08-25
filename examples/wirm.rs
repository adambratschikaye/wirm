use std::{fs, hint::black_box};

fn main() {
    let wasm_bytes = fs::read("user.wasm").expect("Failed to read user.wasm");
    let mut total_parse = std::time::Duration::from_secs(0);
    let mut total_encode = std::time::Duration::from_secs(0);

    for _ in 0..10 {
        let instant = std::time::Instant::now();
        let mut module =
            wirm::Module::parse(black_box(&wasm_bytes), false).expect("Failed to parse module");
        total_parse += instant.elapsed();
        let instant = std::time::Instant::now();
        let encoded = module.encode();
        total_encode += instant.elapsed();
        let _ = black_box(encoded);
    }
    println!(
        "wirm parse time {:?}, encode time {:?}",
        total_parse, total_encode
    );
}
