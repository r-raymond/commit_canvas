use crate::types::Guid;

pub struct GuidGenerator {
    counter: Guid,
}

impl GuidGenerator {
    pub fn new() -> GuidGenerator {
        GuidGenerator { counter: 0 }
    }

    pub fn next(&mut self) -> Guid {
        let guid = self.counter;
        self.counter += 1;
        guid
    }
}
