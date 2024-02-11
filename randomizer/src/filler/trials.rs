use modinfo::settings::trials_door::TrialsDoor;
use modinfo::Settings;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize)]
pub struct TrialsConfig {
    pub bomb_trial: bool,
    pub tile_trial: bool,
    pub lamp_trial: bool,
    pub hook_trial: bool,
}

pub(crate) fn configure(rng: &mut StdRng, settings: &Settings) -> crate::Result<TrialsConfig> {
    let count;
    match settings.trials_door {
        TrialsDoor::Off => {
            return Ok(TrialsConfig { bomb_trial: false, tile_trial: false, lamp_trial: false, hook_trial: false });
        },
        TrialsDoor::OneTrialRequired => {
            count = 1;
        },
        TrialsDoor::TwoTrialsRequired => {
            count = 2;
        },
        TrialsDoor::ThreeTrialsRequired => {
            count = 3;
        },
        TrialsDoor::AllTrialsRequired => {
            count = 4;
        },
    };

    let array = vec![1, 2, 3, 4];
    let chosen = array.choose_multiple(rng, count as usize).collect::<Vec<_>>();

    Ok(TrialsConfig {
        bomb_trial: chosen.contains(&&1),
        tile_trial: chosen.contains(&&2),
        lamp_trial: chosen.contains(&&3),
        hook_trial: chosen.contains(&&4),
    })
}
