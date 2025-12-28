use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::atomic::Ordering::Acquire;

pub fn  logic_counter_updater(old_counter: u32) -> u32 {
        old_counter * 2
}

pub fn apply_counter_updating(old_counter: &AtomicU32, f: fn(u32) -> u32) -> u32 {
    let mut old = old_counter.load(Ordering::SeqCst);
    loop {
        let new = f(old);
        match old_counter.compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return new,
            Err(actual) => old = actual,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logic_double() {
        assert_eq!(logic_counter_updater(1), 2);
        assert_eq!(logic_counter_updater(2), 4);
        assert_eq!(logic_counter_updater(10), 20);
    }

    #[test]
    fn counter_updater() {
        let furkan = AtomicU32::new(4);

        apply_counter_updating(&furkan, logic_counter_updater);

        println!("{}", furkan.load(Ordering::SeqCst));
        assert_eq!(furkan.load(Ordering::SeqCst), 8);

    }
}