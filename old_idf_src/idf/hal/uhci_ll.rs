use esp32c6::{PCR, UHCI0};

use crate::uhci;

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
    uhci.conf1().modify(|_, w| unsafe { w.bits(0) });
}

pub fn uhci_ll_attach_uart_port(uart_num: u32) {
    let uhci = unsafe { UHCI0::steal() };
    if uart_num == 0 {
        uhci.conf0().modify(|_, w| w.uart0_ce().set_bit());
    } else {
        uhci.conf0().modify(|_, w| w.uart0_ce().clear_bit());
    }

    if uart_num == 1 {
        uhci.conf0().modify(|_, w| w.uart1_ce().set_bit());
    } else {
        uhci.conf0().modify(|_, w| w.uart1_ce().clear_bit());
    }
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/uhci_types.h#L24
pub struct uhci_seper_chr_t {
    pub seper_chr: u8,
    pub sub_chr1: u8,
    pub sub_chr2: u8,
    pub sub_chr_en: bool,
}

pub fn uhci_ll_set_seper_chr(seper_char: uhci_seper_chr_t) {
    let uhci = unsafe { UHCI0::steal() };
    if seper_char.sub_chr_en {
        todo!();
    } else {
        uhci.conf0().modify(|_, w| w.seper_en().clear_bit());
        uhci.escape_conf().modify(|_, w| unsafe { w.bits(0) });
    }
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/esp32c6/include/hal/uhci_ll.h#L29C3-L29C19
pub const UHCI_RX_BREAK_CHR_EOF: u32 = 0x1;
pub const UHCI_RX_IDLE_EOF: u32 = 0x2;
pub const UHCI_RX_LEN_EOF: u32 = 0x4;
pub const UHCI_RX_EOF_MAX: u32 = 0x7;
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/esp32c6/include/hal/uhci_ll.h#L142-L153
pub fn uhci_ll_rx_set_eof_mode(eof_mode: u32) {
    let uhci = unsafe { UHCI0::steal() };

    // Unsure about != 0
    if (eof_mode & UHCI_RX_BREAK_CHR_EOF) != 0 {
        uhci.conf0().modify(|_, w| w.uart_rx_brk_eof_en().set_bit());
    }
    if (eof_mode & UHCI_RX_IDLE_EOF) != 0 {
        uhci.conf0().modify(|_, w| w.uart_idle_eof_en().set_bit());
    }
    if (eof_mode & UHCI_RX_LEN_EOF) != 0 {
        uhci.conf0().modify(|_, w| w.len_eof_en().set_bit());
    }
}
