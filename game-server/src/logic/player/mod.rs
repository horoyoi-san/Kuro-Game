use common::time_util;
use shorekeeper_protocol::{
    EEntityType, EntityState, ERemoveEntityType, EntityAddNotify, EntityConfigType, EntityPb, EntityRemoveInfo,
    EntityRemoveNotify, FightFormationNotifyInfo, FightRoleInfo, FightRoleInfos, FormationRoleInfo,
    GroupFormation, ItemPkgOpenNotify, LivingStatus, PbGetRoleListNotify, PlayerBasicData,
    PlayerFightFormations, PlayerRoleData, PlayerSaveData, ProtocolUnit, UpdateFormationNotify,
    UpdateGroupFormationNotify,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, Movement, OwnerPlayer, PlayerEntityMarker, Position,
        Visibility, VisionSkill,
    },
    ecs::component::ComponentContainer,
};
use crate::session::Session;

use super::{
    ecs::world::World,
    role::{Role, RoleFormation},
};

mod basic_info;
mod explore_tools;
mod in_world_player;
mod location;
mod player_func;

use crate::create_player_entity_pb;
use crate::logic::ecs::world::WorldEntity;
use crate::logic::player::basic_info::PlayerBasicInfo;
use crate::logic::player::explore_tools::ExploreTools;
use crate::logic::player::location::PlayerLocation;
use crate::logic::player::player_func::PlayerFunc;
pub use in_world_player::InWorldPlayer;
use shorekeeper_data::base_property_data;
use shorekeeper_data::role_info_data;
use shorekeeper_protocol::message::Message;

pub struct Player {
    session: Option<Arc<Session>>,
    // Persistent
    pub basic_info: PlayerBasicInfo,
    pub role_list: HashMap<i32, Role>,
    pub formation_list: HashMap<i32, RoleFormation>,
    pub cur_formation_id: i32,
    pub location: PlayerLocation,
    pub func: PlayerFunc,
    pub explore_tools: ExploreTools,
    // Runtime
    pub world: Rc<RefCell<World>>,
    pub last_save_time: u64,
    pub quadrant_id: u64,
}

impl Player {
    pub fn init(&mut self) {
        if self.role_list.is_empty() || self.formation_list.is_empty() {
            self.init_role_and_formation();
        }

        self.ensure_basic_unlock_func();
    }

    pub fn notify_general_data(&self) {
        self.notify(self.basic_info.build_notify());
        self.notify(self.func.build_func_open_notify());
        self.notify(self.build_role_list_notify());
        self.notify(self.explore_tools.build_explore_tool_all_notify());
        self.notify(self.explore_tools.build_roulette_update_notify());

        self.notify(ItemPkgOpenNotify {
            open_pkg: (0..8).collect(),
        });

        self.notify(self.build_update_formation_notify());
    }

    fn init_role_and_formation(&mut self) {
        self.role_list.clear();
        let mut role = match self.basic_info.sex {
            0 => Role::new(Role::MAIN_CHARACTER_FEMALE_ID),
            1 => Role::new(Role::MAIN_CHARACTER_MALE_ID),
            _ => unreachable!(),
        };

        role.name = self.basic_info.name.clone();

        self.role_list.insert(role.role_id, role);

        let required_role_ids: Vec<i32> = role_info_data::iter()
            .filter(|role_info| role_info.role_type == 1)
            .map(|role_info| role_info.id)
            .collect();
        let formation = vec![1603, 1504];

        required_role_ids.iter().for_each(|&role_id| {
            if !self.role_list.keys().any(|&k| k == role_id) {
                self.role_list.insert(role_id, Role::new(role_id));
            }
        });

        self.formation_list.insert(
            1,
            RoleFormation {
                id: 1,
                cur_role: *formation.iter().next().unwrap(),
                role_ids: formation,
                is_current: true,
            },
        );
        self.cur_formation_id = 1;

        self.formation_list.values_mut().for_each(|formation| {
            if formation.is_current && formation.id != 1 {
                formation.is_current = false;
            }
        });

        self.ensure_current_formation();
    }

