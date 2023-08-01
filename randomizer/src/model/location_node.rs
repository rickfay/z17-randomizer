use crate::{model::check::Check, world::Path};

#[derive(Clone)]
pub struct LocationNode {
    checks: Vec<Check>,
    paths: Vec<Path>,
}

impl LocationNode {
    pub fn new(_name: &'static str, checks: Vec<Check>, paths: Vec<Path>) -> Self {
        Self { checks, paths }
    }

    pub fn get_checks(&self) -> &Vec<Check> {
        &self.checks
    }

    pub fn get_paths(self) -> Vec<Path> {
        self.paths
    }
}
