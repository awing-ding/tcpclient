mod app;

use std::io::{Read, Write};
use std::net::TcpStream;


fn connect(ip: String) -> TcpStream {
    TcpStream::connect(ip).expect("should connect to server")
}

fn parse_args(args: &[String]) -> Result<(String, String), String> {
    if args.len() < 3 {
        return Err("Usage: tcp_client <server_address:server_port> <username>".to_string());
    }

    
    // Here you can parse the server address or other arguments as needed
    Ok((args[1].clone(), args[2].clone()))
}

fn main() {
    color_eyre::install().expect("Should install color_eyre for logging");
    let args = std::env::args().collect::<Vec<String>>();
    let (server_address, username) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    let mut stream = connect(server_address.clone());
    stream.set_read_timeout(Some(std::time::Duration::from_millis(50))).expect("should set read timeout");
    stream.write((username.clone() + "\r\n").as_ref()).expect("should write to server");
    let buf = &mut [0; 1024];
    stream.read(buf).unwrap_or_else(|_| return 0);
    
    let mut terminal = ratatui::init();
    terminal.clear().expect("should clear terminal");
    let mut application = app::App::new(stream);
    application.run(terminal).expect("app should not fail");
    
}
    