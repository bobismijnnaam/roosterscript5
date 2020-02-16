mod roster;
mod roster_view;
mod view;

use crate::roster::{Roster, Job};
use crate::view::view_state::ViewState;

use nalgebra::Vector2;

//use cursive::{Cursive, Vec2, Printer, XY, View};
//use cursive::views::{Dialog, TextView, SelectView, LinearLayout, Button, Panel, ListView};
//use std::cmp::{Ordering, max};
//use cursive::align::HAlign;
//use cursive::traits::{Nameable, Resizable};
//use roster::Roster;
//use crate::roster::{Job, Slot};
//use crate::roster_view::RosterView;
//use cursive::theme::ColorStyle;
//use std::thread::current;
//use std::fmt::format;
//use cursive::direction::Direction;
//use cursive::event::{EventResult, Event, Key};
//use num::clamp;
//
//use std::io::{stdout, Write};

//use crossterm::{
//    execute,
//    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
//    ExecutableCommand, Result,
//};
//use crate::view::print_roster;
//
//fn create_instructions() -> impl View {
//    Panel::new(
//        TextView::new("tab to switch the person currently active in the person brush, arrow keys to move, enter to place, backspace to delete, u to undo")
//    ).title("Instructions")
//}
//
//fn create_bottom_display() -> LinearLayout {
//    LinearLayout::horizontal()
//        .child(Panel::new(ListView::new()
//                .child("Binky", TextView::new("5"))
//                .child("Steve", TextView::new("10"))
//                .fixed_width(20)
//            )
//            .title("Person brush")
//        )
//        .child(Panel::new(ListView::new()
//                .child("Binky", TextView::new("5"))
//                .child("Steve", TextView::new("10"))
//                .fixed_width(20)
//            )
//            .title("Best fit")
//        )
//        .child(Panel::new(ListView::new()
//            .child("Binky", TextView::new("5"))
//            .child("Steve", TextView::new("10"))
//            .fixed_width(20)
//        )
//            .title("Current #jobs")
//        )
//}

fn main() {
    let roster = Roster::new(vec![
        Job::new(0, 3, 1),
        Job::new(1, 2, 3)
    ], 10);

    let view_state = ViewState {
        roster,
        job_names: vec!["Kitchen".to_owned(), "Hallway".to_owned()],
        person_names: vec!["Binky".to_owned(), "Steve".to_owned()],
        focused_field: Vector2::new(0, 0),
    };

    view_state.draw_ui();
}
