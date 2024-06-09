use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let listener = TcpListener::bind("::1:7878").unwrap();
    // let stream = TcpStream::connect("::1:7878").unwrap();

    // stream.

    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();

    //     let buf_reader = BufReader::new(&mut stream);
    //     let response = buf_reader
    //         .lines()
    //         .flatten()
    //         .take_while(|line| !line.is_empty())
    //         .collect::<Vec<_>>();

    //     println!("response: {:?}", response);
    // }

    let mut buffer = String::new();

    loop {
        std::io::stdin().read_line(&mut buffer)?;

        match buffer
            .clone()
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .as_slice()
        {
            ["exit", ..] => break,
            ["create"] => {
                let listener = TcpListener::bind(format!("::1:0"))?;
                println!("Listening on port {}", listener.local_addr()?);

                for stream in listener.incoming() {
                    buffer.clear();

                    stream?.take(1024).read_to_string(&mut buffer)?;

                    println!("{}", buffer.trim());
                }
            }
            ["join", port] => {
                println!("Connected to port {}", port);
                let address = format!("::1:{}", port);

                loop {
                    let mut stream = TcpStream::connect(address.clone())?;

                    buffer.clear();

                    std::io::stdin().read_line(&mut buffer)?;

                    stream.write_all(buffer.clone().as_bytes())?;
                }
            }
            _ => {}
        }

        buffer.clear();
    }

    Ok(())
}
