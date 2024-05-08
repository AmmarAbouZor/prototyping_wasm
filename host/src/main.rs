use std::{
    path::PathBuf,
    time::{self, Duration},
    u64,
};

use wasmtime::{component::Linker, Config, Engine, Store};

fn main() {
    let mut bytes = Vec::new();
    for _ in 0..2000000 {
        bytes.extend([b'a', b'a', b'b', b'c']);
    }
    let target = b'a';

    println!("Start...");
    println!("Slice len: {}", bytes.len());

    let cycle = 100;

    let wasm = calc_wasm(&bytes, target, cycle).as_nanos();
    println!("Wasm: {}", wasm);

    let native = calc_native(&bytes, target, cycle).as_nanos();
    println!("Native: {}", native);

    let percent = (wasm as f64 / native as f64) * 100.0;

    println!("Percent: {percent}");
}

fn calc_wasm(bytes: &[u8], target: u8, cycle: usize) -> Duration {
    let wasm_path = PathBuf::from("./../client/target/wasm32-unknown-unknown/release/client.wasm");

    assert!(wasm_path.exists(), "wasm file doesn't exist");

    let mut config = Config::new();
    config.wasm_component_model(true);

    // config.wasm_reference_types(true);
    // config.allocation_strategy(wasmtime::InstanceAllocationStrategy::Pooling(
    //     wasmtime::PoolingAllocationConfig::default(),
    // ));

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, ());

    let component = wasmtime::component::Component::from_file(&engine, wasm_path).unwrap();

    let linker = Linker::new(&engine);

    let mut total_count = 0;

    let instance = linker.instantiate(&mut store, &component).unwrap();
    let count = instance
        .get_typed_func::<(&[u8], u8), (u64,)>(&mut store, "count")
        .unwrap();

    let now = time::Instant::now();
    for _ in 0..cycle {
        let res = count.call(&mut store, (bytes, target)).unwrap();
        count.post_return(&mut store).unwrap();
        total_count += res.0;
    }

    let elapsed = now.elapsed();

    println!("Wasm {total_count}");

    elapsed
}

fn calc_native(bytes: &[u8], target: u8, cycle: usize) -> Duration {
    let now = time::Instant::now();
    let mut total = 0;
    for _ in 0..cycle {
        let count = bytes.iter().filter(|&&b| b == target).count();
        total += count;
    }

    let elapsed = now.elapsed();

    println!("Native: {total}");

    elapsed
}
