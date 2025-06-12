use queues::IsQueue;
use ratatui::{
    layout::{Constraint, Layout, Direction, Rect, Alignment},
    style::{Color, Style},
    text::{Text, Line},
    widgets::{Block, Paragraph, Clear},
    Frame,
};
use ratatui::style::Stylize;
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

    let height = chunks[0].height;

    for i in 0..app.data.size() {
        let message = app.data.remove().unwrap();
        let pseudo_end = &message.chars().position(|c| c == ']').unwrap_or(0);
        let text_item = if *pseudo_end != 0 {
            let pseudo = message[1..*pseudo_end].to_string();
            let no_pseudo = message[pseudo_end+1..message.len()].to_string();
            ListItem::new(Line::from(vec![
                ("[".to_owned() + &*pseudo + "]").fg(Color::Indexed(app.registered_users.get(&pseudo).unwrap_or(&255).to_owned())),
                no_pseudo.fg(Color::White),
            ]))
        } else {
            ListItem::new(Text::styled(message.clone(), Style::default().fg(Color::White).bg(Color::Black)))
        };

        if app.offset == -1 {
            let mut start = (app.data.size() as isize - height as isize + 3) as i32;
            if start < 0 {start = 0;}
            if (i >= start as usize) && i <= app.data.size() {
                messages.push(text_item);
            }
        }
        else if !((i < (std::cmp::max(0, app.offset) as usize)) || (i >= (std::cmp::max(0, app.offset) +  (height as i32)) as usize)) {
            messages.push(ListItem::new(Text::styled(message.clone(), Style::default().fg(Color::White).bg(Color::Black))));
        }
        app.data.add(message).expect("should add back to queue");
    }

    let list = List::new(messages).block(title_block).style(Style::default().fg(Color::Blue).bg(Color::Black));

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(list, chunks[0]);

    if app.stream.is_none(){
        let popup : Block = if app.ip.is_none(){
            Block::default()
                .title("Please enter IP with port : <ip>:<port>")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::LightRed).bg(Color::Black))           
        } else {
            Block::default()
                .title("Please enter username")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::LightRed).bg(Color::Black))
        };
        let input = Paragraph::new(Text::styled(&app.input, Style::default().fg(Color::White).bg(Color::Black)))
            .block(popup)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left);
        
        let popup_chunks = centered_rect(50, 10, chunks[0]);
        
        frame.render_widget(Clear, popup_chunks);
        frame.render_widget(input, popup_chunks);
    }
    
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .title(match app.mode { 
            ModeType::Normal => "Normal Mode",
            ModeType::Insert => "Insert Mode",
        })
        .style(Style::default().fg(match app.mode {
            ModeType::Normal => Color::Green,
            ModeType::Insert => Color::Yellow,
        }));
    
    let input_text= if app.stream.is_some() {
        app.input.clone()
    } else if !app.failed_connection {
        "Press 'i' to enter insert mode".to_string()
    } else{
        "Failed to connect, verify IP and port, press esc to return in normal mode and q to quit".to_string()
    };
    
    let input = Paragraph::new(Text::styled(input_text, Style::default().fg(Color::White).bg(Color::Black)))
        .block(footer_block)
        .style(Style::default().fg(match app.mode {
            ModeType::Normal => Color::Green,
            ModeType::Insert => Color::Yellow,
        }).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    
    frame.render_widget(input, chunks[1]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}