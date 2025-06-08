mod ui;

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

pub struct App {
    stream: TcpStream,
    offset: u16,
    data: CircularBuffer<String>,
    input: String,
    mode: ModeType,
    exit: bool,
}

impl App {
    
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            offset: 0,
            data: CircularBuffer::new(128),
            input: String::new(),
            mode: ModeType::Normal,
            exit: false,
        }
    }
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let mut flip = false;
        while !self.exit {
            let buffer = &mut [0; 1024];
            let len = self.stream.read(buffer).unwrap_or_else(|_| {return 0});
            if len > 0 && buffer[0] != "\0".as_bytes()[0] {
                self.data.add(String::from_utf8_lossy(buffer).replace("\0", "").trim().parse().unwrap()).expect("should add to queue");
            }
            terminal.draw(|frame| ui::ui(frame, self) )?;
            self.poll_event(&mut terminal)?;
            if !flip {terminal.clear()?; flip = true;}
        }
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
                        if self.offset < self.data.size() as u16 {
                            self.offset += 1;
                        }
                    }
                    event::KeyCode::Char('k') | event::KeyCode::Up => {
                        if self.offset > 0 {
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