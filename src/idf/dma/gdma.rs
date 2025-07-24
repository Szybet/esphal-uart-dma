use crate::idf::{hal::gdma_hal_ahb_v1::gdma_ahb_hal_init, uhci::{gdma_channel_alloc_config_t, gdma_channel_handle_t}};

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/gdma_hal.h#L37-L39
pub struct gdma_hal_config_t_flags {
    enable_weighted_arbitration: u32,
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/include/hal/gdma_hal.h#L35-L40
pub struct gdma_hal_config_t {
    group_id: i32,
    flags: gdma_hal_config_t_flags,
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/gdma.c#L75
pub struct gdma_channel_search_info_t {
    bus_id: i32,
    start_group_id: i32,
    end_group_id: i32,
    pairs_per_group: i32,
    // hal_init: dyn Fn(gdma_hal_config_t), // Todo
}

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/soc/esp32c6/include/soc/gdma_channel.h#L21
pub const SOC_GDMA_BUS_AHB: i32 = 0;
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/esp32c6/include/hal/gdma_ll.h#L46
pub const GDMA_LL_AHB_GROUP_START_ID: i32 = 0;
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/esp32c6/include/hal/gdma_ll.h#L47
pub const GDMA_LL_AHB_NUM_GROUPS: i32 = 1;
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/esp32c6/include/hal/gdma_ll.h#L48
pub const GDMA_LL_AHB_PAIRS_PER_GROUP: i32 = 3;

// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/gdma.c#L193
// SOC_AHB_GDMA_SUPPORTED is enabled
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/soc/esp32c6/include/soc/soc_caps.h#L31
pub fn gdma_new_ahb_channel(config: gdma_channel_alloc_config_t, ret_chan: gdma_channel_handle_t) {
    let search_info = gdma_channel_search_info_t {
        bus_id: SOC_GDMA_BUS_AHB,
        start_group_id: GDMA_LL_AHB_GROUP_START_ID,
        end_group_id: GDMA_LL_AHB_GROUP_START_ID + GDMA_LL_AHB_NUM_GROUPS,
        pairs_per_group: GDMA_LL_AHB_PAIRS_PER_GROUP,
        // Ok now we need gdma_ahb_hal_init
        // But from where? There are 2
        // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/gdma_hal_ahb_v1.c
        // https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/gdma_hal_ahb_v2.c
        // Here: https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/hal/CMakeLists.txt#L152-L158
        // So it's CONFIG_SOC_AHB_GDMA_VERSION
        // It's 1 for esp32c6: https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/soc/esp32c6/include/soc/Kconfig.soc_caps.in#L442-L444
        // hal_init: gdma_ahb_hal_init,
        // Rust doesn't like it, just call it statically for now
    };
    do_allocate_gdma_channel(&search_info, config, ret_chan);
}

// Well, shit
// https://github.com/espressif/esp-idf/blob/346870a3044010f2018be0ef3b86ba650251c655/components/esp_hw_support/dma/gdma.c#L77-L190
pub fn do_allocate_gdma_channel(search_info: &gdma_channel_search_info_t, config: gdma_channel_alloc_config_t, ret_chan: gdma_channel_handle_t) {

}