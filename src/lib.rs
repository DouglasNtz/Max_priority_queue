#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_no_draw;

#[cfg(test)]
mod tests_smart;
mod priority_queue;

mod priority_queue_no_draw;

mod smart_priority_queue;

pub use smart_priority_queue::{SmartPriorityQueue, Element, PriorityUsize};