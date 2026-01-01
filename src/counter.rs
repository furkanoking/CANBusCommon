use shared_memory:: {Shmem, ShmemConf};

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};
use crate::CounterUpdater::{apply_counter_updating, logic_counter_updater};

#[repr(C)] // this is for the shared memory
pub struct CANBusSharedCounter {
    id: AtomicU32,
    counter: AtomicU32,
}


pub struct SharedCounter {
    shmem: Shmem, // this is for the shared memory
    capacity: usize,
    is_created: bool, // to be sure it is craeted or not
}

impl SharedCounter {
   
    pub fn new(capacity: usize) -> Self {

        let os_name = "CANBusSharedCounter";
        let mut is_created_local = false;
        let size = capacity * std::mem::size_of::<CANBusSharedCounter>();

        let mut is_creator = false;

        let my_shared_memory = match ShmemConf::new()
            .size(size)
            .os_id(os_name)
            .create() {
                Ok(m) => {
                        is_creator = true; // To understand if it is first time created or not
                        m }
                Err(e) => {
                        ShmemConf::new()
                            .os_id(os_name)
                            .open()
                            .expect("Shared memory could not be opened or created.")
                }
        };

        if(is_creator) {
            println!("Shared memory created.");
            is_created_local= true;
            let ptr = my_shared_memory.as_ptr() as *mut CANBusSharedCounter;
            let slots: &mut [CANBusSharedCounter] = unsafe { std::slice::from_raw_parts_mut(ptr, capacity) };

            for(i, slot) in slots.iter_mut().enumerate() {
                let id = (i as u32) + 1;
                slot.id.store(id, Ordering::SeqCst);
                slot.counter.store(id, Ordering::SeqCst);
            }

        } else {
            println!("Shared memory opened.");
        }

       // This is the return value.
        SharedCounter {
            shmem: my_shared_memory,
            capacity,
            is_created: is_created_local,
        }
    }

    pub fn incremet_the_counter(&self, target_id:u32) {
        let slots = unsafe {
            let ptr = self.shmem.as_ptr() as *const CANBusSharedCounter;
            std::slice::from_raw_parts(ptr, self.capacity)
        };

        for slot in slots {
                if slot.id.load(Ordering::SeqCst) == target_id { // the reason that we are using load is it is atomic value
                    apply_counter_updating(&slot.counter, logic_counter_updater);
                    println!("new counter is : {}",slot.counter.load(Ordering::SeqCst));
                    println! ("ID {} bulundu ve güncellendi.", target_id); // TODO it is a random number. We have to generatr eit from another function
                    return; // get out of the function
                }
        }

        println! ("ID {} bulunamadı.", target_id); 
    }

    pub fn show_the_counter(&self, target_id:u32) -> u32{
            let slots = unsafe {
            let ptr = self.shmem.as_ptr() as *const CANBusSharedCounter;
            std::slice::from_raw_parts(ptr, self.capacity)
        };
        
        for slot in slots {
                if slot.id.load(Ordering::SeqCst) == target_id {
                    println!(" counter is : {}",slot.counter.load(Ordering::SeqCst));

                    return slot.counter.load(Ordering::SeqCst);
                }
        }

            println! ("ID {} bulunamadı.", target_id);
            return 0;
    }
}


// The reason why we are using unsafe is the global static
// variable should be protected. We use mutex but because of the
// shared library, it is unsafe. By writing unsafe we assume it is protected
unsafe impl Send for SharedCounter {}
unsafe impl Sync for SharedCounter {}

pub static GLOBAL_COUNTER: LazyLock<Mutex<SharedCounter>> = LazyLock::new(|| {
   let capacity = 64;
    Mutex::new(SharedCounter::new(capacity))
});

