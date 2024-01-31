package src;

import src.Exception.StringLengthLessThanOne;

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
    static 
    public static void main(String[] args) throws StringLengthLessThanOne{

        Runnable runnable = new Runnable() {
            @Override
            public void run() {
                // Code to be executed in the new thread
            }
        };

        Thread thread = new Thread(runnable);
        thread.start();
    }
}