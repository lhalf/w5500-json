#[derive(Debug)]
pub enum Error {
    Spi,
    WiznetEthernet,
    BindPort,
    SpawnTask,
}
