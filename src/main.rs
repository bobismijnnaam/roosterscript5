mod roster;
mod roster_view;

use cursive::{Cursive, Vec2, Printer, XY, View};
use cursive::views::{Dialog, TextView, SelectView, LinearLayout, Button, Panel, ListView};
use std::cmp::{Ordering, max};
use cursive::align::HAlign;
use cursive::traits::{Nameable, Resizable};
use roster::Roster;
use crate::roster::{Job, Slot};
use crate::roster_view::RosterView;
use cursive::theme::ColorStyle;
use std::thread::current;
use std::fmt::format;
use cursive::direction::Direction;
use cursive::event::{EventResult, Event, Key};
use num::clamp;

fn create_instructions() -> impl View {
    Panel::new(
        TextView::new("+/- to switch the person currently active in the person brush, arrow keys to move, enter to place, backspace to delete, u to undo")
    ).title("Instructions")
}

fn create_bottom_display() -> LinearLayout {
    LinearLayout::horizontal()
        .child(Panel::new(ListView::new()
                .child("Binky", TextView::new("5"))
                .child("Steve", TextView::new("10"))
                .fixed_width(20)
            )
            .title("Person brush")
        )
        .child(Panel::new(ListView::new()
                .child("Binky", TextView::new("5"))
                .child("Steve", TextView::new("10"))
                .fixed_width(20)
            )
            .title("Best fit")
        )
        .child(Panel::new(ListView::new()
            .child("Binky", TextView::new("5"))
            .child("Steve", TextView::new("10"))
            .fixed_width(20)
        )
            .title("Current #jobs")
        )
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    let roster = Roster::new(vec![
        Job::new(0, 3, 1),
        Job::new(1, 2, 3)
    ], 10);

    let roster_view = RosterView::new(&roster, vec!["Kitchen".to_owned(), "Hallway".to_owned()], vec!["Binky".to_owned(), "Steve".to_owned()]);

    let options = SelectView::new()
        .item("AAA", "AAA")
        .item("BBB", "AAA");

    let schedule_layer = LinearLayout::vertical()
        .child(create_instructions())
        .child(Panel::new(roster_view)
            .title("Current schedule")
        )
        .child(create_bottom_display());

    siv.add_layer(schedule_layer);

    // Starts the event loop.
    siv.run();
}
