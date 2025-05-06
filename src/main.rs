use std::{io};
use chrono::TimeDelta;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Styled, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
    DefaultTerminal, Frame,
};


// Create an App struct first
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    state: State,
    my_path: String,
}

#[derive(Default, Debug)]
pub enum State {
    #[default] Intro,
    Main,
    Exit,
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
            Event::Mouse(mouse_event) if mouse_event.kind == MouseEventKind::Moved => {
                let my_pa = self.my_update();
                self.update_path(my_pa);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('n') => self.state = State::Main,
            _ => {}
        }
    }

    fn update_path(&mut self, path: String) {
        self.my_path = path;
    }

    fn my_update(&self) -> String {
        let mut path: String = "OOOOOO".to_string();
        let speed = 1;
        for _ in 0..speed {
            path += " ";
        }
        path += "O";
        path
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" TOWER DEFENSE ULTIMATE ".blue().bold()).centered();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(9),
                Constraint::Percentage(70),
                Constraint::Percentage(21),
            ])
            .vertical_margin(1)
            .split(area);
        // ==============================
        // = BLOCK MANAGEMENT & PREFABS =
        // ==============================
        
        // Easy Border With Thick Rounded Green Borders
        let easy_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::new().green().bold())
            .border_type(ratatui::widgets::BorderType::Rounded);

        let b = Block::default()
            .title(title)
            .padding(Padding::vertical(2))
            .style(Style::new().cyan().bold())
            .border_type(ratatui::widgets::BorderType::Rounded)
            .borders(Borders::ALL);

        // =============
        // = RENDERING =
        // =============
        let begin = Paragraph::new("There Ya Go")
            .centered()
            .white()
            .block(easy_block.clone());

        let begin2 = Paragraph::new("I've Designed This Game To Teach You VIM Motions")
            .centered()
            .light_magenta()
            .block(b);

        let main_move = Paragraph::new(self.my_path.clone())
            .centered()
            .light_cyan()
            .block(easy_block.clone());

        // State Rendering
        match self.state {
            State::Intro => {
                begin.render(chunks[0], buf);
                begin2.render(chunks[1], buf);
            }
            State::Main => {
                main_move.render(chunks[1], buf);
            }
            _ => {}
        }
    }
}


