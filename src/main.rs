#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::dma::{Channel, DmaRxBuf, DmaTxBuf};
use esp_hal::time::{Duration, Instant};
use esp_hal::uart::{AtCmdConfig, Config, RxConfig, Uart, UartRx, UartTx};
use esp_hal::uart_uhci::UhciPer;
use esp_hal::{dma_buffers, main};
use log::{error, info};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let config = Config::default()
        .with_rx(RxConfig::default().with_fifo_full_threshold(64))
        .with_baudrate(115200);

    let mut uart1 = Uart::new(peripherals.UART1, config)
        .unwrap()
        .with_tx(peripherals.GPIO2)
        .with_rx(peripherals.GPIO3);

    let (mut rx, mut tx) = uart1.split();

    for _ in 0..2 {
        info!("Before");
        tx.write(b"Before uart").unwrap();
        if rx.read_ready() {
            let mut buf: [u8; 64] = [0u8; 64];
            let size = rx.read_buffered(&mut buf).unwrap();
            info!("Before received on rx: {:?}", buf);
        } else {
            info!("Nothing on rx");
        }
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_secs(2) {}
    }

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let mut dma_rx = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let mut uhci = UhciPer::new(peripherals.UHCI0, peripherals.DMA_CH0);
    uhci.init();
    uhci.configure(&mut dma_rx);

    /*
    in_peri_sel
        fn set_peripheral(&self, peripheral: u8) {
            self.ch()
                .in_peri_sel()
                .modify(|_, w| unsafe { w.peri_in_sel().bits(peripheral) });
        }


    in

    AnyGdmaRxChannel
    */

    // How do I clear this buffer? dma_rx.as_mut_slice().fill(0); doesn't work, RxBuffer doesn't have anything interesting either
    loop {
        info!("After");
        let mut buf: [u8; 128] = [0; 128];
        let received = dma_rx.read_received_data(&mut buf);
        info!("Received bytes on DMA: {}", received);
        if received > 0 {
            let vec = buf.to_vec();
            match core::str::from_utf8(&vec) {
                Ok(x) => info!("Received DMA message: {}", x),
                Err(x) => error!("Error string: {}", x),
            }
        }
        tx.write(b"After uart").unwrap();
        /*
        if rx.read_ready() {
            let mut buf: [u8; 64] = [0u8; 64];
            let size = rx.read_buffered(&mut buf).unwrap();
            info!("After received on rx: {:?}", buf);
        } else {
            info!("Nothing on rx");
        }
        */
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_secs(2) {}
    }
}
