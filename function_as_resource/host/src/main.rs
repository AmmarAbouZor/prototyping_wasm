use std::{
    path::PathBuf,
    time::{self, Duration},
};

use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};

const WASM_FILE_PATH: &str = "../client/target/wasm32-unknown-unknown/release/client.wasm";

struct MyStatus;

bindgen!();

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
    let file_path = PathBuf::from(WASM_FILE_PATH);

    assert!(file_path.exists(), "WASM File doesn't exist");

    let engine = Engine::new(Config::new().wasm_component_model(true)).unwrap();

    let component = Component::from_file(&engine, file_path).unwrap();

    let linker = Linker::new(&engine);

    let mut store = Store::new(&engine, MyStatus);

    let (count, _instance) = Count::instantiate(&mut store, &component, &linker).unwrap();

    let counter_interface = count.interface0.counter();

    let counter = counter_interface.call_constructor(&mut store).unwrap();

    let mut total_count = 0;

    let now = time::Instant::now();
    for _ in 0..cycle {
        let answer = counter_interface
            .call_count(&mut store, counter, &bytes, target)
            .unwrap();
        total_count += answer;
    }

    // This is required to be called for all instances of `ResourceAny`
    // to ensure that state associated with this resource is properly cleaned up.
    counter.resource_drop(&mut store).unwrap();

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
