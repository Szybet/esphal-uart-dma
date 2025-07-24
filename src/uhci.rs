use crate::idf::{hal::{
    uhci_hal::uhci_hal_init,
    uhci_ll::{
        uhci_ll_attach_uart_port, uhci_ll_enable_bus_clock, uhci_ll_reset_register,
        uhci_ll_rx_set_eof_mode, uhci_ll_set_seper_chr, uhci_seper_chr_t, UHCI_RX_IDLE_EOF,
    },
}, uhci::uhci_gdma_initialize};

const UART_NUM: u32 = 0;

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
        uhci_ll_attach_uart_port(UART_NUM);

        // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/esp_driver_uart/src/uhci.c#L520-L524
        let seper_ch = uhci_seper_chr_t {
            seper_chr: 0,
            sub_chr1: 0,
            sub_chr2: 0,
            sub_chr_en: false,
        };
        uhci_ll_set_seper_chr(seper_ch);

        // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/esp_driver_uart/src/uhci.c#L526-L535
        // In examples, only idle_eof is used, so I will do only that for now
        // the rest is TODO

        // if (config->rx_eof_flags.idle_eof) {
        uhci_ll_rx_set_eof_mode(UHCI_RX_IDLE_EOF);
        // }
        /*
        if (config->rx_eof_flags.length_eof) {
            uhci_ll_rx_set_eof_mode(uhci_ctrl->hal.dev, UHCI_RX_LEN_EOF);
            uhci_ll_rx_set_packet_threshold(uhci_ctrl->hal.dev, config->max_packet_receive);
        }
        if (config->rx_eof_flags.rx_brk_eof) {
            uhci_ll_rx_set_eof_mode(uhci_ctrl->hal.dev, UHCI_RX_BREAK_CHR_EOF);
        }
        */

        // Idk
        /*
        esp_cache_get_alignment(MALLOC_CAP_SPIRAM, &uhci_ctrl->ext_mem_cache_line_size);
        esp_cache_get_alignment(MALLOC_CAP_INTERNAL, &uhci_ctrl->int_mem_cache_line_size);
        */

        // https://github.com/espressif/esp-idf/blob/bfe5caf58f742fd35c023335f475114a5b88761e/components/esp_driver_uart/src/uhci.c#L540C23-L540C43
        uhci_gdma_initialize(Default::default());

        Self {}
    }
}
