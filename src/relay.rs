use crate::socket::Socket;
use smoltcp::wire::{Ipv4Packet, UdpPacket};

pub async fn relay<'a>(socket: &impl Socket<'a>, buffer: &'a mut [u8; 4096]) {
    if let Ok(data) = socket.recv(buffer).await
        && let Ok(ipv4) = Ipv4Packet::new_checked(data)
        && let Ok(udp) = UdpPacket::new_checked(ipv4.payload())
        && serde_json_core::from_slice::<serde::de::IgnoredAny>(udp.payload()).is_ok()
    {
        defmt::info!("{:a}", udp.payload());
        let _ = socket.send(data).await;
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::socket::SocketSpy;
    use alloc::vec;
    use alloc::vec::Vec;
    use embassy_net::raw::RecvError;
    use smoltcp::wire::{IpProtocol, Ipv4Address, Ipv4Repr, UdpRepr};

    #[tokio::test]
    async fn packet_too_large_for_buffer_causes_nothing_to_be_sent() {
        let mut buffer = [0; 4096];

        let socket_spy = SocketSpy::default();

        socket_spy
            .recv
            .returns
            .set([Err(RecvError::from(RecvError::Truncated))]);

        relay(&socket_spy, &mut buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }

    #[tokio::test]
    async fn valid_json_packets_are_echoed() {
        let mut buffer = [0; 4096];

        let data = create_udp_packet(b"{}");

        let socket_spy = SocketSpy::default();

        socket_spy.recv.returns.set([Ok(data.as_slice())]);
        socket_spy.send.returns.set([()]);

        relay(&socket_spy, &mut buffer).await;

        assert_eq!([data.to_vec()], socket_spy.send.arguments);
    }

    #[tokio::test]
    async fn invalid_json_is_not_echoed() {
        let mut buffer = [0; 4096];

        let data = create_udp_packet(b"{");

        let socket_spy = SocketSpy::default();

        socket_spy.recv.returns.set([Ok(data.as_slice())]);

        relay(&socket_spy, &mut buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }

    fn create_udp_packet(payload: &[u8]) -> Vec<u8> {
        let src_addr = Ipv4Address::new(192, 168, 50, 1);
        let dst_addr = Ipv4Address::new(192, 168, 50, 40);

        let udp_repr = UdpRepr {
            src_port: 0,
            dst_port: 1,
        };

        let ip_repr = Ipv4Repr {
            src_addr,
            dst_addr,
            next_header: IpProtocol::Udp,
            payload_len: udp_repr.header_len() + payload.len(),
            hop_limit: 0,
        };

        let mut buffer = vec![0; ip_repr.buffer_len() + udp_repr.header_len() + payload.len()];

        ip_repr.emit(
            &mut Ipv4Packet::new_unchecked(&mut buffer[..ip_repr.buffer_len()]),
            &smoltcp::phy::ChecksumCapabilities::default(),
        );

        udp_repr.emit(
            &mut UdpPacket::new_unchecked(&mut buffer[ip_repr.buffer_len()..]),
            &src_addr.into(),
            &dst_addr.into(),
            payload.len(),
            |buf| buf.copy_from_slice(payload),
            &smoltcp::phy::ChecksumCapabilities::default(),
        );

        buffer
    }
}
