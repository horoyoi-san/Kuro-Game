use std::collections::HashMap;

use shorekeeper_data::function_condition_data;
use shorekeeper_protocol::{FuncOpenNotify, Function, PlayerFuncData};

pub struct PlayerFunc {
    pub func_map: HashMap<i32, i32>,
}

impl PlayerFunc {
    pub fn unlock(&mut self, id: i32) {
        self.func_map.insert(id, 2);
    }

    pub fn unlock_icon_only(&mut self, id: i32) {
        self.func_map.insert(id, 1); // ปลดล็อกแสดงเฉพาะไอคอน
    }
    
    pub fn load_from_save(data: PlayerFuncData) -> Self {
        PlayerFunc {
            func_map: data.func_map,
        }
    }

    pub fn build_save_data(&self) -> PlayerFuncData {
        PlayerFuncData {
            func_map: self.func_map.clone(),
        }
    }

    pub fn build_func_open_notify(&self) -> FuncOpenNotify {
        FuncOpenNotify {
            func: self
                .func_map
                .iter()
                .map(|(id, flag)| Function {
                    id: *id,
                    flag: *flag,
                })
                .collect(),
        }
    }
}

impl Default for PlayerFunc {
    fn default() -> Self {
        let mut func_map = function_condition_data::iter()
            .filter(|fc| fc.is_on && fc.open_condition_id == 0)
            .map(|fc| (fc.function_id, 2))
            .collect::<HashMap<i32, i32>>();

        // ตัวอย่างเช่น ถ้าผู้เล่นอยู่ใน Level 10 ปลดล็อกฟังก์ชันใหม่

        // ตั้งค่าฟังก์ชันที่ปลดล็อกเฉพาะไอคอนที่คุณต้องการ
        // เช่น สมมติว่าคุณต้องการปลดล็อกไอคอนของฟังก์ชันที่มี ID 10009
        func_map.insert (10001,1); // เครื่องสะท้อนเสียง
        func_map.insert (10002,1); // กระเป๋าเป้สะพายหลัง
        func_map.insert (10003,1); // เทอร์มินัล Mingyu
        func_map.insert (10004,1); // งาน
        func_map.insert (10005,1); // การเล่นเกมแบบ Resident
        func_map.insert (10006,1); // กิจกรรมในเวลา จำกัด
        func_map.insert (10007,1); // การก่อตัว
        func_map.insert (10008,1); // รวมทีม
        func_map.insert (10009,1); // การปรับแต่ง
        func_map.insert (10010,1); // การแลกเปลี่ยนสหภาพ
        func_map.insert (10011,1); // เพื่อน
        func_map.insert (10012,1); // กิลด์
        func_map.insert (10013,1); // ความสำเร็จ
        func_map.insert (10014,1); // หนังสือภาพ
        func_map.insert (10015,1); // แผนที่
        func_map.insert (10016,1); // สำนักงานจัดหางาน Resonator
        func_map.insert (10017,1); // ความแข็งแกร่งทางกายภาพ
        func_map.insert (10018,1); // เวลา
        func_map.insert (10019,1); // การตั้งค่าระบบ
        func_map.insert (10020,1); // อีเมล
        func_map.insert (10021,1); // ออนไลน์
        func_map.insert (10022,1); // บทช่วยสอน
        func_map.insert (10023,1); // รับและบันทึกเสียง
        func_map.insert (10024,1); // การมอบหมายการประชุมล่วงหน้า
        func_map.insert (10025,1); // เดี่ยวแปลก ๆ

        func_map.insert (10027,1); // ฐานส่งต่อ
        func_map.insert (10028,1); // ความคิดเห็นของผู้ใช้
        func_map.insert (10029,1); // พลัง
        func_map.insert (10030,1); // สถานีวิจัยวิทยาศาสตร์กำเนิด
        func_map.insert (10031,1); // ล็อคทักษะ
        func_map.insert (10032,1); // พื้นที่เงียบ
        func_map.insert (10033,1); // ระบบทำอาหาร
        func_map.insert (10034,1); // การพิมพ์แบบรวม
        func_map.insert (10035,1); // ระบบสังเคราะห์
        func_map.insert (10036,1); // คอนแชร์โต
        func_map.insert (10037,1); // การเปลี่ยนเพศ
       
       
        func_map.insert (10040,1); // คำสั่งการต่อสู้ BP
        func_map.insert (10041,1); // ภาพประกอบตัวละคร
        func_map.insert (10042,1); // การแลกเปลี่ยนพูลมังกร
        func_map.insert (10043,1); // อุปกรณ์การฝึกอบรม
       
        func_map.insert (10046,1); // อาร์เรย์โฟกัส
        func_map.insert (10047,1); // กระแสเสียงที่วุ่นวาย
        func_map.insert (10048,1); // จอแสดงผลย้อนหลัง
        func_map.insert (10049,1); // การรวบรวมรูปภาพ
        func_map.insert (10050,1); // ข้อความย้อนหลัง
        func_map.insert (10051,1); // หนังสือภาพประกอบของทุกสิ่ง
        func_map.insert (10052,1); // การแปลงลำดับเสียง
        func_map.insert (10053,1); // กิจกรรม
        func_map.insert (10054,1); // กิจกรรม
        func_map.insert (10055,1); // หอปีนเขาใหม่
        func_map.insert (10056,1); // วงล้อฟังก์ชัน
        func_map.insert (10057,1); // รางวัลการสำรวจ
        func_map.insert (10058,1); // บล็อกไลบรารี
        func_map.insert (10059,1); // อาณาจักรแห่งเสียง
        func_map.insert (10060,1); // ระบบส่วนบุคคล
        func_map.insert (10061,1); // ประทับตรา Jijing
        func_map.insert (10062,1); // โจวเบน - เต่า
        func_map.insert (10063,1); // โจวเบน - สการ์
        func_map.insert (10064,1); // Zhou Ben- รูปแบบที่แท้จริงของสิ่งที่ไม่มีมงกุฎ
        func_map.insert (10065,1); // ?
        func_map.insert (10066,1); // พื้นที่เก็บข้อมูลความแข็งแกร่ง
        func_map.insert (10067,1); // ตัวแทนเทศกาลไล่ดวงจันทร์
        func_map.insert (10068,1); // การก่อสร้างเทศกาลไล่ดวงจันทร์
        func_map.insert (10069,1); // กระดานเรื่องราวเทศกาลไล่ดวงจันทร์
        func_map.insert (10070,1); // พันธมิตรเทศกาลไล่ดวงจันทร์
        func_map.insert (10071,1); // คัดลอกสองเท่า
        func_map.insert (10072,1); // การเชื่อมโยงอีเมล
        func_map.insert (110058,1); // ลอร์ดแปรงใหม่
        func_map.insert (110056,1); // ลอร์ดแปรงใหม่
        func_map.insert (110057,1); // ลอร์ดยิม
        func_map.insert (10001004,1); // แท็บระบุโครงกระดูกเสียง
        func_map.insert (10023003,1); // คัดลอกครั้งเดียว
        func_map.insert (10024001,1); // การรวมข้อมูล
        func_map.insert (10026001,1); // การรับรู้สนามเสียง
        func_map.insert (10026002,1); // ตะขอขับช่วงล่าง
        func_map.insert (10026003,1); // ไดรเวอร์ช่วงล่าง
        func_map.insert (10026004,1); // วิ่งในแนวตั้ง
        func_map.insert (10026005,1); // ยกปีก
        func_map.insert (10026006,1); // การสร้างภาพเสียง
       
        func_map.insert (10026008,1); // การรับรูปภาพอย่างรวดเร็ว
        func_map.insert (10026009,1); // นักกีฬาลอยน้ำ
        func_map.insert (10026101,1); // ประกอบเสา
        func_map.insert (10001003,1); // แท็บ Resonator-Sound Bones
        func_map.insert (10023005,1); // การประชุมลำดับความสำคัญ
        func_map.insert (10023004,1); // อาณาจักรแห่งเสียง
        func_map.insert (10023002,1); // การสร้างภาพสะท้อน
        func_map.insert (10023001,1); // การรวบรวมเสียงกลับมา
        
        Self { func_map }
    }
}
