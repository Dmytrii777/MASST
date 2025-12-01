use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.q2.push_back(x);

        while let Some(v) = self.q1.pop_front() {
            self.q2.push_back(v);
        }

        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q1.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}
