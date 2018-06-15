#![no_std]
#![feature(asm)]

pub mod raw;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
