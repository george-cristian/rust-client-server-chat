use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

const NR_OF_CLIENTS:usize = 10;

#[tokio::main]
async fn main() {
    //Suspend the current task until our future is ready to be returned
    let listener_result = TcpListener::bind("localhost:8888").await;

    let listener = match listener_result {
        Ok(listener) => listener,
        Err(error) => panic!("Unable to create TCP listener on port 8080: {:?}", error)
    };

    let (tx, _rx) = broadcast::channel::<String>(NR_OF_CLIENTS);

    loop {
        // Accept any new incoming connections, and get the socket and address
        let (mut socket, _addr) = listener.accept().await.unwrap();
        
        // Clone tx because otherwise you run into compile error because of move of tx
        let tx = tx.clone();
        // Get a new receiver from the channel
        let mut rx = tx.subscribe();

        // Create a new async task for the newly connected client
        // Note the presence of async move which basically says that the code block is an async
        // task so you do not need to write it as a separate function
        tokio::spawn(async move {
            // Get split sockets because then we run into moving errors
            let (read_socket, mut write_socket) = socket.split();

            let mut buf_reader = BufReader::new(read_socket);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = buf_reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        // Broadcast the message
                        tx.send(line.clone()).unwrap();
                        // Clear the contents of the string because otherwise it will just append the next message to the existing string
                        line.clear();
                    }
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        write_socket.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
    
}
