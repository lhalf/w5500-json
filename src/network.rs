use embassy_executor::Spawner;
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4};
use static_cell::StaticCell;

const IP_ADDRESS: Ipv4Cidr = Ipv4Cidr::new(Ipv4Address::new(192, 168, 50, 40), 24);
const GATEWAY: Ipv4Address = Ipv4Address::new(192, 168, 50, 1);

use crate::tasks::net::net_task;

pub async fn init(
    spawner: &Spawner,
    device: embassy_net_wiznet::Device<'static>,
    seed: u64,
) -> Stack<'static> {
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: IP_ADDRESS,
        gateway: Some(GATEWAY),
        dns_servers: Default::default(),
    });

    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    spawner.spawn(net_task(runner)).unwrap();

    stack
}
