#![no_std]
#![no_main]

mod hardware;

use embassy_executor::Spawner;
use hardware::run;
use hardware::wiznet;
use panic_halt as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let (stack, ethernet_runner, network_runner) = hardware::init().await;

    spawner.spawn(ethernet_task(ethernet_runner)).unwrap();
    spawner.spawn(network_task(network_runner)).unwrap();

    run::run(stack).await;
}

#[embassy_executor::task]
pub async fn ethernet_task(runner: wiznet::Runner) {
    runner.run().await
}

#[embassy_executor::task]
pub async fn network_task(
    mut runner: embassy_net::Runner<'static, embassy_net_wiznet::Device<'static>>,
) {
    runner.run().await
}
