//shit vm for intel custom i386 with fancy pipeline modern cpu feature expect for branch predtion beascue it allready run a cpu with branch predtion
//https://en.wikipedia.org/wiki/I386

//Pipeline:
//extention:aes,possable support for proteched mode
///info
/// Data width 32 bits
/// Address 32 bits
/// Pipeline
/// 6 stage instruction
mod i386 {
    struct AccumulatorRegister {
        pub eax: i32,
        pub ax: i16,
        pub al: i8,
    }
    impl AccumulatorRegister {
        fn new() -> AccumulatorRegister {
            return AccumulatorRegister {
                eax: 0,
                ax: 0,
                al: 0,
            };
        }
    }
}

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
//https://www.ardent-tool.com/CPU/docs/Intel/387/datasheets/240225-009.pdf
pub mod _80387SX_ {
    struct Processor_input {
        BUSY: bool,

        //cpu from main cpu
        CPU_ip: u32,
        CPU_dp: u32,
    }
    struct Processor_output {}

    struct Processor {
        input: Processor_input,
        output: Processor_output,
    }
    impl Processor {
        fn new() -> Processor {
            return Processor {
                input: Processor_input { BUSY: false },
                output: Processor_output {},
            };
        }
    }
    enum Register {
        // Status Register 16 bits 
        status,
        // Control Register 16 bits
        control,
        tag_word

        // (R0 –R7)
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
    }
    enum InstructionSet {
        /// ArithmeticInstructions
        
        // Addition
        FADD, // Add Real
        FADDP, // Add Real and pop
        FIADD, // Add Integer
        // Subtraction
        FSUB,   // Subtract Real
        FSUBP,  // Subtract Real and pop
        FISUB,  // Subtract Integer
        FSUBR,  // Subtract Real reversed
        FSUBRP, // Subtract Real reversed and pop
        FISUBR, // Subtract Integer reversed
        // Multiplication
        FMUL,  // Multiply Real
        FMULP, // Multiply Real and pop
        FIMUL, // Multiply Integer
        // Division
        FDIV,   // Divide Real
        FDIVP,  // Divide Real and pop
        FIDIV,  // Divide Integer
        FDIVR,  // Divide Real reversed
        FDIVRP, // Divide Real reversed and pop

        /// DataTransfer
        
        FILD, // Load (convert from) Integer (word, short, long)
        FIST,  // Store (convert to) Integer (word, short)
        FISTP, // Store (convert to) Integer and pop (word, short, long)
        // Packed Decimal Transfers
        FBLD,  // Load (convert from) packed decimal
        FBSTP, // Store packed decimal and pop

        /// ProcessorInstructionsAdministrative
        
        FINIT, // Initialize Math CoProcessor
        FLDCW,    // Load Control Word and Load Status Word
        FSTCW,    // Store Control Word
        FSTSW,    // Store Status Word
        FSTSW_AX, // Store Status Word to AX register
        FCLEX,    // Clear Exceptions
        FSTENV,   // Store Environment
        FLDENV,   // Load Environment
        FSAVE,    // Save State
        FRSTOR,   // Restore State
        FINCSTP,  // Increment Stack pointer
        FDECSTP,  // Decrement Stack pointer
        FFREE,    // Free Register
        FNOP,     // No Operation
        FWAIT,    // Report Math CoProcessor Error
    }


}

fn main() {
    //80387SX  math co processor
    let chicken_bit_fpu = 0;
    //
    enum DataTransferInstructions {
        FILD,  // Load (convert from) Integer (word, short, long)
        FIST,  // Store (convert to) Integer (word, short)
        FISTP, // Store (convert to) Integer and pop (word, short, long)

        // Packed Decimal Transfers
        FBLD,  // Load (convert from) packed decimal
        FBSTP, // Store packed decimal and pop
    }
    enum ArithmeticInstruction {
        // Addition
        FADD,  // Add Real
        FADDP, // Add Real and pop
        FIADD, // Add Integer
        // Subtraction
        FSUB,   // Subtract Real
        FSUBP,  // Subtract Real and pop
        FISUB,  // Subtract Integer
        FSUBR,  // Subtract Real reversed
        FSUBRP, // Subtract Real reversed and pop
        FISUBR, // Subtract Integer reversed
        // Multiplication
        FMUL,  // Multiply Real
        FMULP, // Multiply Real and pop
        FIMUL, // Multiply Integer
        // Division
        FDIV,   // Divide Real
        FDIVP,  // Divide Real and pop
        FIDIV,  // Divide Integer
        FDIVR,  // Divide Real reversed
        FDIVRP, // Divide Real reversed and pop
    }
}
