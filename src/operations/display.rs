use std::{
    collections::VecDeque,
    io::{self, stdout},
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
    vec,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
//use log::info;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use super::{get_deadlines, Deadline};

pub const TICK: u64 = 500;
pub const VISIBLE: usize = 5;

pub fn display(file: &PathBuf) {
    enable_raw_mode().expect("Raw mode enable failed");
    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Terminal creation error");

    //Write the render loop here
    render(&mut terminal, file).unwrap();

    disable_raw_mode().expect("Raw mode enable failed");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )
    .unwrap();
}

#[derive(Debug)]
pub enum Input {
    Next,
    Previous,
    Nothing,
}

pub fn render<B: Backend>(terminal: &mut Terminal<B>, file: &PathBuf) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let size = terminal.size().unwrap();
    let mut render_info = RenderInfo::new(size, file);

    //Initial rendering
    tx.send(Input::Next).unwrap();

    loop {
        terminal.draw(|f| ui(f, &rx, &mut render_info))?;

        //TODO: Multithreading to seperate this
        if let Event::Key(input) = event::read()? {
            if KeyCode::Char('q') == input.code {
                return Ok(());
            } else if KeyCode::Char('n') == input.code {
                tx.send(Input::Next).unwrap();
            } else if KeyCode::Char('p') == input.code {
                tx.send(Input::Previous).unwrap();
            } else {
                tx.send(Input::Nothing).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(TICK));
    }
}

pub struct RenderInfo {
    pub visible_deadlines: VisibleDeadlines,
    pub chunks: Vec<Rect>,
}

impl RenderInfo {
    pub fn new(size: Rect, file: &PathBuf) -> Self {
        let visible_deadlines = VisibleDeadlines::new(file);
        let percent = (100 / visible_deadlines.visible_number).try_into().unwrap();
        let constraints = vec![Constraint::Percentage(percent); VISIBLE];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_ref())
            .split(size); //splits the given area into the smaller required areas

        return Self {
            visible_deadlines,
            chunks,
        };
    }
}

//TODO: Think of having p key for rendering previous things
pub fn ui<B: Backend>(frame: &mut Frame<B>, rx: &Receiver<Input>, render_info: &mut RenderInfo) {
    while let Ok(input) = rx.try_recv() {
        match input {
            Input::Next => {
                actual_render(frame, render_info);
            }
            //TODO: Go to prev rendered page
            Input::Previous => {}
            Input::Nothing => {}
        }
    }
}

///Creates the block, paragraph and renders them
pub fn actual_render<B: Backend>(frame: &mut Frame<B>, render_info: &mut RenderInfo) {
    let visible_deadlines = &mut render_info.visible_deadlines;
    let chunks = &render_info.chunks;
    if let Some(deadlines) = visible_deadlines.next() {
        for (i, deadline) in deadlines.into_iter().enumerate() {
            let block = Block::default()
                .title(format!("{}", deadline.course))
                .borders(Borders::ALL);

            let paragraph = Paragraph::new(format!(
                "Course: {}\nAssignment: {}\nDeadline: {}",
                deadline.course, deadline.assignment, deadline.date
            ))
            .block(block);

            frame.render_widget(paragraph, chunks[i])
        }
    } else {
        //End page if it is None
        let block = Block::default().borders(Borders::ALL).title("End");
        let paragraph = Paragraph::new("\n\nPress q to exit the application.").block(block);
        frame.render_widget(paragraph, frame.size());
    }
}

pub struct VisibleDeadlines {
    pub all_deadlines: VecDeque<Deadline>,
    pub visible_number: usize,
}

impl VisibleDeadlines {
    pub fn new(file: &PathBuf) -> Self {
        let all_deadlines = get_deadlines(file).deadlines;
        let visible_number = std::cmp::min(all_deadlines.len(), VISIBLE);
        return Self {
            all_deadlines,
            visible_number,
        };
    }
}

impl Iterator for VisibleDeadlines {
    type Item = Vec<Deadline>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.all_deadlines.len();
        if len == 0 {
            return None;
        }
        let mut visible_deadlines = Vec::new();

        self.visible_number = std::cmp::min(VISIBLE, len);
        for _ in 0..self.visible_number {
            if let Some(deadline) = self.all_deadlines.pop_back() {
                visible_deadlines.push(deadline);
            } else {
                //Theoretically this case should not come
                return None;
            }
        }

        return Some(visible_deadlines);
    }
}
