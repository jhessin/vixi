use std::io::Read;

use super::{KeyStroke, Keyboard};

use termion::event::Key;
use termion::input::{Keys, TermRead};

pub struct TermionKeyboard<R: Read> {
    key_reader: Keys<R>,
}

impl<R: Read> TermionKeyboard<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            key_reader: reader.keys(),
        }
    }
}

impl<R: Read> Keyboard for TermionKeyboard<R> {
    fn get_next_keystroke(&mut self) -> Option<KeyStroke> {
        let res = self.key_reader.next();
        if res.is_none() {
            return None;
        }

        match res.unwrap().unwrap() {
            Key::Backspace => Some(KeyStroke::KeyBackSpace),
            Key::Left => Some(KeyStroke::KeyLeft),
            Key::Right => Some(KeyStroke::KeyRight),
            Key::Up => Some(KeyStroke::KeyUp),
            Key::Down => Some(KeyStroke::KeyDown),
            Key::Home => None,
            Key::End => None,
            Key::PageUp => Some(KeyStroke::KeyNextPage),
            Key::PageDown => Some(KeyStroke::KeyPreviousPage),
            Key::Delete => Some(KeyStroke::KeyDeleteChar),
            Key::Insert => None,
            Key::F(n) => Some(KeyStroke::KeyF(n)),
            Key::Alt(c) => Some(KeyStroke::Alt(c)),
            Key::Ctrl(_) => None,
            Key::Null => None,
            Key::Esc => Some(KeyStroke::KeyEscape),
            Key::Char(c) => Some(KeyStroke::Char(c)),
            _ => None,
        }
    }
}
