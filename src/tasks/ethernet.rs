use crate::wiznet;

#[embassy_executor::task]
pub async fn ethernet_task(runner: wiznet::Runner) {
    runner.run().await
}
