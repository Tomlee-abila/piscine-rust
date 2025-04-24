use chrono::Duration;
use colored::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(content) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: content.to_string(),
            },
            Event::Registration(duration) => {
                let secs = duration.num_seconds();
                let hours = secs / 3600;
                let minutes = (secs % 3600) / 60;
                let seconds = secs % 60;
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    ),
                }
            }
            Event::Appointment(content) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: content.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

// Wrapper struct to allow custom implementation of Colorize
pub struct MyStr<'a>(pub &'a str);

impl<'a> MyStr<'a> {
    pub fn new(s: &'a str) -> Self {
        MyStr(s)
    }
}

impl<'a> fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            MyStr::new(&self.content).truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

impl<'a> Colorize for MyStr<'a> {
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        self.0.color(color)
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        self.0.on_color(color)
    }
    fn clear(self) -> ColoredString {
        self.0.clear()
    }
    fn normal(self) -> ColoredString {
        self.0.normal()
    }
    fn bold(self) -> ColoredString {
        self.0.bold()
    }
    fn dimmed(self) -> ColoredString {
        self.0.dimmed()
    }
    fn italic(self) -> ColoredString {
        self.0.italic()
    }
    fn underline(self) -> ColoredString {
        self.0.underline()
    }
    fn blink(self) -> ColoredString {
        self.0.blink()
    }
    fn reverse(self) -> ColoredString {
        self.0.reversed()
    }
    fn reversed(self) -> ColoredString {
        self.0.reversed()
    }
    fn hidden(self) -> ColoredString {
        self.0.hidden()
    }
    fn strikethrough(self) -> ColoredString {
        self.0.strikethrough()
    }
    fn truecolor(self, r: u8, g: u8, b: u8) -> ColoredString {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self.0).into()
    }
}
