use crate::memory::*;
use crate::scope::*;

pub trait Execute {
    fn execute(&self, memory: &mut Memory, variables: &mut VariableDictionary);
}

pub trait Deref {
    fn deref(&self, memory: &mut Memory, variables: &mut VariableDictionary) -> MemoryID;
}

pub struct Variable {
    pub ident: String
}

impl Deref for Variable {
    fn deref(&self, _memory: &mut Memory, variables: &mut VariableDictionary) -> MemoryID {
        // FIXME: find a more elegant solution than expect
        variables.deref_variable(&self.ident).expect("variable ident not known to scope")
    }
}

pub struct Assign {
    pub org: Box<dyn Deref>,
    pub trg: Box<dyn Deref>,
}

impl Execute for Assign {
    fn execute(&self, memory: &mut Memory, variables: &mut VariableDictionary) {
        let org_id = &self.org.deref(memory, variables);
        let trg_id = &self.trg.deref(memory, variables);

        let org = memory.read(org_id).map(|org| org.clone());

        org.map(|org|
            memory.read_mut(trg_id).map(|trg| *trg = org)
        );
    }
}