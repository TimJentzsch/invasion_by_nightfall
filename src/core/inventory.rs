use bevy::prelude::*;

use std::fmt::Display;

#[derive(Debug, Resource)]
pub struct Inventory {
    pub coins: Item,
}

#[derive(Debug)]
pub struct Item {
    count: f32,
    capacity: f32,
}

impl Item {
    pub fn empty(capacity: u32) -> Self {
        Self {
            count: 0.,
            capacity: capacity as f32,
        }
    }

    #[cfg(test)]
    pub fn new(count: u32, capacity: u32) -> Self {
        Self {
            count: count as f32,
            capacity: capacity as f32,
        }
    }

    pub fn count(&self) -> u32 {
        self.count.max(0.).floor() as u32
    }

    pub fn capacity(&self) -> u32 {
        self.capacity.max(0.).floor() as u32
    }

    /// Add until the capacity is reached, return the rest.
    pub fn add_until_full(&mut self, amount: f32) -> f32 {
        let total = self.count + amount;

        if total >= self.capacity {
            self.count = self.capacity;
            total - self.capacity
        } else {
            self.count = total;
            0.
        }
    }

    /// Try to remove the given amount.
    ///
    /// If not enough item is available, `false` is returned and nothing changes.
    pub fn try_remove(&mut self, amount: u32) -> bool {
        if self.count >= amount as f32 {
            self.count -= amount as f32;
            true
        } else {
            false
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.count(), self.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_until_full_not_reaching_capacity() {
        let mut item = Item::new(2, 10);

        let not_added = item.add_until_full(5.);

        assert_eq!(item.count(), 7);
        assert_eq!(not_added as u32, 0);
    }

    #[test]
    fn add_until_full_reaching_capacity() {
        let mut item = Item::new(5, 10);

        let not_added = item.add_until_full(10.);

        assert_eq!(item.count(), 10);
        assert_eq!(not_added as u32, 5);
    }

    #[test]
    fn try_remove_successful() {
        let mut item = Item::new(8, 10);

        let is_success = item.try_remove(5);

        assert_eq!(item.count(), 3);
        assert!(is_success);
    }

    #[test]
    fn try_remove_failure() {
        let mut item = Item::new(3, 10);

        let is_success = item.try_remove(5);

        assert_eq!(item.count(), 3);
        assert!(!is_success);
    }
}
