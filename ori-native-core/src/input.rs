use keyboard_types::{Key, Modifiers, NamedKey};
use ori::Action;

pub struct Input<T> {
    filter:  InputFilter,
    handler: InputHandler<T>,
}

#[allow(clippy::type_complexity)]
pub struct InputFilter {
    keys: Vec<Box<dyn Fn(&Key, Modifiers, bool) -> bool>>,
}

#[allow(clippy::type_complexity)]
pub struct InputHandler<T> {
    keys: Vec<Box<dyn FnMut(&mut T, Key, Modifiers, bool) -> Action>>,
}

impl<T> Default for Input<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Input<T> {
    pub fn new() -> Self {
        Self {
            filter:  InputFilter::new(),
            handler: InputHandler::new(),
        }
    }

    pub fn split(self) -> (InputFilter, InputHandler<T>) {
        (self.filter, self.handler)
    }

    pub fn add_key<A>(
        &mut self,
        key: impl MatchKey + 'static,
        mods: Modifiers,
        mut on_key: impl FnMut(&mut T) -> A + 'static,
    ) where
        A: Into<Action>,
    {
        self.add_any_key(
            move |k, m, p| key.matches(k) && m == mods && p,
            move |data, _, _, _| on_key(data),
        );
    }

    pub fn add_any_key<A>(
        &mut self,
        filter: impl Fn(&Key, Modifiers, bool) -> bool + 'static,
        mut handler: impl FnMut(&mut T, Key, Modifiers, bool) -> A + 'static,
    ) where
        A: Into<Action>,
    {
        self.filter.keys.push(Box::new(filter));
        self.handler.keys.push(Box::new(
            move |data, key, modifiers, pressed| handler(data, key, modifiers, pressed).into(),
        ));
    }
}

impl InputFilter {
    fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn filter(&self, key: Key, modifiers: Modifiers, pressed: bool) -> Option<InputMessage> {
        for (index, filter) in self.keys.iter().enumerate() {
            if filter(&key, modifiers, pressed) {
                return Some(InputMessage::Key {
                    key,
                    modifiers,
                    pressed,
                    index,
                });
            }
        }

        None
    }
}

impl<T> InputHandler<T> {
    fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn handle(&mut self, data: &mut T, message: InputMessage) -> Action {
        match message {
            InputMessage::Key {
                key,
                modifiers,
                pressed,
                index,
            } => (self.keys[index])(data, key, modifiers, pressed),
        }
    }
}

pub trait MatchKey {
    fn matches(&self, key: &Key) -> bool;
}

impl MatchKey for Key {
    fn matches(&self, key: &Key) -> bool {
        self == key
    }
}

impl MatchKey for NamedKey {
    fn matches(&self, key: &Key) -> bool {
        matches!(key, Key::Named(named) if named == self)
    }
}

impl MatchKey for &str {
    fn matches(&self, key: &Key) -> bool {
        matches!(key, Key::Character(c) if c == self)
    }
}

impl MatchKey for char {
    fn matches(&self, key: &Key) -> bool {
        if let Key::Character(c) = key
            && c.len() == self.len_utf8()
            && c.starts_with(*self)
        {
            true
        } else {
            false
        }
    }
}

pub enum InputMessage {
    Key {
        key:       Key,
        modifiers: Modifiers,
        pressed:   bool,
        index:     usize,
    },
}
