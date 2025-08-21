use std::{fs, hint::black_box};

fn main() {
    let wasm_bytes = fs::read("user.wasm").expect("Failed to read user.wasm");

    let instant = std::time::Instant::now();
    for _ in 0..10 {
        let module =
            wirm::Module::parse(black_box(&wasm_bytes), false).expect("Failed to parse module");
        let _ = black_box(module);
    }
    println!("wirm parse: {:?}", instant.elapsed());
}
