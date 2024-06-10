wit_bindgen::generate!({
    path: "../host/wit/count.wit",
    world: "counting",
});

struct Component;

impl Guest for Component {
    fn count(bytes: _rt::Vec<u8>, target: Option<u8>) -> Result<u64, String> {
        print("Hello from Rust Plugin");

        panic!("test");
        let target = target.ok_or("No Target is provided".to_string())?;

        let count = bytes.iter().filter(|&&b| b == target).count();

        Ok(count as u64)
    }
}

export!(Component);