    // Ensure basic functionality is unlocked
    // Should be handled by quest progression,
    // but as of right now, just unlock what we need
    fn ensure_basic_unlock_func(&mut self) {
        // หมวดหมู่: เครื่องมือสำหรับการสำรวจ
        self.func.unlock(10001); // เครื่องสะท้อนเสียง - Sound Reflection Machine
        self.func.unlock(10026); // เครื่องมือสำรวจ - Explore Tools
        self.func.unlock(10027); // ฐานส่งต่อ - Relay Base
        self.func.unlock(10026001); // การรับรู้สนามเสียง - Sound Field Recognition
        self.func.unlock(10026002); // ตะขอขับช่วงล่าง - Lower Range Hook Drive
        self.func.unlock(10026003); // ไดรเวอร์ช่วงล่าง - Lower Range Driver
        self.func.unlock(10026006); // การสร้างภาพเสียง - Sound Imaging
    
        // หมวดหมู่: การปรับแต่งผู้เล่น
        self.func.unlock(10002); // กระเป๋าเป้สะพายหลัง - Backpack
        self.func.unlock(10009); // การปรับแต่ง - Customization
        self.func.unlock(10052); // การแปลงลำดับเสียง - Sound Sequence Conversion
    
        // หมวดหมู่: ระบบสังคม
        self.func.unlock(10010); // การแลกเปลี่ยนสหภาพ - Union Exchange
        self.func.unlock(10011); // เพื่อน - Friends
        self.func.unlock(10012); // กิลด์ - Guild
        self.func.unlock(10072); // การเชื่อมโยงอีเมล - Email Linking
        self.func.unlock(10070); // พันธมิตรเทศกาลไล่ดวงจันทร์ - Moon Chase Festival Alliance
    
        // หมวดหมู่: ระบบการต่อสู้และทักษะ
        self.func.unlock(10040); // คำสั่งการต่อสู้ BP - BP Battle Orders
        self.func.unlock(10046); // อาร์เรย์โฟกัส - Focus Array
        self.func.unlock(10047); // กระแสเสียงที่วุ่นวาย - Noisy Soundwaves
    
        // หมวดหมู่: กิจกรรมและอีเวนต์พิเศษ
        self.func.unlock(10006); // กิจกรรมในเวลา จำกัด - Limited-Time Events
        self.func.unlock(10067); // ตัวแทนเทศกาลไล่ดวงจันทร์ - Moon Chase Festival Representative
        self.func.unlock(10068); // การก่อสร้างเทศกาลไล่ดวงจันทร์ - Moon Chase Festival Construction
        self.func.unlock(10069); // กระดานเรื่องราวเทศกาลไล่ดวงจันทร์ - Moon Chase Festival Storyboard
        self.func.unlock(10070); // พันธมิตรเทศกาลไล่ดวงจันทร์ - Moon Chase Festival Alliance
    
        // หมวดหมู่: อื่นๆ
        self.func.unlock(10003); // เทอร์มินัล Mingyu - Mingyu Terminal
        self.func.unlock(10004); // งาน - Jobs
        self.func.unlock(10005); // การเล่นเกมแบบ Resident - Resident Game Mode
        self.func.unlock(10013); // ความสำเร็จ - Achievements
        self.func.unlock(10014); // หนังสือภาพ - Picture Book
        self.func.unlock(10015); // แผนที่ - Map
        self.func.unlock(10016); // สำนักงานจัดหางาน Resonator - Resonator Job Board
        self.func.unlock(10017); // ความแข็งแกร่งทางกายภาพ - Physical Strength
        self.func.unlock(10018); // เวลา - Time
        self.func.unlock(10019); // การตั้งค่าระบบ - System Settings
        self.func.unlock(10020); // อีเมล - Email
        self.func.unlock(10021); // ออนไลน์ - Online Mode
        self.func.unlock(10022); // บทช่วยสอน - Tutorial
        self.func.unlock(10023); // รับและบันทึกเสียง - Audio Recording
        self.func.unlock(10024); // การมอบหมายการประชุมล่วงหน้า - Pre-assigned Meetings
        self.func.unlock(10025); // เดี่ยวแปลก ๆ - Strange Solo
        self.func.unlock(10028); // ความคิดเห็นของผู้ใช้ - User Feedback
        self.func.unlock(10029); // พลัง - Power
        self.func.unlock(10030); // สถานีวิจัยวิทยาศาสตร์กำเนิด - Genesis Science Research Station
        self.func.unlock(10031); // ล็อคทักษะ - Skill Lock
        self.func.unlock(10032); // พื้นที่เงียบ - Quiet Zone
        self.func.unlock(10033); // ระบบทำอาหาร - Cooking System
        self.func.unlock(10034); // การพิมพ์แบบรวม - Collective Printing
        self.func.unlock(10035); // ระบบสังเคราะห์ - Synthesis System
        self.func.unlock(10036); // คอนแชร์โต - Concerto
        self.func.unlock(10037); // การเปลี่ยนเพศ - Gender Change
    
        // หมวดหมู่: อุปกรณ์การฝึกอบรม
        self.func.unlock(10042); // การแลกเปลี่ยนพูลมังกร - Dragon Pool Exchange
        self.func.unlock(10043); // อุปกรณ์การฝึกอบรม - Training Equipment
    
        // หมวดหมู่: ฟังก์ชันพิเศษ
        self.func.unlock(10055); // หอปีนเขาใหม่ - New Climbing Tower
        self.func.unlock(10056); // วงล้อฟังก์ชัน - Function Wheel
        self.func.unlock(10057); // รางวัลการสำรวจ - Exploration Rewards
        self.func.unlock(10058); // บล็อกไลบรารี - Library Block
        self.func.unlock(10059); // อาณาจักรแห่งเสียง - Sound Kingdom
        self.func.unlock(10060); // ระบบส่วนบุคคล - Personal System
        self.func.unlock(10061); // ประทับตรา Jijing - Jijing Stamp
        self.func.unlock(10062); // โจวเบน - เต่า - Zhou Ben - Turtle
        self.func.unlock(10063); // โจวเบน - สการ์ - Zhou Ben - Scar
        self.func.unlock(10064); // Zhou Ben - รูปแบบที่แท้จริงของสิ่งที่ไม่มีมงกุฎ - Zhou Ben - True Form of the Crownless
        self.func.unlock(10065); // ? - Unknown
        self.func.unlock(10066); // พื้นที่เก็บข้อมูลความแข็งแกร่ง - Strength Storage Area
    }
    

