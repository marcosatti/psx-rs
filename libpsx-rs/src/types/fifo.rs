use crossbeam::queue::ArrayQueue;

pub(crate) struct Fifo<T> {
    fifo: ArrayQueue<T>,
}

impl<T> Fifo<T> {
    pub(crate) fn new(capacity: usize) -> Fifo<T> {
        Fifo {
            fifo: ArrayQueue::new(capacity),
        }
    }

    pub(crate) fn read_one(&self) -> Result<T, ()> {
        self.fifo.pop().map_err(|_| ())
    }

    pub(crate) fn write_one(&self, data: T) -> Result<(), ()> {
        self.fifo.push(data).map_err(|_| ())
    }

    pub(crate) fn read_available(&self) -> usize {
        self.fifo.len()
    }

    pub(crate) fn write_available(&self) -> usize {
        self.fifo.capacity() - self.fifo.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.fifo.is_empty()
    }

    pub(crate) fn is_full(&self) -> bool {
        self.fifo.is_full()
    }

    pub(crate) fn clear(&self) {
        while let Ok(_) = self.fifo.pop() {}
        assert!(self.fifo.is_empty())
    }
}

impl<T> Clone for Fifo<T>
where 
    T: Clone 
{
    fn clone(&self) -> Self {
        let capacity = self.fifo.capacity();

        let mut buffer = Vec::with_capacity(capacity);
        while let Ok(item) = self.fifo.pop() {
            buffer.push(item);
        }

        let fifo = Fifo::new(capacity);
        for item in buffer.drain(..) {
            fifo.fifo.push(item).unwrap();
        }

        fifo
    }
}

#[cfg(feature = "serialization")]
mod serialization {
    use super::*;
    use serde::{
        Deserialize,
        Deserializer,
        Serialize,
        Serializer,
    };

    #[derive(Serialize, Deserialize)]
    struct State<T> {
        capacity: usize,
        buffer: Vec<T>,
    }

    impl<T> Serialize for Fifo<T> 
    where 
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where 
            S: Serializer,
        {
            let capacity = self.fifo.capacity();
            let mut buffer = Vec::with_capacity(self.fifo.len());
            
            while let Ok(item) = self.fifo.pop() {
                buffer.push(item);
            }

            let state = State {
                capacity,
                buffer,
            };

            <State<T> as Serialize>::serialize(&state, serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Fifo<T>    
    where 
        T: Deserialize<'de>, 
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where 
            D: Deserializer<'de> 
        {
            let mut state = <State<T> as Deserialize>::deserialize(deserializer)?;

            let fifo = Fifo::new(state.capacity);

            for item in state.buffer.drain(..) {
                fifo.write_one(item).unwrap();
            }
            
            Ok(fifo)
        }
    } 
}
