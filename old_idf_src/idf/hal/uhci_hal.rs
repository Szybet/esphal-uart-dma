use crate::idf::hal::uhci_ll::uhci_ll_init;

// https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/hal/uhci_hal.c#L11
pub fn uhci_hal_init() {
    // Just execute uhci_ll_init
    uhci_ll_init();
}