    fn ensure_current_formation(&mut self) {
        // If the list off formation is empty, add a default formation
        if self.formation_list.is_empty() {
            let mut role_list_clone = self.role_list.iter().clone();

            self.formation_list.insert(
                1,
                RoleFormation {
                    id: 1,
                    cur_role: role_list_clone.next().unwrap().1.role_id,
                    role_ids: role_list_clone
                        .take(3)
                        .map(|(&role_id, _)| role_id)
                        .collect(),
                    is_current: true,
                },
            );
        }

        // If there is no current formation, set the first formation as the current formation
        if !self.formation_list.values().any(|rf| rf.is_current) {
            self.formation_list.get_mut(&1).unwrap().is_current = true;
        }

        // Ensure that the set of character IDs for the current formation is not empty and that the current character ID is in the set
        if let Some(rf) = self.formation_list.values_mut().find(|rf| rf.is_current) {
            if rf.role_ids.is_empty() {
                rf.role_ids
                    .push(self.role_list.iter().next().unwrap().1.role_id);
            }

            if !rf.role_ids.contains(&rf.cur_role) {
                rf.cur_role = *rf.role_ids.iter().nth(0).unwrap();
            }
        }
    }

    pub fn build_in_world_player(&self) -> InWorldPlayer {
        InWorldPlayer {
            player_id: self.basic_info.id,
            player_name: self.basic_info.name.clone(),
            player_icon: 0,
            level: self.basic_info.level,
            group_type: 1,
        }
    }

    pub fn build_player_entity_add_notify(
        &self,
        role_list: Vec<Role>,
        world: &mut WorldEntity,
    ) -> EntityAddNotify {
        create_player_entity_pb!(
            role_list,
            self.basic_info.cur_map_id,
            world,
            self.basic_info.id,
            self.location.position.clone(),
            self.explore_tools
        )
    }

    pub fn build_player_entity_remove_notify(
        &self,
        entities: Vec<i64>,
        remove_type: ERemoveEntityType,
    ) -> EntityRemoveNotify {
        EntityRemoveNotify {
            remove_infos: entities
                .iter()
                .map(|&entity_id| EntityRemoveInfo {
                    entity_id,
                    r#type: remove_type.into(),
                })
                .collect(),
            is_remove: true,
        }
    }

    pub fn build_update_group_formation_notify(
        &self,
        cur_formation: RoleFormation,
        world: &mut WorldEntity,
    ) -> UpdateGroupFormationNotify {
        let group_type = 1;
        UpdateGroupFormationNotify {
            group_formation: vec![GroupFormation {
                player_id: self.basic_info.id,
                fight_role_infos: vec![FightRoleInfos {
                    group_type,
                    fight_role_infos: cur_formation
                        .role_ids
                        .iter()
                        .map(|&role_id| FightRoleInfo {
                            role_id,
                            entity_id: world.get_entity_id(role_id),
                        })
                        .collect(),
                    cur_role: cur_formation.cur_role,
                    is_retain: false,
                    living_status: LivingStatus::Alive.into(),
                    is_fixed_location: false,
                }],
                current_group_type: group_type,
            }],
        }
    }

