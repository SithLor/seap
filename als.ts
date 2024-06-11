//Als Complier framework


export function version(): string {
    return "0.1.1"
}

// define marco solver
interface DefineAstNode {
    expr:string
    name:string
    is_func:string

    debug:{
        debug_1:string,
        debug_2:string,
        debug_3:string,
    }
}


function DefineString_to_DefineAstNode(og_expr:string){
    //spilt
    //#define e 1+2/3 -> 

    const debug_1 = og_expr.charAt(0);
    const debug_2 = og_expr.substring(1,7);
    const debug_3 = og_expr.substring(0,7);
    console.log(debug_1,debug_2,debug_3)
    if (og_expr.charAt(0) != "#" == true ||
        og_expr.substring(0,6) != "#define"
    ){
        const name_and_expr = og_expr.substring(8,og_expr.length)
        const name = name_and_expr.split(" ");
        
        console.log(name_and_expr,name)
    }else {
        return "no"
    }
}
DefineString_to_DefineAstNode("#define e 1/2+3")