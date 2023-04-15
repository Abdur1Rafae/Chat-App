use std::io::{Read, Write};
use std::net::{TcpStream};
use std::thread;

fn main() {
    let ip = "127.0.0.1";
    let port = 5566;

    let mut stream = TcpStream::connect((ip, port)).unwrap();

    println!("[+] Connected to the server.\n");

    let mut buffer = [0; 2048];
    let mut input = String::new();
    let mut arr: [u8; 2] = [0; 2];

    let mut stream_clone = stream.try_clone().unwrap();
    // thread for sending messages
    let send_thread = thread::spawn(move || {
        loop {
            print!("You: ");
            std::io::stdout().flush().unwrap();
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input == "exit" {
                break;
            }
            let len = input.trim().len();
            arr[0] = len as u8;
            arr[1] = len as u8;
            stream_clone.write_all(&arr).unwrap();
            stream_clone.write_all(input.trim().as_bytes()).unwrap();
            buffer.iter_mut().for_each(|x| *x = 0);
        }
    });

    let mut stream_clone2 = stream.try_clone().unwrap();
    // thread for receiving messages
    let recv_thread = thread::spawn(move || {
        loop {
            let mut recv_len: usize = 0;
            let mut recv_arr: [u8; 2] = [0; 2];
            stream_clone2.read_exact(&mut recv_arr).unwrap();
            recv_len = recv_arr[0] as usize;
            let mut bytes_received: usize = 0;
            let mut ptr = &mut buffer[..recv_len];
            while bytes_received < recv_len as usize {
                let bytes_received_this_call = stream_clone2.read(ptr).unwrap();
                if bytes_received_this_call <= 0 {
                    break;
                }
                bytes_received += bytes_received_this_call;
                ptr = &mut ptr[bytes_received_this_call..];
            }
            let received_string = std::str::from_utf8(&buffer[..recv_len as usize]).unwrap();
            if received_string == "exit" {
                break;
            }
            println!("\nServer: {}", received_string);
            buffer.iter_mut().for_each(|x| *x = 0);
        }
    });

    // wait for both threads to finish before exiting the program
    send_thread.join().unwrap();
    recv_thread.join().unwrap();

    println!("Disconnected from the server\n");
}
