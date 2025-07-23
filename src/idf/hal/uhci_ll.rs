use esp32c6::{PCR, UHCI0};

// https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/hal/esp32c6/include/hal/uhci_ll.h#L37
pub fn uhci_ll_enable_bus_clock(state: bool) {
    // let reg: &esp32c6::uhci0::RegisterBlock = uhci.register_block();
    // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/soc/esp32c6/ld/esp32c6.peripherals.ld#L57
    let pcr = unsafe { PCR::steal() };
    pcr.uhci_conf().modify(|_, w| {
        if state {
            w.uhci_clk_en().set_bit()
        } else {
            w.uhci_clk_en().clear_bit()
        }
    });
}

pub fn uhci_ll_reset_register() {
    let pcr = unsafe { PCR::steal() };
    pcr.uhci_conf().modify(|_, w| w.uhci_rst_en().set_bit());
    pcr.uhci_conf().modify(|_, w| w.uhci_rst_en().clear_bit());
}

// https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/hal/esp32c6/include/hal/uhci_ll.h#L55
pub fn uhci_ll_init() {
    let uhci = unsafe { UHCI0::steal() };
    // Conf 0 doesn't mean its for uart 0 because:
    // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/hal/esp32c6/include/hal/uhci_ll.h#L67-L68

    // Wtf is this logic
    let mut conf0_reg = uhci.conf0().read();
    uhci.conf0().modify(|_, w| w.clk_en().set_bit());
    // What is val? it's not in rust
    // Rust doc for this reg: https://docs.rs/esp32c6/0.21.0/esp32c6/uhci0/conf0/type.W.html#method.clk_en
    // Main idf struct: https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/soc/esp32c6/register/soc/uhci_struct.h#L621
    // So we are searching for uhci_conf0_reg_t
    // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/soc/esp32c6/register/soc/uhci_struct.h#L79
    // Okay wtf
    // That explained nothing, well, page 798
    // https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf#chapter.27
    // It allows access either as a full 32-bit value (val) or via named bitfields like tx_rst, rx_rst, etc. Each field controls specific UHCI behavior such as reset, CRC, framing, and UART interface linking.
    // This doesn't make any fucking sense
    // It sets registers to 0 except clk_on
    // but others have also default value to 1
    // Idk maybe it's defaults for power management but not for being on or smth

    // To replicate the exact code, I leave it as it is above - for idk a cycle it is clk to 1 but the rest is at default, just like in idf
    uhci.conf0().modify(|_, w| {
        unsafe { w.bits(0) };
        w.clk_en().set_bit()
    });
    uhci.conf1().modify(|_, w| {
        unsafe { w.bits(0) }
    });
}
