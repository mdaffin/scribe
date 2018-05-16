use std::fmt::Display;
use termion::event::Key;
use termion::input::TermRead;
use termion::{self, color, raw::IntoRawMode};
use std::io::{stdin, stdout, Write};

pub fn select_from<T>(items: &[T]) -> Option<&T>
where
    T: Display,
{
    let menu = Menu { items, current: 0 };
    menu.select()
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

        self.print(&mut stdout);
        write!(stdout, "{}", termion::cursor::Hide,).unwrap();
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
        write!(w, "{}\r", termion::cursor::Up(self.items.len() as u16 - 1)).unwrap();
        self.print(w);
    }

    fn print(&self, w: &mut impl Write) {
        for (i, item) in self.items.iter().enumerate() {
            let new_line = if i == self.items.len() - 1 {
                ""
            } else {
                "\n\r"
            };
            let padding = if i == self.current { "> " } else { "  " };
            write!(
                w,
                "{}{}{}{}",
                termion::clear::CurrentLine,
                padding,
                item,
                new_line,
            ).unwrap();
        }
    }
}
