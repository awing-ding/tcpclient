use queues::IsQueue;
use ratatui::{
    layout::*,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Clear},
    Frame,
};
use ratatui::widgets::{Borders, ListItem, List, Wrap};
use crate::app::{App, ModeType};

pub fn ui(frame: &mut Frame, app: &mut App){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Min(3),
        ])
        .split(frame.area());

    // ANCHOR: title_paragraph
    let title_block = Block::default()
        .title("TCP Client")
        .borders(Borders::ALL)
        .style(Style::default());

    let mut messages = Vec::<ListItem>::new();

    for _ in 0..app.data.size() {
        let message = app.data.remove().unwrap();
        messages.push(ListItem::new(Text::styled(message.clone(), Style::default().fg(Color::White).bg(Color::Black))));
        app.data.add(message).expect("should add back to queue");
    }

    let list = List::new(messages).block(title_block).style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(list, chunks[0]);
    
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .title(match app.mode { 
            ModeType::Normal => "Normal Mode",
            ModeType::Insert => "Insert Mode",
        })
        .style(Style::default());
    
    let input = Paragraph::new(app.input.clone())
        .block(footer_block)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    
    frame.render_widget(input, chunks[1]);
}