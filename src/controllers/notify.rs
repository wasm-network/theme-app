/// This events system follows the Observer design pattern described here:
/// https://blog.rom1v.com/2017/09/gnirehtet-rewritten-in-rust/#observer
/// In this code, the Storage struct is called EventQueue

use std::cell::RefCell;
use std::rc::{Rc, Weak};


pub trait EventListener {
    fn on_event(&mut self, event: Event);
}

impl<F: FnMut(Event)> EventListener for F {
    fn on_event(&mut self, event: Event) {
        self(event);
    }
}

pub struct Notifier {
    listeners: Vec<Box<dyn EventListener>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    pub fn register<T: EventListener + 'static>(&mut self, listener: T) {
        self.listeners.push(Box::new(listener));
    }

    pub fn notify(&mut self, event: Event) {
        for listener in &mut self.listeners {
            listener.on_event(event);
        }
    }
}

// *****************************************************************************************************
// Model objects for passing around event info
// *****************************************************************************************************

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Button(u32),
    Selected(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub action: Action,
}

impl Event {
    pub fn new(action: Action) -> Self {
        Event { action }
    }
}

// Unused. It doesn't work because the delegate field in EventQueue is not holding the
// reference without ownership issues.
pub trait EventDelegate {
    fn handle_event(&mut self, event: Event);
}

pub struct EventQueue {
    weak_self: Weak<RefCell<EventQueue>>,
    delegate: Weak<Rc<RefCell<dyn EventDelegate>>>, // Does not work
    handlers: Vec<Rc<RefCell<dyn EventDelegate>>>,      // Does not work
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Rc<RefCell<Self>> {
        let rc = Rc::new(RefCell::new(Self {
            weak_self: Weak::new(), // initialize empty
            delegate: Weak::new(),
            handlers: Vec::new(),
            events: Vec::new(),
        }));
        rc.borrow_mut().weak_self = Rc::downgrade(&rc);
        rc
    }

    pub fn register_to(&self, notifier: &mut Notifier) {
        let rc = self.weak_self.upgrade().unwrap();
        notifier.register(move |event| {
            eprintln!("register event={:?}", event);
            rc.borrow_mut().store(event);
        })
    }

    pub fn store(&mut self, evt: Event) {
        self.events.push(evt);
        eprintln!("events count={:?}", self.events.len());
        if let Some(delegate) = self.delegate.upgrade() {
            delegate.borrow_mut().handle_event(evt.clone());
        }
        for handler in &mut self.handlers {
            handler.borrow_mut().handle_event(evt.clone());
        }
    }

    pub fn queue(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    // Unused and doesn't work
    pub fn set_delegate(&mut self, delegate: Weak<RefCell<dyn EventDelegate>>) {
        if let Some(rc) = delegate.upgrade() {
            let weak_delegate = Rc::downgrade(&Rc::new(rc));
            self.delegate = weak_delegate;
        }
    }

    // Unused and doesn't work
    pub fn add_handler(&mut self, handler: Rc<RefCell<dyn EventDelegate>>) {
        self.handlers.push(handler);
    }
}

