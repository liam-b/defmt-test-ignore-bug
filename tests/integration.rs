#![no_std]
#![no_main]

use defmt_test_ignore_bug as _; // memory layout + panic handler

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[ignore]
    #[test]
    fn it_does_not_work() {
        assert!(false, "should be ignored")
    }
}
