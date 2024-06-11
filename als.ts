//Als Complier framework


export function version(): string {
    return "0.1.1"
}

// define marco solver
interface DefineAstNode {
    expr:string
    name:string
    is_func:string
}
function DefineString_to_DefineAstNode(og_expr:string){
    //spilt
    //#define e 1+2/3 -> 

    const debug_1 = og_expr.charAt(0);
    const debug_2 = og_expr.substring(1,6);
    const debug_3 = og_expr.substring(0,6);
    console.log(debug_1,debug_2,debug_3)
    if (og_expr.charAt(0) != "#" == true ||
        og_expr.substring(1,6) != "define"||
        og_expr.substring(0,6) != "#define"
    ){
        const name_and_expr = og_expr.substring(7,og_expr.length)
        console.log(name_and_expr)
    }else {
        return "no"
    }
}