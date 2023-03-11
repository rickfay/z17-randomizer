use {crate::model::Hints, serde::Serialize, std::collections::BTreeMap};

#[derive(Default, Debug, Serialize)]
pub struct Metrics {
    spheres: usize,
    playthrough: BTreeMap<String, BTreeMap<&'static str, &'static str>>,
    hints: Hints,
}

impl Metrics {
    pub fn new(
        spheres: usize, playthrough: BTreeMap<String, BTreeMap<&'static str, &'static str>>,
        hints: Hints,
    ) -> Self {
        Self { spheres, playthrough, hints }
    }

    pub fn get_hints(self) -> Hints {
        self.hints
    }
}
