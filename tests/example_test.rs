#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

use esp32c6_hal as _; // exception handler
//use panic_probe as _; // semihosting::process::abort on test failure
use esp_backtrace as _;

#[cfg(test)]
#[embedded_test::tests]
mod unit_tests {

    #[test]
    fn it_works() {
        esp_println::logger::init_logger_from_env();
        assert!(true)
    }


    #[test]
    #[cfg(abc)]
    fn it_works2() {
        esp_println::logger::init_logger_from_env();
        assert!(false)
    }

    #[test]
    #[ignore]
    #[cfg(not(abc))]
    fn it_works3() {
        esp_println::logger::init_logger_from_env();
        assert!(false)
    }

    #[test]
    #[should_error]
    #[cfg(not(abc))]
    fn it_works4() {
        esp_println::logger::init_logger_from_env();
        assert!(false)
    }
}
