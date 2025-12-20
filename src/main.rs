#![no_std]
#![no_main]

mod hardware;

use embassy_executor::Spawner;
use embassy_net::Stack;
use hardware::error::Error;
use hardware::run;
use hardware::wiznet;
use panic_halt as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let stack = setup(&spawner).await.unwrap();
    run::run(stack).await;
}

async fn setup(spawner: &Spawner) -> Result<Stack<'static>, Error> {
    let (stack, ethernet_runner, network_runner) = hardware::init().await?;

    spawner
        .spawn(ethernet_task(ethernet_runner))
        .map_err(|_| Error::SpawnTask)?;

    spawner
        .spawn(network_task(network_runner))
        .map_err(|_| Error::SpawnTask)?;

    Ok(stack)
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
