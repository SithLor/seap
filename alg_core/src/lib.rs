
//shity shader functions for cpus

pub mod shader {
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    
    impl Vec3 {
        pub fn length(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
    
        // HLSL normalize function
        pub fn normalize_hlsl(&self) -> Vec3 {
            let len = self.length();
            if len > 0.0 {
                Vec3 {
                    x: self.x / len,
                    y: self.y / len,
                    z: self.z / len,
                }
            } else {
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        }
    
        // GLSL normalize function
        pub fn normalize_glsl(&self) -> Vec3 {
            let len = self.length();
            if len != 0.0 {
                Vec3 {
                    x: self.x / len,
                    y: self.y / len,
                    z: self.z / len,
                }
            } else {
                panic!("Cannot normalize a zero vector in GLSL");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use shader::Vec3;

    use super::*;

    #[test]
fn test_normalize() {
    let v = Vec3 { x: 3.0, y: 4.0, z: 0.0 };

    let n_hlsl = v.normalize_hlsl();
    assert_eq!(n_hlsl.x, 0.6);
    assert_eq!(n_hlsl.y, 0.8);
    assert_eq!(n_hlsl.z, 0.0);

    let n_glsl = v.normalize_glsl();
    assert_eq!(n_glsl.x, 0.6);
    assert_eq!(n_glsl.y, 0.8);
    assert_eq!(n_glsl.z, 0.0);
}

#[test]
#[should_panic(expected = "Cannot normalize a zero vector in GLSL")]
fn test_normalize_zero_glsl() {
    let v = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    v.normalize_glsl();
}

#[test]
fn test_normalize_zero_hlsl() {
    let v = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let n_hlsl = v.normalize_hlsl();
    assert_eq!(n_hlsl.x, 0.0);
    assert_eq!(n_hlsl.y, 0.0);
    assert_eq!(n_hlsl.z, 0.0);
}
}