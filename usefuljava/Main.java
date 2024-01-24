
class Main {

    public static java.util.ArrayList<String> ScannerTokensToArray(java.util.Scanner arg) {
        java.util.ArrayList<String> ar = new ArrayList<String>();
        while(arg.hasNext()){
            ar.add(arg.next().toString());
        }
        //this close the Scanner Perforce I hate java
        arg.close();
        System.gc();
        return ar;
    }

    public static void main(){
        java.util.Scanner scanner = new java.util.Scanner("1 1.0 -1 lol");
        java.util.ArrayList e = ScannerTokensToArray(scanner);
        for (int i = 0; i < e.size(); i++) {
            System.out.print(e.get(i));
        }
    }
}