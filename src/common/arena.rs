use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle<T> {
    offset: u32,
    length: u32,
    generation: u32,
    _marker: PhantomData<T>,
}

pub struct Arena {
    buffer: Vec<u8>,
    generation: u32,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            generation: 0,
        }
    }

    pub fn reserve<T>(&mut self, length: usize) -> Handle<T> {
        let align = std::mem::align_of::<T>();
        let size = std::mem::size_of::<T>() * length;

        let current_length = self.buffer.len();
        let padding = (align - (current_length % align)) % align;
        let total_bytes_needed = padding + size;
        self.buffer.reserve(total_bytes_needed);

        // Benchmark this.
        // Zero-filling prevents garbage memory from spilling out, but might slow things down.
        self.buffer.resize(current_length + total_bytes_needed, 0);

        let offset = current_length + padding;

        Handle {
            offset: offset as u32,
            length: length as u32,
            generation: self.generation,
            _marker: PhantomData,
        }
    }

    pub fn get<T>(&self, handle: Handle<T>) -> &[T] {
        debug_assert_eq!(handle.generation, self.generation, "stale handle");

        let byte_offset = handle.offset as usize;
        let byte_len = handle.length as usize * std::mem::size_of::<T>();

        assert!(
            byte_offset + byte_len <= self.buffer.len(),
            "handle out of bounds"
        );

        unsafe {
            let ptr = self.buffer.as_ptr().add(byte_offset) as *const T;
            std::slice::from_raw_parts(ptr, handle.length as usize)
        }
    }

    pub fn get_mut<T>(&mut self, handle: Handle<T>) -> &mut [T] {
        debug_assert_eq!(handle.generation, self.generation, "stale handle");

        let byte_offset = handle.offset as usize;
        let byte_len = handle.length as usize * std::mem::size_of::<T>();

        assert!(
            byte_offset + byte_len <= self.buffer.len(),
            "handle out of bounds"
        );

        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(byte_offset) as *mut T;
            std::slice::from_raw_parts_mut(ptr, handle.length as usize)
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.generation = self.generation.wrapping_add(1);
    }
}
