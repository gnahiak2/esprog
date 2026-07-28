// firmware/src/dummy.rs

use core::cell::RefCell;
use critical_section::Mutex;

const MEMORY_SIZE: usize = 64 * 1024;

static MEMORY: Mutex<RefCell<[u8; MEMORY_SIZE]>> = Mutex::new(RefCell::new([0xFF; MEMORY_SIZE]));

pub fn probe() -> (&'static str, usize) {
    ("DUMMY", MEMORY_SIZE)
}

pub fn erase() {
    critical_section::with(|cs| {
        let mut memory = MEMORY.borrow_ref_mut(cs);
        memory.fill(0xFF);
    });
}

pub fn read(address: usize, buffer: &mut [u8]) -> Result<(), ()> {
    let end = address.checked_add(buffer.len()).ok_or(())?;

    if end > MEMORY_SIZE {
        return Err(());
    }

    critical_section::with(|cs| {
        let memory = MEMORY.borrow_ref(cs);
        buffer.copy_from_slice(&memory[address..end]);
    });

    Ok(())
}

pub fn write(address: usize, data: &[u8]) -> Result<(), ()> {
    let end = address.checked_add(data.len()).ok_or(())?;

    if end > MEMORY_SIZE {
        return Err(());
    }

    critical_section::with(|cs| {
        let mut memory = MEMORY.borrow_ref_mut(cs);
        memory[address..end].copy_from_slice(data);
    });

    Ok(())
}
