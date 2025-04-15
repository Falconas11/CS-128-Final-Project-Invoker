
use std::collections::HashMap;

#[derive(Debug)]
pub struct Invoker {
    pub combos: HashMap<&'static str, &'static str>,
}

impl Invoker {
    pub fn new() -> Self {
        let mut combos = HashMap::new();
        combos.insert("QQQ", "Cold Snap");
        combos.insert("QQW", "Ghost Walk");
        combos.insert("QWQ", "Ghost Walk");
        combos.insert("WQQ", "Ghost Walk");
        combos.insert("QQE", "Ice Wall");
        combos.insert("EQQ", "Ice Wall");
        combos.insert("QEQ", "Ice Wall");
        combos.insert("WWW", "EMP");
        combos.insert("WWQ", "Tornado");
        combos.insert("WQW", "Tornado");
        combos.insert("QWW", "Tornado");
        combos.insert("WWE", "Alacrity");
        combos.insert("EWW", "Alacrity");
        combos.insert("WEW", "Alacrity");
        combos.insert("EEE", "Sun Strike");
        combos.insert("EEQ", "Forge Spirit");
        combos.insert("QEE", "Forge Spirit");
        combos.insert("EQE", "Forge Spirit");
        combos.insert("EEW", "Chaos Meteor");
        combos.insert("EWE", "Chaos Meteor");
        combos.insert("WEE", "Chaos Meteor");
        combos.insert("QWE", "Deafening Blast");
        combos.insert("QEW", "Deafening Blast");
        combos.insert("EQW", "Deafening Blast");
        combos.insert("EWQ", "Deafening Blast");
        combos.insert("WQE", "Deafening Blast");
        combos.insert("WEQ", "Deafening Blast");

        Self { combos }
    }

    pub fn get_spell(&self, combo: &str) -> Option<&&str> {
        self.combos.get(combo)
    }

    pub fn random_spell(&self) -> (&'static str, &'static str) {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        self.combos.iter().choose(&mut rng).map(|(k, v)| (*k, *v)).unwrap()
    }
}
