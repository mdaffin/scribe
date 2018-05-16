use std::fmt::Display;
use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::{self, color, raw::IntoRawMode};

pub fn select_from<T>(items: &[T]) -> Option<&T>
where
    T: Display,
{
    match items.len() {
        0 => {
            println!("No sutible devices found");
            None
        }
        _ => {
            let menu = Menu { items, current: 0 };
            menu.select()
        }
    }
}

struct Menu<'a, T>
where
    T: 'a,
{
    items: &'a [T],
    current: usize,
}

impl<'a, T> Menu<'a, T>
where
    T: Display,
{
    pub fn select(mut self) -> Option<&'a T> {
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = stdin();
        let stdin = stdin.lock();

        write!(
            stdout,
            "{}Select device to write image to ('q' or 'n' to cancel):\n\r",
            termion::cursor::Hide,
        ).unwrap();
        self.print(&mut stdout);
        stdout.flush().unwrap();

        let mut selected = None;

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Up => {
                    if self.current > 0 {
                        self.current -= 1;
                    }
                }
                Key::Down => {
                    if self.current < self.items.len() - 1 {
                        self.current += 1;
                    }
                }
                Key::Char('\n') => {
                    selected = Some(&self.items[self.current]);
                    break;
                }
                Key::Char('q') | Key::Ctrl('c') | Key::Char('n') | Key::Esc => break,
                _ => {}
            }
            self.reset_and_print(&mut stdout);
            stdout.flush().unwrap();
        }
        write!(
            stdout,
            "{}{}\n\r",
            termion::cursor::Show,
            termion::style::Reset
        ).unwrap();
        stdout.flush().unwrap();
        selected
    }

    fn reset_and_print(&self, w: &mut impl Write) {
        if self.items.len() > 1 {
            write!(w, "{}", termion::cursor::Up(self.items.len() as u16 - 1)).unwrap();
        }
        self.print(w);
    }

    fn print(&self, w: &mut impl Write) {
        for (i, item) in self.items.iter().enumerate() {
            write!(
                w,
                "\r{}{}{}{}",
                termion::clear::CurrentLine,
                if i == self.current { "> " } else { "  " },
                item,
                if i == self.items.len() - 1 { "" } else { "\n" },
            ).unwrap();
        }
    }
}
