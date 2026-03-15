use godot::classes::{ENetMultiplayerPeer, INode, MultiplayerPeer, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node)]
struct Game {
    #[export]
    port: i32,

    base: Base<Node>,
}

#[godot_api]
impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        Self { port: 9050, base }
    }

    fn ready(&mut self) {
        let mut peer = ENetMultiplayerPeer::new_gd();
        let err = peer.create_server(self.port);
        if err != godot::global::Error::OK {
            godot_error!(
                "Game: failed to create ENet server on port {}: {:?}",
                self.port,
                err
            );
            return;
        }
        let Some(mut multiplayer) = self.base().get_multiplayer() else {
            godot_error!("Game: no multiplayer API");
            return;
        };
        let peer_upcast: Gd<MultiplayerPeer> = peer.upcast();
        multiplayer.set_multiplayer_peer(&peer_upcast);
        godot_print!("Game: ENet server listening on port {}", self.port);
    }
}

#[godot_api]
impl Game {
    #[rpc(any_peer, call_remote, reliable)]
    fn hello(&mut self) {
        let sender = self
            .base()
            .get_multiplayer()
            .map(|mut m| m.get_remote_sender_id());
        godot_print!("Game: hello from peer {:?}", sender);
    }
}
