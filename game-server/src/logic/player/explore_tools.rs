use std::collections::HashSet;

use shorekeeper_data::explore_tools_data;
use shorekeeper_protocol::{
    ExploreSkillRoulette, ExploreSkillRouletteUpdateNotify, ExploreToolAllNotify,
    PlayerExploreToolsData,
};

type Roulette = [i32; 8];

pub struct ExploreTools {
    pub unlocked_explore_skills: HashSet<i32>,
    pub active_explore_skill: i32,
    pub roulette: Roulette,
}

impl ExploreTools {
    pub fn build_save_data(&self) -> PlayerExploreToolsData {
        PlayerExploreToolsData {
            unlocked_skill_list: self.unlocked_explore_skills.iter().cloned().collect(),
            active_skill_id: self.active_explore_skill,
            roulette: self.roulette.iter().cloned().collect(),
        }
    }

    pub fn load_from_save(data: PlayerExploreToolsData) -> Self {
        Self {
            unlocked_explore_skills: data.unlocked_skill_list.into_iter().collect(),
            active_explore_skill: data.active_skill_id,
            roulette: data
                .roulette
                .try_into()
                .unwrap_or_else(|_| Self::default_roulette()),
        }
    }

    pub fn build_explore_tool_all_notify(&self) -> ExploreToolAllNotify {
        ExploreToolAllNotify {
            skill_list: self.unlocked_explore_skills.iter().cloned().collect(),
            explore_skill: self.active_explore_skill,
            ..Default::default()
        }
    }

    pub fn build_roulette_update_notify(&self) -> ExploreSkillRouletteUpdateNotify {
        ExploreSkillRouletteUpdateNotify {
            roulette_info: vec![ExploreSkillRoulette {
                skill_ids: self.roulette.iter().cloned().collect(),
                extra_item_id: 0,
            }],
        }
    }

    fn default_roulette() -> Roulette {
        let mut roulette = [0i32; 8];
        explore_tools_data::iter()
            .take(3)
            .enumerate()
            .for_each(|(i, e)| roulette[i] = e.phantom_skill_id);

        roulette
    }
}

impl Default for ExploreTools {
    fn default() -> Self {
        let mut unlocked_skills: HashSet<i32> = explore_tools_data::iter()
            .filter(|e| e.authorization.is_empty())
            .map(|e| e.phantom_skill_id)
            .collect();


        unlocked_skills.insert(1004);
        unlocked_skills.insert(1003);
        unlocked_skills.insert(1005);
        unlocked_skills.insert(1006);
        unlocked_skills.insert(1007);
        unlocked_skills.insert(1009);
        unlocked_skills.insert(1010);
        unlocked_skills.insert(1011);
        unlocked_skills.insert(1012);
        unlocked_skills.insert(1013);
        unlocked_skills.insert(3001);
        unlocked_skills.insert(3002);

        Self {
            unlocked_explore_skills: unlocked_skills,
            active_explore_skill: 1001,
            roulette: Self::default_roulette(),
        }
    }
}