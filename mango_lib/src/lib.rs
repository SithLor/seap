#[macro_export]
macro_rules! called_from {
    () => {
        format!("/{}/{}:{}",file!(),line!(),column!())
    };
}
//MOD EXPORTS
pub mod usd;
pub mod excel;
pub mod ip;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip(){
        let mut ip = ip::IpV4::new("10.0.0.1".to_string());
        //test the get and set functions
        assert_eq!(ip.get_ip(),"10.0.0.1");
        ip.set_ip("1.1.1.1".to_string());
        assert_eq!(ip.get_ip(),"1.1.1.1");
        ip.set_parts(1,2,3,4);
        assert_eq!(ip.get_parts(),(1,2,3,4));
        ip.set_part_1(5);
        assert_eq!(ip.get_part_1(),5);
        ip.set_part_2(6);
        assert_eq!(ip.get_part_2(),6);
        ip.set_part_3(7);
        assert_eq!(ip.get_part_3(),7);
        ip.set_part_4(8);
        assert_eq!(ip.get_part_4(),8);
        //test the is_local function
        ip.set_parts(192,168,1,1);
        assert_eq!(ip.is_local(),true);
        ip.set_parts(10,0,0,1);
        assert_eq!(ip.is_local(),true);
        ip.set_parts(172,16,1,1);
        assert_eq!(ip.is_local(),true);
        ip.set_parts(172,31,1,1);
        assert_eq!(ip.is_local(),true);
        ip.set_parts(172,15,1,1);
        assert_eq!(ip.is_local(),false);
        ip.set_parts(172,32,1,1);
        assert_eq!(ip.is_local(),false);
        ip.set_parts(172,0,1,1);
    }
    #[test]
    fn test_excel(){

        
         
    }
}
