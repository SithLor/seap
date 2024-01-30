package src;

public class Main  {
    
    
    static java.util.ArrayList<String> ScannerTokensToArray(java.util.Scanner arg) {
        java.util.ArrayList<String> ar = new java.util.ArrayList<String>();
        String Data = arg.next().toString();
        while(arg.hasNext()){
            ar.add(arg.next());
        }

        arg.close();
        return ar;
    }
    public static void main(String[] args){
        src.rust e = new rust();
        System.out.println(src.rust.format("{} hi {}", {src.cmd.util.BLUE,src.cmd.util.RESET}));
    }
}