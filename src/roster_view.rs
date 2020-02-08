use crate::roster::{Roster, Slot};
use cursive::{Vec2, Printer, XY};
use cursive::theme::ColorStyle;
use cursive::event::{Event, EventResult, Key};
use cursive::direction::Direction;
use std::cmp::max;
use num::clamp;

pub struct RosterView {
    roster: Roster,
    job_names: Vec<String>,
    person_names: Vec<String>,
    focused: Option<Vec2>,
}

impl RosterView {
    pub fn new(roster: &Roster, job_names: Vec<String>, person_names: Vec<String>) -> RosterView {
        RosterView {
            job_names,
            person_names,
            roster: roster.clone(),
            focused: None,
        }
    }

    fn job_cols_needed(&self) -> u32 {
        self.roster.jobs.iter().map(|job| job.num_people).sum()
    }

    fn max_col_width(&self) -> u32 {
        max(
            self.job_names.iter().map(|job_name| job_name.len()).max().unwrap(),
            self.person_names.iter().map(|person_name| person_name.len()).max().unwrap()
        ) as u32
    }

    fn get_fields_for_week(&self, week: u32) -> Vec<Option<String>> {
        let mut res = vec![];

        for job in &self.roster.jobs {
            let slot = Slot {
                week,
                job: *job
            };
            let mut persons: Vec<Option<String>> =  self.roster.get_assigned(&slot).iter().map(|&person| Some(self.person_names[person as usize].clone())).collect();
            let empty_fields = job.num_people - (persons.len() as u32);
            res.append(&mut persons);
            for _ in 0..empty_fields {
                res.push(None)
            }
        }

        res
    }

    fn draw_field(&self, printer: &Printer<'_, '_>, pos: Vec2, contents: &str) {
        let max_col_width = self.max_col_width();
        let pos = (pos.x * max_col_width as usize + pos.x, pos.y);
        printer.print(pos, contents);
    }

    fn draw_column_header(&self, printer: &Printer<'_, '_>, index: u32, title: &str) {
        let max_col_width = self.max_col_width();

        printer.print(((index + 1) * max_col_width + index + 1, 0), title);
    }

    fn draw_row_header(&self, printer: &Printer<'_, '_>, index: u32, title: &str) {
        let max_col_width = self.max_col_width();

        printer.print((0, index), title);
    }

    fn move_focus(&mut self, delta: (i32, i32)) -> bool {
        assert!(self.focused.is_some());

        let old_focused = self.focused;

        self.focused = self.focused.map(|focus| {
            Vec2::new(
                clamp(focus.x as i32 + delta.0, 0, self.job_cols_needed()  as i32 - 1) as usize,
                clamp(focus.y as i32 + delta.1, 0, self.roster.num_weeks as i32 - 1)  as usize
            )
        });

        old_focused != self.focused
    }
}

impl cursive::view::View for RosterView {
    fn draw(&self, printer: &Printer<'_, '_>) {
        // Draw headers
        let mut current_job_start: u32 = 0;
        for job in &self.roster.jobs {
            for i in 0..job.num_people {
                self.draw_field(printer, Vec2::new((current_job_start + i + 1) as usize, 0), &self.job_names[job.id as usize]);
            }
            current_job_start += job.num_people;
        }

        // Draw week numbers
        for i in 0..self.roster.num_weeks {
            self.draw_field(printer, Vec2::new(0, i as usize + 1), &format!("Week {}", i + 1));
        }

        // Construct "empty" string
        let mut empty_str = "".to_owned();
        for i in 0..self.max_col_width() {
            empty_str.push('_');
        }

        // Fill in roster assignments
        for week in 0..self.roster.num_weeks {
            for (job_index, name) in self.get_fields_for_week(week).into_iter().enumerate() {
                let style = if self.focused == Some(Vec2::new(job_index, week as usize)) {
                    if printer.focused {
                        ColorStyle::highlight()
                    } else {
                        ColorStyle::highlight_inactive()
                    }
                } else {
                    ColorStyle::primary()
                };

                let person_name = name.as_ref().unwrap_or(&empty_str);

                printer
                    .with_style(style, |printer| {
                        self.draw_field(printer, Vec2::new(job_index + 1, week as usize + 1), person_name);
                    });
            }
        }
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        let _ = constraint;
        let height = self.roster.num_weeks + 1; // Including header
        let width = (self.job_cols_needed() + 1) * self.max_col_width() + self.job_cols_needed(); // Including Week on the side and spacers inbetween
        Vec2::new(width as usize, height as usize)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        match e {
            Event::Key(k) => {
                let delta = match k {
                    Key::Left => {
                        Some((-1, 0))
                    },
                    Key::Right => {
                        Some((1, 0))
                    },
                    Key::Up => {
                        Some((0, -1))
                    },
                    Key::Down => {
                        Some((0, 1))
                    },
                    _ => None,
                };

                if let Some(delta) = delta {
                    if self.move_focus(delta) {
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                } else {
                    EventResult::Ignored
                }
            },
            _ => EventResult::Ignored
        }
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        if self.focused.is_none() {
            self.focused = Some(Vec2::new(0, 0));
        }

        true
    }
}

