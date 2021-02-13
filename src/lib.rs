mod ring_buffer {
    /// FIFO Ring buffer with fixed capacity.
    /// If it already contains n = capacity elements
    /// new element will override the oldest element.
    #[derive(Clone)]
    pub struct RingBuffer<T> {
        data: Vec<Option<T>>,
        capacity: usize,
        size: usize,
        start: usize,
        end: usize,
    }

    impl<T> RingBuffer<T> {
        pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
            RingBuffer {
                data: Vec::with_capacity(capacity),
                capacity,
                size: 0,
                start: 0,
                end: 0,
            }
        }

        pub fn push(&mut self, element: T) {
            if self.capacity == 0 {
                panic!("Can't push element to ring_buffer with zero capacity");
            }
            if self.size < self.capacity {
                self.size += 1;
            }
            if self.data.len() < self.capacity {
                self.data.push(Some(element));
                self.next_end();
            } else {
                self.data[self.end] = Some(element);
                if self.end == self.start {
                    self.next_start();
                }
                self.next_end();
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.size == 0 {
                None
            } else {
                self.size -= 1;
                let position: usize = self.start;
                self.next_start();
                self.data[position].take()
            }
        }

        fn next_start(&mut self) {
            self.start = (self.start + 1) % self.capacity;
        }

        fn next_end(&mut self) {
            self.end = (self.end + 1) % self.capacity;
        }
    }

    impl<T> IntoIterator for RingBuffer<T> {
        type Item = T;
        type IntoIter = ConsumingRingBufferIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            ConsumingRingBufferIterator {
                ring: self,
            }
        }
    }

    pub struct ConsumingRingBufferIterator<T> {
        ring: RingBuffer<T>,
    }

    impl<T> Iterator for ConsumingRingBufferIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.ring.pop()
        }
    }

    impl<'a, T> IntoIterator for &'a RingBuffer<T> {
        type Item = &'a T;
        type IntoIter = RingBufferIterator<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            RingBufferIterator {
                ring: self,
                size: self.size,
                position: self.start,
            }
        }
    }

    pub struct RingBufferIterator<'a, T> {
        ring: &'a RingBuffer<T>,
        size: usize,
        position: usize,
    }

    impl <'a, T> Iterator for RingBufferIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.size == 0 {
                None
            } else {
                self.size -= 1;
                let ret: &T = &(self.ring.data[self.position].as_ref().unwrap());
                self.position = (self.position + 1) % self.ring.capacity;
                Some(ret)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn test_push() {
        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(3);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(vec![&1, &2], (&buffer).into_iter().collect::<Vec<&i32>>());
        assert_eq!(vec![1, 2], buffer.into_iter().collect::<Vec<i32>>());

        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);
        assert_eq!(vec![&2, &3, &4], (&buffer).into_iter().collect::<Vec<&i32>>());
        assert_eq!(vec![2, 3, 4], buffer.into_iter().collect::<Vec<i32>>());

        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(vec![&1, &2, &3], (&buffer).into_iter().collect::<Vec<&i32>>());
        buffer.push(4);
        assert_eq!(vec![&2, &3, &4], (&buffer).into_iter().collect::<Vec<&i32>>());

        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(1);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(vec![&3], (&buffer).into_iter().collect::<Vec<&i32>>());

        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(2);
        for i in 0..100 {
            buffer.push(i);
        }
        assert_eq!(vec![&98, &99], (&buffer).into_iter().collect::<Vec<&i32>>());
    }

    #[test]
    fn test_pop() {
        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(5);
        assert_eq!(None, buffer.pop());
        for i in 1..6 {
            buffer.push(i);
        }
        for i in 1..6 {
            assert_eq!(i, buffer.pop().unwrap());
        }
        assert_eq!(None, buffer.pop());

        for i in 1..101 {
            buffer.push(i);
        }
        for i in 96..101 {
            assert_eq!(i, buffer.pop().unwrap());
        }
        assert_eq!(None, buffer.pop());

        for i in 1..101 {
            buffer.push(i);
        }
        buffer.pop();
        buffer.pop();
        assert_eq!(vec![&98, &99, &100], (&buffer).into_iter().collect::<Vec<&i32>>());
        for i in 98..101 {
            assert_eq!(i, buffer.pop().unwrap());
        }
        assert_eq!(None, buffer.pop());
    }

    #[test]
    fn test_iter() {
        let mut buffer: RingBuffer<i32> = RingBuffer::with_capacity(5);
        for i in 1..101 {
            buffer.push(i);
        }

        let vec1: Vec<&i32> = (&buffer).into_iter().collect();
        let vec2: Vec<&i32> = (&buffer).into_iter().collect();
        assert_eq!(vec1, vec2);

        for i in 96..101 {
            assert_eq!(i, buffer.pop().unwrap());
        }
    }
}
