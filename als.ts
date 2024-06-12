namespace Debug {
    export interface DebugInfo {
        time_created: string;
        id: string;
    }
    export function CreateDebugObject(): DebugInfo {
        return {
            time_created: new Date().toISOString(),
            id: Math.random().toString(36).substring(2, 15)
        };
    }
}

namespace Node {
    
    export interface EmptyNode {
        type:"empty";
    }

}

namespace C_Preprossor {



}
// define marco solver




console.log(DefineString_to_DefineAstNode("#define e 1/2+3"));
console.log(DefineString_to_DefineAstNode("#define e(x) x+3"));
console.log(DefineString_to_DefineAstNode("#define e(x,y) x+y"));