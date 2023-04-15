use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let ip = "127.0.0.1";
    let port = 5566;

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
    println!("[+] Listening on {}:{}", ip, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[+] Client connected.\n");

                let mut read_stream = stream.try_clone().unwrap();
                let mut write_stream = stream.try_clone().unwrap();

                let receive_handle = thread::spawn(move || {
                    let mut buffer = [0; 2048];

                    loop {
                        let mut arr = [0; 2];
                        let bytes_read = read_stream.read(&mut arr).unwrap();
                        if bytes_read == 0 {
                            break;
                        }
                        let recv_len = arr[0] as usize;

                        let mut bytes_received = 0;
                        let mut ptr = &mut buffer[..recv_len];

                        while bytes_received < recv_len {
                            let bytes_received_this_call = read_stream.read(ptr).unwrap();
                            if bytes_received_this_call <= 0 {
                                break;
                            }
                            bytes_received += bytes_received_this_call;
                            ptr = &mut ptr[bytes_received_this_call..];
                        }

                        let client_input = String::from_utf8_lossy(&buffer[..recv_len]);
                        println!("\nClient: {}", client_input.trim());
                    }

                    println!("Client disconnected.");
                });

                let send_handle = thread::spawn(move || {
                    let mut buffer = [0; 2048];
                    let mut input = String::new();

                    loop {
                        print!("You: ");
                        std::io::stdout().flush().unwrap();
                        input.clear();
                        std::io::stdin().read_line(&mut input).unwrap();
                        input = input.trim().to_string();
                        if input == "exit" {
                            break;
                        }

                        let len = input.len();
                        let arr = [len as u8, len as u8];

                        write_stream.write_all(&arr).unwrap();
                        write_stream.write_all(input.as_bytes()).unwrap();

                        buffer.iter_mut().for_each(|x| *x = 0);
                    }

                    println!("Disconnected from the client.");
                });

                receive_handle.join().unwrap();
                send_handle.join().unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
