pub mod decode;
pub mod exec;

pub use exec::{Computer, Device, ExecError, IOError, Stepped};

pub type Word = i32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
