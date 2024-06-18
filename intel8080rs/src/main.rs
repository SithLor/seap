
//gen_getter_and_setter!(AccumulatorRegister, eax, ax, al);
macro_rules! near_limit {
    ($val:expr) => {
        match std::mem::size_of_val(&$val) {
            1 => ($val as i8) >= i8::MAX - 1,
            2 => ($val as i16) >= i16::MAX - 1,
            4 => ($val as i32) >= i32::MAX - 1,
            8 => ($val as i64) >= i64::MAX - 1,
            _ => panic!("Unsupported type"),
        }
    };
}

fn main() {

    //
    let instructions = ["mov","add","sub","mul","div","inc","dec","cmp","jmp","je","jne","jg","jge","jl","jle","call","ret","push","pop","lea","nop","hlt","int","iret","cli","sti","cld","std","clc","stc","sar","shl","shr","rol","ror","rcl","rcr","and","or","xor","not","neg","test","set","movzx","movsx","xchg","bswap","bsf","bsr","bt","btc","btr","bts","popf","pushf","lahf","sahf","xlat","in","out","ins","outs","rep","repe","repne","repnz","repz","lock","stos","lods","scas","cmps","movs","enter","leave","loop","loopz","loopnz","loopne","loopnz","loopnz","loopne","loopnz"];
    

}
