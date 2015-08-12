pub trait Observer<T> {
    fn update(&mut self, data: T);
}

pub trait Subject<T, E> where T: Observer<E> {
    fn register_observer(&mut self, observer: T);
    fn remove_observer(&mut self, observer: T);
    fn notify_observers(&mut self);
}
