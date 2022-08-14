use super::{display::VISIBLE, get_deadlines, Deadline};
use std::path::PathBuf;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct VisibleDeadlines {
    pub all_deadlines: Vec<Deadline>,
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

        self.visible_number = std::cmp::min(VISIBLE, len);
        let mut visible_deadlines = Vec::with_capacity(self.visible_number);
        for _ in 0..self.visible_number {
            if let Some(deadline) = self.all_deadlines.pop() {
                visible_deadlines.push(deadline);
            } else {
                //Theoretically this case should not come
                return None;
            }
        }

        return Some(visible_deadlines);
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
