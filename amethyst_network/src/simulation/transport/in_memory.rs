//! Network systems implementation backed by the Laminar network protocol.

use crate::simulation::{
    events::NetworkSimulationEvent,
    requirements::DeliveryRequirement,
    timing::{NetworkSimulationTime, NetworkSimulationTimeSystem},
    transport::{
        TransportResource, NETWORK_POLL_SYSTEM_NAME, NETWORK_RECV_SYSTEM_NAME,
        NETWORK_SEND_SYSTEM_NAME, NETWORK_SIM_TIME_SYSTEM_NAME,
    },
};
use amethyst_core::{
    bundle::SystemBundle,
    ecs::{DispatcherBuilder, Read, System, World, Write},
    shrev::EventChannel,
};
use amethyst_error::Error;
pub use laminar::{Config as LaminarConfig, Socket as LaminarSocket};
use laminar::{Packet, SocketEvent};

use bytes::Bytes;
use log::error;
use std::time::Instant;
use std::collections::VecDeque;

/// Use this network bundle to add an in-memory transport layer to your game.
/// No sockets or networks are used for this transport layer. Everything goes trough memory.
///
/// This can be valuable when you want to test your game or setup a client-server architecture for a single player game.
pub struct InMemoryNetworkBundle;

impl InMemoryNetworkBundle {
    /// Constructs a new in-memory transport layer.
    pub fn new() -> Self {
        Self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for InMemoryNetworkBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'_, '_>,
    ) -> Result<(), Error> {
        builder.add(InMemoryNetworkSendSystem, NETWORK_SEND_SYSTEM_NAME, &[]);
        builder.add(
            InMemoryNetworkRecvSystem,
            NETWORK_RECV_SYSTEM_NAME,
            &[NETWORK_POLL_SYSTEM_NAME],
        );
        builder.add(
            NetworkSimulationTimeSystem,
            NETWORK_SIM_TIME_SYSTEM_NAME,
            &[NETWORK_RECV_SYSTEM_NAME],
        );
        world.insert(InMemorySocketResource::new());
        Ok(())
    }
}

struct InMemoryNetworkSendSystem;

impl<'s> System<'s> for InMemoryNetworkSendSystem {
    type SystemData = (
        Write<'s, TransportResource>,
        Write<'s, InMemorySocketResource>,
        Read<'s, NetworkSimulationTime>,
    );

    fn run(&mut self, (mut transport, mut resource, sim_time): Self::SystemData) {
        let messages = transport.drain_messages_to_send(|_| sim_time.should_send_message_now());

        for message in messages.iter() {
            let packet = match message.delivery {
                DeliveryRequirement::Default => Packet::unreliable(
                    message.destination,
                    message.payload.to_vec(),
                ),
                delivery => panic!(
                    "{:?} is unsupported. In-memory transport layer can't have any guarantees.",
                    delivery
                ),
            };

            resource.send(packet);
        }
    }
}

struct InMemoryNetworkRecvSystem;

impl<'s> System<'s> for InMemoryNetworkRecvSystem {
    type SystemData = (
        Write<'s, InMemorySocketResource>,
        Write<'s, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut resource, mut event_channel): Self::SystemData) {
        while let Some(SocketEvent::Packet(packet)) = resource.recv() {
            event_channel.single_write(NetworkSimulationEvent::Message(
                packet.addr(),
                Bytes::from(packet.payload()),
            ));
        }
    }
}

/// Resource that owns the Laminar socket.
pub struct InMemorySocketResource {
    packets: VecDeque<Packet>
}

impl Default for InMemorySocketResource {
    fn default() -> Self {
        Self { packets: VecDeque::new() }
    }
}

impl InMemorySocketResource {
    /// Creates a new instance of the `UdpSocketResource`.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn send(&mut self, packet: Packet) {
        self.packets.push_back(packet);
    }

    pub fn recv(&mut self) -> Option<SocketEvent> {
        self.packets.pop_front().map(|x| SocketEvent::Packet(x))
    }
}
