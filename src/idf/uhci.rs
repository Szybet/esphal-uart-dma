use crate::idf::{dma::gdma::gdma_new_ahb_channel, UHCI_ISR_CACHE_SAFE};

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/uart_types.h#L38
#[derive(Default)]
pub enum UartPort {
    #[default]
    UartNum0,
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/include/driver/uhci.h#L34
// size_t is u32, probably
#[derive(Default)]
pub struct uhci_controller_config_t {
    uart_port: UartPort,
    tx_trans_queue_depth: u32,
    max_transmit_size: u32,
    max_receive_internal_mem: u32,
    dma_burst_size: u32,
    max_packet_receive: u32,
}

// It's the same as dw_gdma_channel_t, alias
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/include/esp_private/dw_gdma.h#L20
// so dw_gdma_channel_t
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/dw_gdma.c#L91
pub struct gdma_channel_handle_t {
    // TODO!
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/gdma_types.h#L39
pub enum gdma_channel_direction_t {
    GDMA_CHANNEL_DIRECTION_TX,
    GDMA_CHANNEL_DIRECTION_RX,
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/include/esp_private/gdma.h#L32-L35
pub struct gdma_channel_alloc_config_t_flags {
    reserve_sibling: bool,
    isr_cache_safe: bool,
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/include/esp_private/gdma.h#L36
pub struct gdma_channel_alloc_config_t {
    sibling_chan: Option<gdma_channel_handle_t>,
    direction: gdma_channel_direction_t,
    flags: gdma_channel_alloc_config_t_flags,
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/include/driver/uhci_types.h#L24
// uhci_controller_handle_t is simply a pointer to uhci_controller_t
// Here it's defined:
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/src/uhci_private.h#L100

struct uhci_tx_dir {

}

struct uhci_rx_dir {
    dma_chan: gdma_channel_handle_t,
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/include/driver/uhci_types.h#L69
    // Ugh
    // on_rx_trans_event: 
    dma_link: 
    // gdma_link_list_handle_t -> gdma_link_list_t For fucks sake stop doing defines for stupid pointers
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/include/esp_private/gdma_link.h#L19
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/gdma_link.c#L51-L60
    // 
}

struct uhci_controller_t {
    uhci_num: i32,
    //  hal: uhci_hal_context_t,
    // Uhm idk maybe no
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/src/uhci_private.h#L102C5-L102C23
    // goes here: https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/uhci_hal.h#L33
    // Then goes here:
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/uhci_hal.h#L26
    // And that's registers: 
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/soc/esp32s2/register/soc/uhci_struct.h#L352
    // This link is for s2 but the point stands
    tx_dir: uhci_tx_dir,
    rx_dir: uhci_rx_dir,
    // user_data: *mut std::ffi::c_void,
    int_mem_cache_line_size: u32, // Not sure about type
    ext_mem_cache_line_size: u32, // Not sure about type
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/src/uhci.c#L172
pub fn uhci_gdma_initialize(uhci_config: uhci_controller_config_t) {
    let tx_alloc_config = gdma_channel_alloc_config_t {
        sibling_chan: None,
        direction: gdma_channel_direction_t::GDMA_CHANNEL_DIRECTION_TX,
        flags: gdma_channel_alloc_config_t_flags {
            reserve_sibling: false,
            isr_cache_safe: UHCI_ISR_CACHE_SAFE,
        }
    };
    // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_driver_uart/src/uhci.c#L181C25-L181C45
    // gdma_new_ahb_channel(&tx_alloc_config);
}
