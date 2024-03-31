use crate::model::EventHistory;

use crate::model::Shape;

pub enum Event<'a> {
    Reload { shapes: Vec<&'a Shape> },
    Modify { event: EventHistory },
}
