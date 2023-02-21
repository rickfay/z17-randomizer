use {serde::Serialize, std::collections::BTreeMap};

#[derive(Default, Debug, Serialize)]
pub struct Metrics {
    spheres: usize,
    playthrough: BTreeMap<String, BTreeMap<&'static str, &'static str>>,
    hints: Vec<String>,
}

impl Metrics {
    pub fn new(
        spheres: usize, playthrough: BTreeMap<String, BTreeMap<&'static str, &'static str>>,
        hints: Vec<String>,
    ) -> Self {
        Self { spheres, playthrough, hints }
    }
}
