use std::path::PathBuf;

use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

const RUST_WASM_FILE_PATH: &str = "../client/target/wasm32-wasi/debug/client.wasm";
const C_WASM_FILE_PATH: &str = "../c_client/my-component.wasm";

struct MyStatus {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl MyStatus {
    fn new(ctx: WasiCtx, table: ResourceTable) -> Self {
        Self { ctx, table }
    }
}

impl WasiView for MyStatus {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

bindgen!();

impl CountingImports for MyStatus {
    fn print(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("Msg From Plugin: {msg}");

        Ok(())
    }
}

fn main() {
    let mut bytes = Vec::new();
    for _ in 0..200 {
        bytes.extend([b'a', b'a', b'b', b'c']);
    }

    // let file_path = PathBuf::from(RUST_WASM_FILE_PATH);
    let file_path = PathBuf::from(C_WASM_FILE_PATH);

    assert!(file_path.exists(), "WASM File doesn't exist");

    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();

    let component = Component::from_file(&engine, file_path).unwrap();

    let ctx = WasiCtxBuilder::new().inherit_stdout().build();
    let table = ResourceTable::new();

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    Counting::add_to_linker(&mut linker, |s| s).unwrap();

    let mut store = Store::new(&engine, MyStatus::new(ctx, table));

    let (count, _instance) = Counting::instantiate(&mut store, &component, &linker).unwrap();

    let answer = count.call_count(&mut store, &bytes, Some(b'a'));

    println!("Host: Got {answer:?} from Plugin");
    println!("Finished");
}
