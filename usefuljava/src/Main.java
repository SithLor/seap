package src;

import src.cmd.util;

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
        
        System.out.println(src.cmd.util.BLUE+Emoji.MERICA+"  This Is a test"+src.cmd.util.RESET);
    }
}