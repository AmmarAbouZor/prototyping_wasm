use exports::host::c_package::countt::Guest;

wit_bindgen::generate!({
    path: "../host/wit/count.wit",
    world: "count",
});

struct Component;

impl Guest for Component {
    type Counter = MyCounter;
}

struct MyCounter;

impl exports::host::c_package::countt::GuestCounter for MyCounter {
    fn new() -> Self {
        Self
    }

    fn count(&self, data: Vec<u8>, target: u8) -> u64 {
        data.iter().filter(|&&b| b == target).count() as u64
    }
}

export!(Component);
