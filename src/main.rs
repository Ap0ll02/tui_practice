use std::{io};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Styled, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

// Create an App struct first
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    render_count: u32,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.render_count += 1;
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Left | KeyCode::Char('+') => self.increment_counter(),
            KeyCode::Right | KeyCode::Char('-') => self.decrement_counter(),
            _ => {}
        }
    }

    fn decrement_counter(&mut self) {
        if self.counter == 0 {
            self.counter = 0;
        } else {
            self.counter -= 1;
        }
    }

    fn increment_counter(&mut self) {
            self.counter += 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(8),
                Constraint::Min(2),
            ])
            .margin(3)
            .spacing(5)
            .split(area);
        let instructions = Line::from(vec![
            " + ".blue().into(),
            " - ".red().into(),
            " q ".yellow().into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
                "Value: ".green().into(),
                self.counter.to_string().light_blue(),
        ])]);

        let render_text = 
            Line::from(vec![
                " Render Counter ".blue().into(),
            ]); 


        let text_time = Line::from(self.render_count.to_string().cyan());
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(chunks[0] , buf);

        let b = Block::default()
            .title(render_text)
            .padding(ratatui::widgets::Padding::vertical(2))
            .style(Style::new().cyan().bold())
            .border_type(ratatui::widgets::BorderType::Rounded)
            .borders(Borders::ALL);
        Paragraph::new(text_time)
            .centered()
            .block(b)
            .render(chunks[1], buf);
    }
}

