pub mod counter;
pub mod CounterUpdater;
mod Interface;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_shared_counter() {
        use crate::counter::SharedCounter;
        let path = "/tmp/test_shared_map.redb";
        let _ = std::fs::remove_file(path); // Cleanup before
        
        let map = SharedCounter::new(path).expect("Failed to create map");
        map.set("test_key", 42).expect("Failed to set");
        assert_eq!(map.get("test_key").unwrap(), Some(42));
        assert_eq!(map.increment("test_key").unwrap(), 43);
        
        let _ = std::fs::remove_file(path); // Cleanup after
    }
}
*/