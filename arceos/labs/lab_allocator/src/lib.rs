//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{BaseAllocator, ByteAllocator, AllocResult};
use core::ptr::NonNull;
use core::alloc::Layout;

// #[macro_use]
// extern crate log;

pub struct LabByteAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,

    free_blocks: [Option<(usize, usize)>; 2048],
    free_count: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            free_blocks: [None; 2048], 
            free_count: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        self.next = start;
        self.free_count = 0;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.heap_end = start + size;
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let size = layout.size();
        let adjusted_size = size + 1;



        for i in 0..self.free_count {
            if let Some((free_start, free_size)) = self.free_blocks[i] {

                if free_size == size {
                    // info!("Found a free block at {} with size {}", free_start, free_size);
                    
                    let ptr = NonNull::new(free_start as *mut u8).unwrap();

                    self.free_blocks[i] = None;
                    self.free_count -= 1;
                    for j in i..self.free_count {
                        self.free_blocks[j] = self.free_blocks[j + 1];
                    }
                    self.free_blocks[self.free_count] = None;

                   


                    return Ok(unsafe { NonNull::new_unchecked(ptr.as_ptr()) });
                }
            }
        
        }

        if self.next + size > self.heap_end {
            return Err(allocator::AllocError::NoMemory)
        }


        let ptr = NonNull::new(self.next as *mut u8).unwrap();
        self.next = self.next + adjusted_size;


        Ok(ptr)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {

        let size = layout.size();
        let adjusted_size = size + 1;
        let start = pos.as_ptr() as usize;

        // info!("Deallocating memory at {} with size {}", start, size);

        if self.free_count >= self.free_blocks.len() {
            return;
        }

        self.free_blocks[self.free_count] = Some((start, adjusted_size));
        self.free_count += 1;


    }

    fn total_bytes(&self) -> usize {
        self.heap_end - self.heap_start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.heap_start
    }

    fn available_bytes(&self) -> usize {
        self.heap_end - self.next
    }
}
