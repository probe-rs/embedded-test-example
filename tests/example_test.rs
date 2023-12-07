#![no_std]
#![no_main]

use esp32c6_hal as _; // exception handler
use panic_probe as _; // semihosting::process::abort on test failure

#[cfg(test)]
#[embedded_test::tests]
mod unit_tests {

    #[test]
    #[cfg(feature="log")]
    fn log() {
        esp_println::logger::init_logger_from_env();
        log::info!("Hello, log!"); // Prints via esp-println to rtt
        assert!(true)
    }

    #[test]
    #[cfg(feature="defmt")]
    fn defmt() {
        use defmt_rtt as _;
        defmt::info!("Hello, defmt!"); // Prints via defmt-rtt to rtt
        assert!(true)
    }

    #[test]
    #[cfg(abc)]
    fn it_works_disabled() {
        assert!(false)
    }

    #[test]
    #[ignore]
    #[cfg(not(abc))]
    fn it_works_ignored() {
        assert!(false)
    }

    #[test]
    #[cfg(not(abc))]
    fn it_works() {
        assert!(false)
    }
}
