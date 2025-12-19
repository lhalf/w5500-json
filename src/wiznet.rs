use crate::board::Board;
use crate::tasks::ethernet::ethernet_task;
use embassy_executor::Spawner;
use embassy_net_wiznet::chip::W5500;
use embassy_net_wiznet::{Device, State};
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Spi};
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use static_cell::StaticCell;

const MAC_ADDRESS: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];

pub type Runner = embassy_net_wiznet::Runner<
    'static,
    W5500,
    ExclusiveDevice<Spi<'static, SPI0, Async>, Output<'static>, Delay>,
    Input<'static>,
    Output<'static>,
>;

pub async fn init(spawner: &Spawner, board: Board) -> Device<'static> {
    static STATE: StaticCell<State<8, 8>> = StaticCell::new();

    let (device, runner) = embassy_net_wiznet::new(
        MAC_ADDRESS,
        STATE.init(State::new()),
        ExclusiveDevice::new(board.spi, board.cs, Delay).unwrap(),
        board.w5500_int,
        board.w5500_reset,
    )
    .await
    .unwrap();

    spawner.spawn(ethernet_task(runner)).unwrap();

    device
}
