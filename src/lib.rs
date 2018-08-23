#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate capnp;
extern crate protobuf;

pub mod bench;
pub mod bench_capnp;

pub use bench as bench_protobuf;
// pub struct Basic {
//     id: u64
// }

// pub struct Complex<'a> {
//     name: String,
//     basic: Basic,
//     reference: &'a str,
// }

