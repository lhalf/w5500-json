use crate::hardware::wiznet::Runner;
use embassy_net::Stack;
use embassy_net_wiznet::Device;

pub mod board;
pub mod network;
pub mod run;
pub mod wiznet;

pub async fn init() -> (
    Stack<'static>,
    Runner,
    embassy_net::Runner<'static, Device<'static>>,
) {
    let mut board = board::init();

    let seed = board.rng.next_u64();

    let (device, ethernet_runner) = wiznet::init(board).await;

    let (stack, network_runner) = network::init(device, seed).await;

    (stack, ethernet_runner, network_runner)
}
