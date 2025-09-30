#[derive(Default)]
pub struct SymbolGenerator {
    counter: u128,
}

impl SymbolGenerator {
    pub fn gensym(&mut self, name: &str) -> String {
        let out = format!("#{}#{}", name, self.counter);
        self.counter += 1;
        out
    }
}
