

public class Main {
    
    public static java.util.ArrayList<String> ScannerTokensToArray(java.util.Scanner arg) {
        java.util.ArrayList<String> ar = new java.util.ArrayList<String>();
        while(arg.hasNext()){
            ar.add(arg.next());
        }
        arg.close();
        return ar;
    }
    public static void main(String[] args){
        java.util.Scanner scanner = new java.util.Scanner("1 1.0 -1 lol");
        java.util.ArrayList<String> e = ScannerTokensToArray(scanner);
        java.util.StringBuilder sb = new java.util.StringBuilder();
        for (String s : e) {
            sb.append(s).append(" ");
        }
        System.out.println(sb.toString());
    }
}