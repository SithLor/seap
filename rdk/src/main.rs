static VERSION: &str = "0.1.0";

mod rdk {
    pub mod Objects {
        pub trait ObjectToString {
            fn to_string(&self) -> String;
        }
        pub trait ObjectNew<T> {
            fn new(value: T) -> Object<T>;
        }
        pub trait ObjectGetValue<T> {
            fn get_value(&self) -> &T;
        }
        pub trait ObjectSetValue<T> {
            fn set_value(&mut self, value: T);
        }
        pub struct Object<T> {
            pub value: T,
        }
        impl<T: std::fmt::Debug> ObjectToString for Object<T> {
            fn to_string(&self) -> String {
                format!("{:?}", self.value)
            }
        }
        impl<T> ObjectNew<T> for Object<T> {
            fn new(value: T) -> Object<T> {
                Object { value }
            }
        }

        impl<T> ObjectGetValue<T> for Object<T> {
            fn get_value(&self) -> &T {
                &self.value
            }
        }
        impl<T> ObjectSetValue<T> for Object<T> {
            fn set_value(&mut self, value: T) {
                self.value = value;
            }
        }
    }
}

fn main() {
    use crate::rdk::Objects::Object;
    use crate::rdk::Objects::ObjectNew;
    use crate::rdk::Objects::ObjectToString;
    use crate::rdk::Objects::ObjectGetValue;
    use crate::rdk::Objects::ObjectSetValue;

    let mut obj: Object<i32> = Object::new(10);


    println!("Object: {}", obj.to_string());
    println!("Object value: {}", obj.get_value());
    obj.set_value(20);
    println!("Object value: {}", obj.get_value());
}
