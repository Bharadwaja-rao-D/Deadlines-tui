use std::{
    io::{self, stdout},
    path::PathBuf,
    str::FromStr,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use chrono::{Datelike, NaiveDate};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
//use log::info;
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use super::{info::RenderInfo, Deadline};

pub const TICK: u64 = 500;
pub const VISIBLE: usize = 4;

pub fn display(file: &PathBuf) {
    //Offs options of the current terminal
    enable_raw_mode().expect("Raw mode enable failed");
    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Terminal creation error");

    //Write the render loop here
    render(&mut terminal, file).unwrap();

    //Turns on the default terminal things
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
    //Contains the deadlines to be rendered an the layout chunks on which it should be rendered.
    let mut render_info = RenderInfo::new(size, file);

    //Initial rendering
    tx.send(Input::Next).unwrap();

    loop {
        terminal.draw(|f| ui(f, &rx, &mut render_info))?;

        //Captures the input and sends it via channel
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

        //MISTAKE: If you click when it is here then there will be no change
        thread::sleep(Duration::from_millis(TICK));
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

fn construct_block(deadline: Deadline) -> Paragraph<'static> {
    let mut nearer = false;
    let mut completed = false;

    //if deadline.date is less than one day then make borders red
    let date_now = chrono::offset::Local::now();
    let date_now = date_now.naive_local().ordinal();
    let date_deadline = NaiveDate::from_str(&deadline.date).unwrap().ordinal();

    let diff = date_deadline.abs_diff(date_now);
    if date_deadline < date_now {
        completed = true;
    } else if diff <= 1 {
        nearer = true;
    }

    let mut block = Block::default()
        .title(format!("{}", deadline.course))
        .borders(Borders::ALL);

    //TODO: Find a way to remove the completed things
    if completed {
        block = block.border_style(Style::default().fg(Color::Red));
    } else if nearer {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    let paragraph = Paragraph::new(format!(
        "Course: {}\nAssignment: {}\nDeadline: {}",
        deadline.course, deadline.assignment, deadline.date
    ))
    .block(block);

    return paragraph;
}

//Creates the block, paragraph and renders them
pub fn actual_render<B: Backend>(frame: &mut Frame<B>, render_info: &mut RenderInfo) {
    let visible_deadlines = &mut render_info.visible_deadlines;
    let chunks = &render_info.chunks;
    //Constructed a custom iterator for visible_deadlines which returns a batch of deadlines based
    //on the value of VISIBLE
    if let Some(deadlines) = visible_deadlines.next() {
        for (i, deadline) in deadlines.into_iter().enumerate() {
            let paragraph = construct_block(deadline);
            frame.render_widget(paragraph, chunks[i])
        }
    } else {
        //End page if it is None
        let block = Block::default().borders(Borders::ALL).title("End");
        let paragraph = Paragraph::new("\n\nPress q to exit the application.").block(block);
        frame.render_widget(paragraph, frame.size());
    }
}
