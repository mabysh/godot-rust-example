# Example: server + client (Godot + Rust)

- **server** – Godot 4 project that loads the Rust GDExtension (`server-lib`) and runs an ENet server.
- **client** – Godot 4 project that connects to the server and sends an RPC.
- **server-lib** – Rust crate (gdext) that builds the GDExtension used by the server.

## Prerequisites

- **Godot 4.x** (4.1+)
- **Rust** (for building the server extension)

## Build the server extension

From the repo root:

```bash
cd server-lib && cargo build
```

Debug build is fine too; the server project is wired to both debug and release libs via `server_lib.gdextension`.

## Run the server

1. Open the **server** project in Godot (`server/project.godot`).
2. Ensure the main scene has a **Game** node (from the server-lib extension) at the root. It will start the ENet server on `_ready()` (default port **9050**).
3. Run the project (Play or F5). You should see something like: `Game: ENet server listening on port 9050`.

## Run the client

1. With the server running, open the **client** project in Godot (`client/project.godot`).
2. Run the project (Play or F5). The client connects to `127.0.0.1:9050` and, after connecting, calls the `hello` RPC on the server.
3. In the server console you should see the client’s “hello” (e.g. peer id). In the client console you’ll see its own connection/hello output.

## Quick test (order of operations)

1. **Start server first**: open `server/` in Godot → Run.
2. **Then start client**: open `client/` in Godot → Run.

Change the server port or client host/port in the inspector on the respective **Game** nodes if needed.
