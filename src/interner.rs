use std::collections::HashMap;
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StrId(pub u32);

pub struct Interner {
    map: HashMap<&'static str, (String, StrId)>,
    id_to_str: HashMap<u32, &'static str>,
    idx: usize,
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            map: HashMap::new(),
            id_to_str: HashMap::new(),
            idx: 0,
        }
    }
    pub fn intern(&mut self, name: &str) -> StrId {
        if let Some((_, id)) = self.map.get(name) {
            return *id;
        }

        let string = String::from(name);
        let name = unsafe { self.alloc(&string) };
        let id = self.get_next_idx() as u32;
        self.map.insert(name, (string, StrId(id)));
        self.id_to_str.insert(id, name);
        StrId(id)
    }
    unsafe fn alloc(&self, name: &str) -> &'static str {
        &*(name as *const str)
    }
    pub fn lookup(&self, str_id: StrId) -> &str {
        let id = str_id.0;
        self.id_to_str
            .get(&id)
            .expect(format!("{} not a valid str, lookup string failed.", str_id.0).as_str())
    }
    pub fn free(&mut self) {
        self.id_to_str.clear();
        self.map.clear();
    }
    fn get_next_idx(&mut self) -> usize {
        let idx = self.idx;
        self.idx += 1;
        idx
    }
}
