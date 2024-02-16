use std::result;

pub fn bmi(weight: f64, height: f64) -> f64 {
    let w: f64 = weight;
    let h: f64 = height;
    w / (h*h)*703.0
}

pub fn perimeter_of_rectangle(l: f64, w: f64) -> f64 {
    let result: f64;
    //platfrom if it is MODERN x64 
    if cfg!(target_arch = "x86_64") {
        unsafe {
            std::arch::asm!(
                "
                movsd qword ptr [rsp - 8], xmm1
                movaps xmm1, xmm0
                movsd xmm0, qword ptr [rsp - 8]
                addsd xmm1, xmm0
                movsd xmm0, qword ptr [rip + .LCPI0_0]
                mulsd xmm0, xmm1
                ",
                in("xmm0") l,
                in("xmm1") w,
                out("xmm2") result,
                options(nostack, nomem, preserves_flags)
            );
        }
    } else {
        result = 2.0 * (l + w);
    };
    return result;
}

pub fn time() -> String {
    //get the data MM/DD/YYYY/HH:MM:SS
    let data_MM_DD_YYYY_HH_MM_SS: String = chrono::Local::now().format("%m/%d/%Y/%H:%M:%S").to_string();
}