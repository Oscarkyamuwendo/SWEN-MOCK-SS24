pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    start: usize,
    end: usize,
    capacity: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        CircularBuffer {
            data,
            start: 0,
            end: 0,
            capacity,
            size: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.size == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.data[self.end] = Some(element);
        self.end = (self.end + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            return Err(Error::EmptyBuffer);
        }
        let element = self.data[self.start].take().unwrap();
        self.start = (self.start + 1) % self.capacity;
        self.size -= 1;
        Ok(element)
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.data.resize_with(self.capacity, || None);
        self.start = 0;
        self.end = 0;
        self.size = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.size == self.capacity {
            self.data[self.start] = Some(element);
            self.start = (self.start + 1) % self.capacity;
        } else {
            self.data[self.end] = Some(element);
            self.end = (self.end + 1) % self.capacity;
            self.size += 1;
        }
    }
}
