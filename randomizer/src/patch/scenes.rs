use albw::{
    course,
    scene::{Flag, Obj},
};

use super::Patcher;
use crate::Result;

macro_rules! apply {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(course::Id::$course, $stage - 1)?.stage_mut().get_mut();
            $(action!((stage
                .get_mut($unq)
                .ok_or_else(|| $crate::Error::game("Could not find scene."))?
            ).$action $value);)+
        })+
    };
}

macro_rules! action {
    ($unq:tt.id($id:literal)) => {
        $unq.set_id($id);
    };
    ($unq:tt.each [$($action:ident $value:tt,)+]) => {
        $(action!($unq.$action $value);)+
    };
    ($unq:tt.active(0)) => {
        $unq.set_active_flag(None);
    };
    ($unq:tt.active($flag:literal)) => {
        $unq.set_active_flag(Flag::Event($flag));
    };
    ($unq:tt.inactive($flag:literal)) => {
        $unq.set_inactive_flag(Flag::Event($flag));
    };
    ($unq:tt.enable($flag:literal)) => {
        $unq.set_enable_flag(Flag::Event($flag));
    };
    ($unq:tt.disable($flag:expr)) => {
        $unq.set_disable_flag($flag);
    };
    ($unq:tt.enable()) => {
        $unq.enable();
    };
    ($unq:tt.disable()) => {
        $unq.disable();
    };
    ($unq:tt.call $fn:block) => {
        ($fn)($unq);
    };
}

pub fn apply(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        // Lost Woods
        FieldLight 1 {
            [34].active(375), // Skip Poes
        },
        // Outside Rosso's house
        FieldLight 2 {
            [100].disable(None), // Keep Entry_KikoriMan3 from disappearing
            [101].disable(None),
            [135].disable(), // Disable IndoorLight4
            [136].enable(250), // Replace with IndoorLight10
        },
        // Outside witch's house
        FieldLight 14 {
            [123].disable(), // Disable surprised Zora
        },
        // Kakariko Village
        FieldLight 16 {
            [197].disable(), // Disable merchant's Smooth Gem text
            [299].disable(), // Disable merchant's bottle text
        },
        // Outside your house
        FieldLight 27 {
            [158].disable(), // Disable Blacksmith's Wife
        },
        // Outside woods
        FieldLight 32 {
            [76].disable(), // Disable Blacksmith's Wife
        },
        // Master Sword
        FieldLight 34 {
            [71].each [
                active(0),
                call {|obj: &mut Obj| {
                    let arg = obj.arg_mut();
                    arg.5 = 3;
                    arg.7 = 150;
                }},
                enable(),
                id(0x23), // replace with chest
            ],
        },
        // Your house
        IndoorLight 1 {
            [46].disable(), // Disable Ravio's bye-bye
            [54].disable(), // Disable Ravio's welcome
            [55].disable(Flag::Course(244)),
            [56].disable(Flag::Course(244)),
            [57].disable(Flag::Course(244)),
            [58].disable(), // Disable Ravio's welcome
            [59].disable(), // Disable Ravio's welcome
        },
        // Rosso's house
        IndoorLight 10 {
            [7].each [
                id(35),
                inactive(282),
                enable(),
            ],
        },
        // Hyrule Castle
        IndoorLight 12 {
            [48].call {|obj: &mut Obj| {obj.arg_mut().5 = 3;}},
        },
        // Bar
        IndoorLight 15 {
            [15].disable(), // Disable post Climber dialogue?
        },
        // Blacksmith's House
        IndoorLight 19 {
            [0x10].disable(), // Remove captain
        },
        // Zora's Domain
        CaveLight 7 {
            [0x84].enable(), // Enable Zora Queen event always
        },
        // Eastern Palace
        DungeonEast 3 {
            // Open door after defeating Yuga
            [0x5D].each [
                inactive(250),
                enable(),
            ],
        },
        // Hyrule Castle
        DungeonCastle 7 {
            [19].enable(415),
            [20].enable(415),
            [21].enable(415),
            [22].enable(415),
        },
    );

    //if settings.behavior.barrier.is_start() {

    apply!(patcher,
        // Hyrule Castle
        FieldLight 18 {
            [155].enable(1),
            [165].active(1),
            [393].disable(Flag::Event(1)),
        },
    );

    //}
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::apply;
    use crate::{patch::Patcher, test_game, Result};

    #[test]
    fn it_works() -> Result<()> {
        let mut patcher = Patcher::new(test_game()?)?;
        apply(&mut patcher)
    }
}
