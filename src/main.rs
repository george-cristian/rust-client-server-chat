use tokio::net::TcpListener;

const BUFFER_SIZE:usize = 1024;

#[tokio::main]
async fn main() {
    //Suspend the current task until our future is ready to be returned
    let listener_result = TcpListener::bind("localhost:8080").await;

    let listener = match listener_result {
        Ok(listener) => listener,
        Err(error) => panic!("Unable to create TCP listener on port 8080: {:?}", error)
    };

    // Accept any new incoming connections, and get the socket and address
    let (socket, addr) = listener.accept().await.unwrap();

    let mut buffer = [0u8; BUFFER_SIZE];
}
