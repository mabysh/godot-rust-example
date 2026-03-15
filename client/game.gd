extends Node

@export var server_host: String = "127.0.0.1"
@export var server_port: int = 9050

func _ready() -> void:
	var peer := ENetMultiplayerPeer.new()
	var err := peer.create_client(server_host, server_port)
	if err != OK:
		push_error("Game: failed to connect to %s:%d - error %s" % [server_host, server_port, error_string(err)])
		return
	var multiplayer_api := get_tree().get_multiplayer()
	multiplayer_api.set_multiplayer_peer(peer)
	multiplayer_api.connected_to_server.connect(_on_connected_to_server)
	print("Game: connecting to %s:%d" % [server_host, server_port])

func _on_connected_to_server() -> void:
	rpc_id(1, "hello")

@rpc("any_peer", "call_remote", "reliable")
func hello() -> void:
	print("Game: hello from peer ", multiplayer.get_remote_sender_id())
