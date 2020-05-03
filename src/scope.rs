use crate::instruction::*;
use crate::memory::*;
use std::collections::HashMap;

pub struct VariableDictionary {
    pub variables: HashMap<String, MemoryID>,
}

impl VariableDictionary {
    pub fn deref_variable(&self, ident: &String) -> Option<MemoryID> {
        self.variables.get(ident).map(|id| id.clone())
    }
}

pub enum ScopeStatus {
    Working,
    Done
}

pub struct Scope {
    pub variable_dictionary: VariableDictionary,

    pub current_instruction: usize,

    pub instructions: Vec<Box<dyn Execute>>,
}

impl Scope {
    pub fn step(&mut self, memory: &mut Memory) -> ScopeStatus {
        self.instructions[self.current_instruction].execute(memory, &mut self.variable_dictionary);
        self.current_instruction += 1;

        if self.current_instruction >= self.instructions.len() {
            ScopeStatus::Done
        } else {
            ScopeStatus::Working
        }
    }    
}