#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn count(data: Vec<u8>, target: u8) -> u64 {
        data.iter().filter(|&&b| b == target).count() as u64
    }
}

bindings::export!(Component with_types_in bindings);
