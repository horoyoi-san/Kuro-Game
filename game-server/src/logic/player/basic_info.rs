use shorekeeper_protocol::{
    player_attr, BasicInfoNotify, PlayerAttr, PlayerAttrKey, PlayerAttrType, PlayerBasicData,
};

pub struct PlayerBasicInfo {
    pub id: i32,
    pub name: String,
    pub sex: i32,
    pub level: i32,
    pub exp: i32,
    pub head_photo: i32,
    pub head_frame: i32,
    pub cur_map_id: i32,
    pub role_show_list: Vec<i32>,
    pub origin_world_level: i32, // เพิ่มฟิลด์นี้
    pub cur_world_level: i32,     // เพิ่มฟิลด์นี้
    pub sign: String,             // เพิ่มฟิลด์นี้
}

impl PlayerBasicInfo {
    pub fn build_notify(&self) -> BasicInfoNotify {
        BasicInfoNotify {
            id: self.id,
            attributes: vec![
                build_str_attr(PlayerAttrKey::Name, self.name.as_str()),
                build_int_attr(PlayerAttrKey::Level, self.level),
                build_int_attr(PlayerAttrKey::Exp, self.exp),
                build_int_attr(PlayerAttrKey::Sex, self.sex),
                build_int_attr(PlayerAttrKey::HeadPhoto, self.head_photo),
                build_int_attr(PlayerAttrKey::HeadFrame, self.head_frame),
                build_int_attr(PlayerAttrKey::OriginWorldLevel, self.origin_world_level),
                build_int_attr(PlayerAttrKey::CurWorldLevel, self.cur_world_level),
                build_str_attr(PlayerAttrKey::Sign, self.sign.as_str()),
            ],
            ..Default::default()
        }
    }

    pub fn load_from_save(data: PlayerBasicData) -> Self {
        Self {
            id: data.id,
            name: data.name,
            sex: data.sex,
            level: data.level,
            exp: data.exp,
            head_photo: data.head_photo,
            head_frame: data.head_frame,
            cur_map_id: data.cur_map_id,
            role_show_list: data.role_show_list,
            origin_world_level: data.origin_world_level, // ใช้ฟิลด์ที่เพิ่มเข้ามา
            cur_world_level: data.cur_world_level,       // ใช้ฟิลด์ที่เพิ่มเข้ามา
            sign: data.sign,                             // ใช้ฟิลด์ที่เพิ่มเข้ามา
        }
    }

    pub fn build_save_data(&self) -> PlayerBasicData {
        PlayerBasicData {
            id: self.id,
            name: self.name.clone(),
            sex: self.sex,
            level: self.level,
            exp: self.exp,
            head_photo: self.head_photo,
            head_frame: self.head_frame,
            cur_map_id: self.cur_map_id,
            role_show_list: self.role_show_list.clone(),
            origin_world_level: self.origin_world_level, // ใช้ฟิลด์ที่เพิ่มเข้ามา
            cur_world_level: self.cur_world_level,       // ใช้ฟิลด์ที่เพิ่มเข้ามา
            sign: self.sign.clone(),                      // ใช้ฟิลด์ที่เพิ่มเข้ามา
        }
    }
}

#[inline]
fn build_int_attr(key: PlayerAttrKey, value: i32) -> PlayerAttr {
    PlayerAttr {
        key: key.into(),
        value_type: PlayerAttrType::Int32.into(),
        value: Some(player_attr::Value::Int32Value(value)),
    }
}

#[inline]
fn build_str_attr(key: PlayerAttrKey, value: &str) -> PlayerAttr {
    PlayerAttr {
        key: key.into(),
        value_type: PlayerAttrType::String.into(),
        value: Some(player_attr::Value::StringValue(value.to_string())),
    }
}
