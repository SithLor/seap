//ip paser for mango_lib 

pub struct IpV4 {
    ip:String, 
    part_1:u8,
    part_2:u8,
    part_3:u8,
    part_4:u8,
}
impl IpV4 {
    pub fn new(ip:String) -> IpV4 {
        let mut parts: std::str::Split<'_, &str> = ip.split(".");
        let part_1: u8 = parts.next().unwrap().parse().unwrap();
        let part_2: u8 = parts.next().unwrap().parse().unwrap();
        let part_3: u8 = parts.next().unwrap().parse().unwrap();
        let part_4: u8 = parts.next().unwrap().parse().unwrap();
        IpV4 {
            ip,
            part_1,
            part_2,
            part_3,
            part_4
        }
    }
    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

    pub fn get_parts(&self) -> (u8,u8,u8,u8) {
        (self.part_1,self.part_2,self.part_3,self.part_4)
    }
    pub fn get_part_1(&self) -> u8 {
        self.part_1
    }
    pub fn get_part_2(&self) -> u8 {
        self.part_2
    }
    pub fn get_part_3(&self) -> u8 {
        self.part_3
    }
    pub fn get_part_4(&self) -> u8 {
        self.part_4
    }
    pub fn set_ip(&mut self,ip:String) {
        self.ip = ip;
    }
    pub fn set_parts(&mut self,part_1:u8,part_2:u8,part_3:u8,part_4:u8) {
        self.part_1 = part_1;
        self.part_2 = part_2;
        self.part_3 = part_3;
        self.part_4 = part_4;
    }
    pub fn set_part_1(&mut self,part_1:u8) {
        self.part_1 = part_1;
    }
    pub fn set_part_2(&mut self,part_2:u8) {
        self.part_2 = part_2;
    }
    pub fn set_part_3(&mut self,part_3:u8) {
        self.part_3 = part_3;
    }
    pub fn set_part_4(&mut self,part_4:u8) {
        self.part_4 = part_4;
    }
    pub fn is_local(&self) -> bool {
        if self.part_1 == 192 && self.part_2 == 168 {
            return true;
        }
        if self.part_1 == 10 {
            return true;
        }
        if self.part_1 == 172 && self.part_2 >= 16 && self.part_2 <= 31 {
            return true;
        }
        return false;
    }
}
pub fn is_valid_port(port:u16) -> bool {
    if port > 0 && port < 65536 {
        return true;
    }
    return false;
}
