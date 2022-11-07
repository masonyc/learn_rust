pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&self, key: String, value: String) {}

    pub fn get(&self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&self, key: String) {}
}
