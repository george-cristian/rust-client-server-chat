use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() {
    //Suspend the current task until our future is ready to be returned
    let listener_result = TcpListener::bind("localhost:8888").await;

    let listener = match listener_result {
        Ok(listener) => listener,
        Err(error) => panic!("Unable to create TCP listener on port 8080: {:?}", error)
    };

    loop {
        // Accept any new incoming connections, and get the socket and address
        let (mut socket, _addr) = listener.accept().await.unwrap();
        
        // Create a new async task for the newly connected client
        // Note the presence of async move which basically says that the code block is an async
        // task so you do not need to write it as a separate function
        tokio::spawn(async move {
            // Get split sockets because then we run into moving errors
            let (read_socket, mut write_socket) = socket.split();

            let mut buf_reader = BufReader::new(read_socket);
            let mut line = String::new();

            loop {
                let bytes_read = buf_reader.read_line(&mut line).await.unwrap();

                // Check if the client disconnect and if there is no data left
                if bytes_read == 0 {
                    break;
                }

                write_socket.write_all(line.as_bytes()).await.unwrap();

                // Clear the contents of the string because otherwise it will just append the next message to the existing string
                line.clear();
            }
        });

    }
    
}
