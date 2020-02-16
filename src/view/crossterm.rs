use crate::roster::Roster;
use crate::view::view_state::ViewState;

use std::io::{ stdout, Write };
use crossterm::{
    execute, queue,
    style::{ Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor },
    ExecutableCommand, Result,
    cursor::{ SavePosition, RestorePosition, position }
};
use nalgebra::Vector2;

impl ViewState {
    pub fn draw_ui(&self) {
        if let Ok(_) = self.inner_draw_ui() {
            // Nice
        } else {
            // Oops
        }
    }

    fn inner_draw_ui(&self) -> Result<()> {
        let mut grid = vec![];

        let mut headers = self.job_col_names();
        headers.insert(0, "".to_string());
        grid.push(headers);

        for week in 0..self.roster.num_weeks {
            let mut names = self.get_fields_for_week(week);
            names.insert(0, Some(format!("Week {}", week)));

            let names = names.into_iter()
                .map(|opt_name| if let Some(name) = opt_name { name } else { "".to_string() })
                .collect();

            grid.push(names);
        }

        // TODO: Refactor this to taking a Vec<Vec<StyledContent>>, since then we don't need to do
        // the styling magic, which will be a cleaner pass. The styling can still be inserted afterwards anyway.
        ViewState::print_grid(&grid, "|", vec![
            (Vector2::new(1, 1), Color::DarkBlue),
            (Vector2::new(3, 3), Color::DarkRed),
            (Vector2::new(1, 4), Color::Black)
        ])?;

        Ok(())
    }

    fn get_color_of<T: PartialEq>(elems: &[(T, Color)], elem: &T) -> Option<Color> {
        elems.iter()
            .find_map(|(other_elem, color)| if elem == other_elem { Some(*color) } else { None })
    }

    fn print_separated_by(elems: &Vec<String>, sep: &str, col_width: u32, highlighted: &Vec<(u32, Color)>) -> Result<()> {
        assert!(elems.len() > 0);

        for (i, elem) in elems.iter().enumerate() {
            queue!(
                stdout(),
                Print("|"),
            )?;

            let highlight_color = ViewState::get_color_of(highlighted, &(i as u32));
            // Emit highlighted message
            if let Some(highlight_color) = highlight_color {
                queue!(
                    stdout(),
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(highlight_color),
                )?;
            }

            queue!(
                stdout(),
                Print(" "),
                Print(&elem),
            )?;

            if elem.len() < col_width as usize + 1 {
                // Emit highlighted message
                for i in 0..(col_width + 1).saturating_sub(elem.len() as u32) {
                    queue!(
                        stdout(),
                        Print(" ")
                    )?;
                }
            }

            if highlight_color.is_some() {
                queue!(
                    stdout(),
                    ResetColor
                )?;
            }
        }

        queue!(
            stdout(),
            Print("|"),
        )?;

        Ok(())
    }

    fn print_grid(elems: &Vec<Vec<String>>, sep: &str, highlighted: Vec<(Vector2<u32>, Color)>) -> Result<()> {
        let max_col_width = elems.iter().map(|row| {
            row.iter().map(|x| x.len()).max().unwrap() as u32
        }).max().unwrap();

        for (i, row) in elems.iter().enumerate() {
            let highlighted = highlighted.iter()
                .filter_map(|pos| if pos.0.y == (i as u32) { Some((pos.0.x, pos.1)) } else { None })
                .collect();

            ViewState::print_separated_by(row, sep, max_col_width, &highlighted)?;

            queue!(
                stdout(),
                Print("\n"),
            )?;
        }

        Ok(())
    }
}
