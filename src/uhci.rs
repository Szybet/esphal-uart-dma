use crate::idf::hal::{uhci_hal::uhci_hal_init, uhci_ll::{uhci_ll_enable_bus_clock, uhci_ll_reset_register}};

pub struct UhciPer {}

impl UhciPer {
    pub fn new(uhci: esp_hal::peripherals::UHCI0<'static>) -> Self {
        // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/esp_driver_uart/src/uhci.c#L463-L503
        // This is handled by:
        // The argument
        // No config yet

        // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/esp_driver_uart/src/uhci.c#L505
        // Smarter note: Critical section is the same everywhere
        critical_section::with(|_| {
            uhci_ll_enable_bus_clock(true);
            uhci_ll_reset_register();
        });

        // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/esp_driver_uart/src/uhci.c#L510
        uhci_hal_init();

        Self {}
    }
}