    pub fn build_update_formation_notify(&self) -> UpdateFormationNotify {
        let role_map: HashMap<i32, (i32, i32)> = self
            .role_list
            .values()
            .map(|role| (role.role_id, (role.role_id, role.level)))
            .collect();

        UpdateFormationNotify {
            players_formations: vec![PlayerFightFormations {
                player_id: self.basic_info.id,
                formations: self
                    .formation_list
                    .iter()
                    .map(|(&formation_id, formation)| FightFormationNotifyInfo {
                        formation_id,
                        cur_role: formation.cur_role,
                        role_infos: formation
                            .role_ids
                            .iter()
                            .map(|role_id| {
                                if !role_map.contains_key(role_id) {
                                    tracing::warn!("Role {} not found in use role list", role_id);
                                    return Default::default();
                                }
                                let &(role_id, level) = role_map.get(&role_id).unwrap();
                                FormationRoleInfo {
                                    role_id,
                                    max_hp: 0,
                                    cur_hp: 0,
                                    level,
                                }
                            })
                            .collect(),
                        is_current: formation.is_current,
                    })
                    .collect(),
            }],
        }
    }

    pub fn load_from_save(save_data: PlayerSaveData) -> Self {
        let role_data = save_data.role_data.unwrap_or_default();

        Self {
            session: None,
            basic_info: PlayerBasicInfo::load_from_save(
                save_data.basic_data.clone().unwrap_or_default(),
            ),
            role_list: role_data
                .role_list
                .into_iter()
                .map(Role::load_from_save)
                .collect::<HashMap<i32, Role>>(),
            formation_list: role_data
                .role_formation_list
                .into_iter()
                .map(|(k, v)| (k, RoleFormation::load_from_save(v)))
                .collect(),
            cur_formation_id: role_data.cur_formation_id,
            location: save_data
                .location_data
                .map(PlayerLocation::load_from_save)
                .unwrap_or_default(),
            func: save_data
                .func_data
                .map(PlayerFunc::load_from_save)
                .unwrap_or_default(),
            explore_tools: save_data
                .explore_tools_data
                .map(ExploreTools::load_from_save)
                .unwrap_or_default(),
            world: Rc::new(RefCell::new(World::new())),
            last_save_time: time_util::unix_timestamp(),
            quadrant_id: 0,
        }
    }

    pub fn build_save_data(&self) -> PlayerSaveData {
        PlayerSaveData {
            basic_data: Some(self.basic_info.build_save_data()),
            role_data: Some(PlayerRoleData {
                role_list: self
                    .role_list
                    .iter()
                    .map(|(_, role)| role.build_save_data())
                    .collect(),
                role_formation_list: self
                    .formation_list
                    .iter()
                    .map(|(&k, v)| (k, v.build_save_data()))
                    .collect(),
                cur_formation_id: self.cur_formation_id,
            }),
            location_data: Some(self.location.build_save_data()),
            func_data: Some(self.func.build_save_data()),
            explore_tools_data: Some(self.explore_tools.build_save_data()),
        }
    }

    pub fn set_session(&mut self, session: Arc<Session>) {
        self.session = Some(session);
    }

    pub fn build_role_list_notify(&self) -> PbGetRoleListNotify {
        PbGetRoleListNotify {
            role_list: self
                .role_list
                .iter()
                .map(|(_, role)| role.to_protobuf())
                .collect(),
        }
    }

    pub fn notify(&self, content: impl ProtocolUnit) {
        if let Some(session) = self.session.as_ref() {
            session.forward_to_gateway(Message::Push {
                sequence_number: 0,
                message_id: content.get_message_id(),
                payload: Some(content.encode_to_vec().into_boxed_slice()),
            });
        }
    }

    pub fn respond(&self, content: impl ProtocolUnit, rpc_id: u16) {
        if let Some(session) = self.session.as_ref() {
            session.forward_to_gateway(Message::Response {
                sequence_number: 0,
                message_id: content.get_message_id(),
                rpc_id,
                payload: Some(content.encode_to_vec().into_boxed_slice()),
            });
        }
    }

    pub fn create_default_save_data(id: i32, name: String, sex: i32) -> PlayerSaveData {
        let role_id = match sex {
            0 => Role::MAIN_CHARACTER_FEMALE_ID, // 1502
            1 => Role::MAIN_CHARACTER_MALE_ID,   // 1501
            _ => Role::MAIN_CHARACTER_MALE_ID,   // Default to male
        };

        PlayerSaveData {
            basic_data: Some(PlayerBasicData {
                id: 90085205,
                name,
                sex,
                level: 90,
                head_photo: 1603,
                head_frame: 80060009,
                cur_map_id: 8,
                role_show_list: vec![role_id],
                origin_world_level: 5, // ค่าเริ่มต้น
                cur_world_level: 11,     // ค่าเริ่มต้น
                sign: String::new(),    // ค่าเริ่มต้น
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
