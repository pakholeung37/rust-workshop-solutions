use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_BUCKET_COUNT: usize = 16;
const MAX_LOAD_FACTOR: f64 = 0.9;

type Slot<K, V> = Option<((K, V), usize)>;
pub struct HashMap<K: Hash + Eq, V> {
    slots: Vec<Slot<K, V>>,
    slot_count: usize,
    item_count: usize,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        HashMap {
            slots: Self::create_slots(INITIAL_BUCKET_COUNT),
            slot_count: INITIAL_BUCKET_COUNT,
            item_count: 0,
        }
    }
    // pub fn insert(&mut self, key: K, value: V) -> Option<&V> {
    //     Some(())
    // }

    pub fn get(&self, key: &K) -> Option<&V> {
        let slot_index = self.slot_index(&key);
        let slot = self.slot(slot_index, key)?;
        let ((_, v), _) = slot.as_ref()?;
        Some(v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let slot_index = self.slot_index(&key);
        let slot = self.slot_mut(slot_index, key)?;
        let ((_, v), _) = slot.take()?;
        Some(v)
    }

    fn slot(&self, slot_index: usize, key: &K) -> Option<&Slot<K, V>> {
        self.slots.iter().skip(slot_index).find(|item| match item {
            Some(((k, _), _)) => k == key,
            None => true,
        })
    }
    fn slot_mut(&mut self, slot_index: usize, key: &K) -> Option<&mut Slot<K, V>> {
        self.slots
            .iter_mut()
            .skip(slot_index)
            .find(|item| match item {
                Some(((k, _), _)) => k == key,
                None => true,
            })
    }
    fn slot_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.slot_count as u64) as usize
    }
    fn create_slots(slot_count: usize) -> Vec<Slot<K, V>> {
        let mut new_slots = Vec::with_capacity(slot_count);
        for _ in 0..slot_count {
            new_slots.push(None);
        }
        new_slots
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn map_works() {
        let mut map = HashMap::new();
        // assert_eq!(map.insert("foo", "bar"), None);
        // assert_eq!(map.insert("foo", "lol"), Some("bar"));

        assert_eq!(map.get(&"foo"), Some(&"lol"));
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        assert_eq!(map.get(&"qux"), None);

        // assert_eq!(map.remove(&"foo"), Some("lol"));
        assert_eq!(map.get(&"foo"), None);
    }
}
