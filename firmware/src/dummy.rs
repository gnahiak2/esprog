// firmware/src/dummy.rs

use core::cell::UnsafeCell;

pub const MEMORY_SIZE: usize = 64 * 1024;

struct DummyMemory {
    data: UnsafeCell<[u8; MEMORY_SIZE]>,
}

unsafe impl Sync for DummyMemory {}

static MEMORY: DummyMemory = DummyMemory {
    data: UnsafeCell::new([0xFF; MEMORY_SIZE]),
};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    OutOfBounds,
}

// Return device information
pub fn probe() -> (&'static str, usize) {
    ("DUMMY", MEMORY_SIZE)
}

// Erase all memory
pub fn erase() {
    unsafe {
        let memory = &mut *MEMORY.data.get();

        memory.fill(0xFF);
    }
}

// Read bytes
pub fn read(address: usize, buffer: &mut [u8]) -> Result<(), Error> {
    let end = address
        .checked_add(buffer.len())
        .ok_or(Error::OutOfBounds)?;

    if end > MEMORY_SIZE {
        return Err(Error::OutOfBounds);
    }

    unsafe {
        let memory = &*MEMORY.data.get();

        buffer.copy_from_slice(&memory[address..end]);
    }

    Ok(())
}

// Write bytes
pub fn write(address: usize, data: &[u8]) -> Result<(), Error> {
    let end = address.checked_add(data.len()).ok_or(Error::OutOfBounds)?;

    if end > MEMORY_SIZE {
        return Err(Error::OutOfBounds);
    }

    unsafe {
        let memory = &mut *MEMORY.data.get();

        memory[address..end].copy_from_slice(data);
    }

    Ok(())
}

// Fill entire memory with a value
pub fn fill(value: u8) {
    unsafe {
        let memory = &mut *MEMORY.data.get();

        memory.fill(value);
    }
}
