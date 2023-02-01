# rust-client-server-chat
A client server chat application written in Rust using the Tokio async library. It allows for multiple clients to be connected, and one client sends a message, the other connected clients will receive it.

It makes use of the tokio broadcasting channels and tokio select statement to run the tasks asynchronously.
