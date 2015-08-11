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

trait Reactor<I, O> {
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<EventHandler<I, Handler=O>>);
    fn remove_handler(&mut self, handler: Box<EventHandler<I, Handler=O>>);
}

struct ConcreteReactor<I, O> {
    map: BTreeMap<O, Box<EventHandler<I, Handler=O>>>
}

impl<I, O: Ord> Reactor<I, O> for ConcreteReactor<I, O> {
    fn handle_events(&mut self) {
    }
    fn register_handler(&mut self, handler: Box<EventHandler<I, Handler=O>>) {
        self.map.insert(handler.handler(), handler);
    }
    fn remove_handler(&mut self, handler: Box<EventHandler<I, Handler=O>>) {
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

enum Event {
    A(i32),
    B(f32)
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
