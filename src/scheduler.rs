use crate::event::Event;
pub trait Scheduler<Args, Output> {
    fn insert(&mut self, event : Event<Args, Output>);

    fn cancel(&mut self, event : Event<Args, Output>);

    fn is_empty(&self) -> bool;

    fn remove_next(&mut self) -> Output;

    fn remove(&mut self, event : Event<Args, Output>);

    fn peek_next(&self) -> Output;
}