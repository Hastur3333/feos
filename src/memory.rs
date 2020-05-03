//! The memory is a HashMap indexed by MemoryIDs, containing the Values.
//! The reason for using a HashMap in stead of an array or Vec, is that 
//! we can just continue to increase the ID and not worry about taking up unnessesarry memory.
//! Every memory entry is reference counted, that is so we do not ever have to deal with lifetimes,
//! or obsolete references.

use std::collections::HashMap;

/// The type of MemoryID.
pub type MemoryID = u64;

/// All of the types of values storable.
#[derive(Clone)]
pub enum Value {
    Reference(MemoryID),
    I32(i32),
    F32(f32),
}

/// The memory entry contains a Value and a reference count.
pub struct MemoryEntry {
    pub value: Value,
    pub reference_count: u32,
}

impl MemoryEntry {
    /// Creates a new memory entry
    pub fn new(value: Value) -> Self {
        Self {
            value,
            reference_count: 0,
        }
    }
}

/// The Memory struct is responsible for holding all of the data, insertions and retrievals.
pub struct Memory {
    pub memory: HashMap<MemoryID, MemoryEntry>,
    pub current_memory_id: MemoryID,
}

impl Memory {
    /// Creates a new Memory.
    pub fn new() -> Memory {
        Memory {
            memory: HashMap::new(),
            current_memory_id: 0,
        }
    }

    /// Inserts a value into memory and returns a MemoryID pointing to the value.
    pub fn allocate(&mut self, value: Value) -> MemoryID {
        let id = self.current_memory_id;

        self.memory.insert(id, MemoryEntry::new(value));
        self.current_memory_id += 1;

        id
    }

    /// Gets a Value from Memory at the given MemoryID.
    pub fn read(&self, id: &MemoryID) -> Option<&Value> {
        self.memory.get(id).map(|entry| &entry.value)
    }

    /// Gets a mutable reference to a Value from Memory at the given MemoryID.
    pub fn read_mut(&mut self, id: &MemoryID) -> Option<&mut Value> {
        self.memory.get_mut(id).map(|entry| &mut entry.value)
    }

    /// Frees a MemoryEntry from memory
    pub fn free(&mut self, id: &MemoryID) {
        let entry = self.memory.get_mut(id);

        if entry.is_none() {
            return;
        }

        let entry = entry.unwrap();

        // if the reference count of the entry is zero, we free it from memory and return
        if entry.reference_count == 0 {
            self.memory.remove(id);

            return;
        }

        entry.reference_count -= 1;
    }
}
