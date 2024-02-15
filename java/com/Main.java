import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.IOException;

class Main {
   public static void main(String[] args){
      System.out.println("w,a,s,d move around , x close it,c for clear");
      game();
   }

   public static void game(){
       BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
       Turtle t = new Turtle(0,0);

       do {
         String key = null;
         try {
            key = reader.readLine();
         } catch (IOException e) {
            e.printStackTrace();
         }

         switch(key){
            case "w":
               t.forward(10);
               break;
            case "s":
               t.backward(10);
               break;
            case "a":
               t.right(90);
               break;
            case "d":
               t.left(90);
               break;
            case "c":
               t.clear();
               break;
            case "x":
               t.clear();
               System.exit(1);
               break;
         }
       } while (true);
   }
}