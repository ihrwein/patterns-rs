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
    fn remove_handler(&mut self, handler: &EventHandler<Self::Event, Handler=Self::Handler>);
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

impl ConcreteReactor {
    fn new(selector: Box<EventDemultiplexer<Event=i32>>) -> ConcreteReactor {
        ConcreteReactor {
            selector: selector,
            map: BTreeMap::new()
        }
    }
}

impl Reactor for ConcreteReactor {
    type Event = i32;
    type Handler = i32;
    fn handle_events(&mut self) {
        loop {
            let event = self.selector.select();

            if let Some(handler) = self.map.get_mut(&event.handler()) {
                handler.handle_event(event);
            }
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=Self::Handler>>) {
        self.map.insert(handler.handler(), handler);
    }
    fn remove_handler(&mut self, handler: &EventHandler<Self::Event, Handler=Self::Handler>) {
        self.map.remove(&handler.handler());
    }
}

struct Demultiplexer(i32);

impl EventDemultiplexer for Demultiplexer {
    type Event = i32;
    fn select(&mut self) -> Self::Event {
        thread::sleep_ms(500);
        self.0 += 1;
        self.0 % 5
    }
}

struct A(i32);

impl EventHandler<i32> for A {
    type Handler = i32;
    fn handle_event(&mut self, event: i32) {
        println!("handled i32 event: {}", event);
    }
    fn handler(&self) -> Self::Handler {
        self.0
    }
}

fn register_handlers(reactor: &mut Reactor<Event=i32, Handler=i32>) {
    reactor.register_handler(Box::new(A(0)));
    reactor.register_handler(Box::new(A(1)));
    reactor.register_handler(Box::new(A(2)));
    reactor.register_handler(Box::new(A(3)));
    reactor.register_handler(Box::new(A(4)));
}

fn main() {
    let dem = Demultiplexer(0);
    let mut reactor = ConcreteReactor::new(Box::new(dem));
    register_handlers(&mut reactor);
    reactor.handle_events();
}
