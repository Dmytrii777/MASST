struct MyCircularQueue {
    data: Vec<i32>,
    front: usize,
    rear: usize,
    capacity: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let cap = k as usize + 1;
        Self {
            data: vec![0; cap],
            front: 0,
            rear: 0,
            capacity: cap,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() { return false; }
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() { return false; }
        self.front = (self.front + 1) % self.capacity;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() { -1 } else { self.data[self.front] }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.rear + self.capacity - 1) % self.capacity]
        }
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }
}
