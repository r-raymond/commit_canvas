pub struct GuidGenerator {
    counter: i32,
}

impl GuidGenerator {
    pub fn new() -> GuidGenerator {
        GuidGenerator { counter: 0 }
    }

    pub fn next(&mut self) -> i32 {
        let guid = self.counter;
        self.counter += 1;
        guid
    }
}
