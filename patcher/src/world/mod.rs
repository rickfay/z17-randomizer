use std::collections::HashMap;
use jack::rom::Rom;
use seed::Seed;

mod byaml;

///
pub(crate) fn patch(seed: &Seed, assets: &mut Rom) {
    byaml::patch();

    todo!()
}
