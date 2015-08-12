use std::thread;
use std::collections::BTreeMap;

trait EventHandler<T> {
    type Handler;
    fn handle_event(&mut self, event: T);
    fn handler(&self) -> Self::Handler;
}

trait EventDemultiplexer {
    type Event;
    fn select(&mut self) -> Self::Event;
}

trait Reactor {
    type Event: Event;
    type Handler;
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=Self::Handler>>);
    fn remove_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=Self::Handler>>);
}

trait Event {
    type Handler;
    fn handler(&self) -> Self::Handler;
}

impl Event for i32 {
    type Handler = i32;
    fn handler(&self) -> Self::Handler {
        *self
    }
}

struct ConcreteReactor {
    selector: Box<EventDemultiplexer<Event=i32>>,
    map: BTreeMap<i32, Box<EventHandler<i32, Handler=i32>>>,
}

impl Reactor for ConcreteReactor {
    type Event = i32;
    type Handler = i32;
    fn handle_events(&mut self) {
        let event = self.selector.select();

        if let Some(handler) = self.map.get_mut(&event.handler()) {
            println!("Event handled");
            handler.handle_event(event);
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=Self::Handler>>) {
        self.map.insert(handler.handler(), handler);
    }
    fn remove_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=Self::Handler>>) {
        self.map.remove(&handler.handler());
    }
}

struct Demultiplexer(i32);

impl EventDemultiplexer for Demultiplexer {
    type Event = i32;
    fn select(&mut self) -> Self::Event {
        thread::sleep_ms(500);
        self.0 += 1;
        self.0
    }
}

struct A {
    i: i32,
    f: f32
}

impl EventHandler<i32> for A {
    type Handler = i32;
    fn handle_event(&mut self, event: i32) {
        println!("handled i32 event");
    }
    fn handler(&self) -> Self::Handler {
        self.i
    }
}

impl EventHandler<f32> for A {
    type Handler = f32;
    fn handle_event(&mut self, event: f32) {
        println!("handled f32 event");
    }
    fn handler(&self) -> Self::Handler {
        self.f
    }
}

fn main() {
    let mut a = A{i:1, f:3.14};
    a.handle_event(2);
    a.handle_event(2.0);
    let mut b = Box::new(a);
    b.handle_event(2);
    b.handle_event(2.0);
    println!("Hello, world!");
}
