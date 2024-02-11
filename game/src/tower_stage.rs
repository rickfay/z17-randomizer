use crate::Course;
use crate::Course::{EnemyAttackL, EnemyAttackM, EnemyAttackS};
use serde::{Serialize, Serializer};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct TowerStage {
    pub course: Course,
    pub stage: usize,
}

impl TowerStage {
    pub fn new(course: Course, stage: usize) -> Self {
        Self { course, stage }
    }
}

impl Serialize for TowerStage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let course_name = match self.course {
            EnemyAttackS => "Beginner    ",
            EnemyAttackM => "Intermediate",
            EnemyAttackL => "Advanced    ",
            _ => panic!("Invalid course"),
        };

        serializer.serialize_str(&format!("{} {: >2}F", course_name, self.stage))
    }
}
