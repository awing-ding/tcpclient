mod ui;

use std::collections::HashMap;
use queues::*;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    DefaultTerminal,
};

pub enum ModeType {
    Normal,
    Insert,
}

const COLORS: [u8; 212] = [
    22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 57, 58, 59, 60, 61, 62, 63, 64, 65,
    66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
    86, 87, 93, 94, 95, 96, 97, 98, 99, 100,
    101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
    111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
    141, 142, 143, 144, 145, 146, 147, 148, 149, 150,
    151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 164, 165, 166, 167, 168, 169, 170,
    171, 172, 173, 174, 175, 176, 177, 178, 179, 180,
    181, 182, 183, 184, 185, 186, 187, 188, 189, 190,
    191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
    211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
    221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];


pub struct App {
    ip: Option<String>,
    username: Option<String>,
    stream: Option<TcpStream>,
    connection_status: bool,
    failed_connection: bool,
    offset: i32,
    registered_users: HashMap<String, u8>,
    data: CircularBuffer<String>,
    input: String,
    mode: ModeType,
    exit: bool,
}

impl App {
    
    pub fn new(payload: Option<(String, String)>) -> Self {
        let mut stream: TcpStream;
        let mut opt_server_address: Option<String> = None;
        let mut opt_username: Option<String> = None;
        if payload.is_none() {
            Self {
                ip: opt_server_address,
                username: opt_username,
                stream: None,
                connection_status: false,
                failed_connection: false,
                offset: -1,
                registered_users: HashMap::new(),
                data: CircularBuffer::new(128),
                input: String::new(),
                mode: ModeType::Normal,
                exit: false,
            }
            
        } else {
            let (server_address, username) = payload.unwrap();
            opt_server_address = Some(server_address.clone());
            opt_username = Some(username.clone());
            stream = TcpStream::connect(server_address).expect("should connect to server");
            stream.set_read_timeout(Some(std::time::Duration::from_millis(50))).expect("should set read timeout");
            stream.write_all((username + "\r\n").as_ref()).expect("should write to server");
            let buf = &mut [0; 1024];
            stream.read_exact(buf).unwrap_or_default();
            
            Self {
                ip: opt_server_address,
                username: opt_username,
                stream: Option::from(stream),
                connection_status: true,
                failed_connection: false,
                offset: -1,
                registered_users: HashMap::new(),
                data: CircularBuffer::new(128),
                input: String::new(),
                mode: ModeType::Normal,
                exit: false,
            }
        }
        
        
    }
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        
        let mut flip = false;
        
        while !self.exit {
            if self.connection_status {
                self.read_data();
            }
            terminal.draw(|frame| ui::ui(frame, self) )?;
            self.poll_event(&mut terminal)?;
            if !flip {terminal.clear()?; flip = true;}
        }
        Ok(())
    }
    
    fn read_data(&mut self) {
        let buffer = &mut [0; 1024];
        let stream = self.stream.as_mut().expect("stream should be available");
        let len = stream.read(buffer).unwrap_or(0);
        if len > 0 && buffer[0] != "\0".as_bytes()[0] {
            let message : String = String::from_utf8_lossy(buffer).replace("\0", "").trim().parse().unwrap();
            self.data.add(message.clone()).expect("should add to queue");
            let pseudo_end = message.chars().position(|c| c == ']').unwrap_or(0);
            if pseudo_end != 0 {
                let pseudo = message[1..pseudo_end].to_string();
                if !(self.registered_users.contains_key(&pseudo) || pseudo.contains("discussion") || pseudo.contains("rejoint") || pseudo.contains("quitté")) {
                    self.registered_users.insert(pseudo, COLORS[self.registered_users.len() % COLORS.len()]);
                }
            }
        }
    }

    fn poll_event(&mut self, default_terminal: &mut DefaultTerminal) -> io::Result<()> {
        match event::poll(core::time::Duration::from_millis(50))?{
            true => self.handle_events(default_terminal),
            false => Ok(()),
        }
    }
    pub fn handle_events(&mut self, default_terminal: &mut DefaultTerminal) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                return Ok(());
            }
            match self.mode {
                ModeType::Normal => match key.code {
                    event::KeyCode::Char('q') => {
                        if self.connection_status {
                            self.stream.as_mut().unwrap().write_all("/exit\r\n".as_bytes()).unwrap_or_default();
                        }
                        self.exit = true;
                    }
                    event::KeyCode::Char('c') => {
                        default_terminal.clear()?;
                        let capacity = self.data.capacity();
                        self.data = CircularBuffer::new(capacity);
                    }
                    event::KeyCode::Char('i') => {
                        self.mode = ModeType::Insert;
                    }
                    event::KeyCode::Char('j') | event::KeyCode::Down => {
                        if self.offset < self.data.size() as i32 {
                            self.offset += 1;
                        }
                    }
                    event::KeyCode::Char('k') | event::KeyCode::Up => {
                        if self.offset > -1 {
                            self.offset -= 1;
                        }
                    }
                    _ => {}
                },
                ModeType::Insert => match key.code {
                    event::KeyCode::Esc => {
                        self.mode = ModeType::Normal;
                    }
                    event::KeyCode::Backspace => {
                        if !self.input.is_empty() {
                            self.input.pop();
                        }
                    }
                    event::KeyCode::Enter => {
                        if self.connection_status {
                            let stream = self.stream.as_mut().expect("stream should be available");
                            stream.write_all((self.input.to_owned() + "\r\n").as_ref()).expect("should write to upstream");
                            self.data.add("[".to_string() + self.username.as_mut().expect("Username should exist") + "] " + &self.input + "\r\n").expect("should add to queue");
                        }
                        else if self.ip.is_none() {
                            self.ip = Some(self.input.to_owned());
                        }
                        else if self.username.is_none() {
                            self.username = Some(self.input.to_owned());
                            let result = self.connect();
                            if result.is_err() {
                                self.failed_connection = true;
                                self.connection_status = false;
                                self.stream = None;
                                self.ip = None;
                                self.username = None;
                            } else {
                                self.failed_connection = false;
                            }
                        }
                        self.input.clear();
                    }
                    event::KeyCode::Char(value) => {
                        self.input.push(value);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    
    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.stream.is_some(){
            panic!("Already connected to a server !");
        } else if self.ip.is_some() && self.username.is_some() {
            let server_address = self.ip.as_ref().expect("IP should be available");
            let username = self.username.as_ref().expect("Username should be available");
            let mut stream = TcpStream::connect(server_address)?;
            stream.set_read_timeout(Some(std::time::Duration::from_millis(50))).expect("should set read timeout");
            stream.set_nonblocking(true).expect("should set nonblocking");
            stream.write_all(("".to_string() + username + "\r\n").as_ref()).expect("should write to server");
            let buf = &mut [0; 1024];
            stream.read_exact(buf).unwrap_or_default();
            self.stream = Some(stream);
            self.connection_status = true;
            self.registered_users.insert(username.to_string(), 21);
            Ok(())
        } else {
            panic!("IP or Username is not set!");
        }
    }
}