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
    stream: TcpStream,
    offset: i32,
    registered_users: HashMap<String, u8>,
    data: CircularBuffer<String>,
    input: String,
    mode: ModeType,
    exit: bool,
}

impl App {
    
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            offset: -1,
            registered_users: HashMap::new(),
            data: CircularBuffer::new(128),
            input: String::new(),
            mode: ModeType::Normal,
            exit: false,
        }
    }
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        
        self.registered_users.insert("Myself".to_string(), 21);
        let mut flip = false;
        while !self.exit {
            let buffer = &mut [0; 1024];
            let len = self.stream.read(buffer).unwrap_or_else(|_| {return 0});
            if len > 0 && buffer[0] != "\0".as_bytes()[0] {
                let message : String = String::from_utf8_lossy(buffer).replace("\0", "").trim().parse().unwrap();
                self.data.add(message.clone()).expect("should add to queue");
                let pseudo_end = message.chars().position(|c| c == ']').unwrap();
                let pseudo = message[1..pseudo_end].to_string();
                if !self.registered_users.contains_key(&pseudo) && !(pseudo.contains("discussion") || pseudo.contains("rejoint") || pseudo.contains("quitté")) {
                    self.registered_users.insert(pseudo, COLORS[self.registered_users.len() % COLORS.len()]);
                }
            }
            terminal.draw(|frame| ui::ui(frame, self) )?;
            self.poll_event(&mut terminal)?;
            if !flip {terminal.clear()?; flip = true;}
        }
        dbg!(&self.registered_users);
        Ok(())
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
                        self.stream.write((self.input.clone() +"\r\n" ).as_ref()).expect("should write to upstream");
                        self.data.add("[Myself] ".to_string() + &*self.input.clone() + "\r\n").expect("should add to queue");
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
}