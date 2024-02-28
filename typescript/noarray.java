import java.io.*;
import java.nio.file.*;
import java.util.*;
//gavin rhoades 2/28/2024 apc121 
public class noarray {
    private static final String INPUT_FILE = "./people.txt";
    private static final String OUTPUT_FILE = "./seating_chart.txt";
    public static void uw_code(){
        Scanner keyboard = new Scanner(System.in);
        System.out.println("How many rows to you want? " );
        int row = keyboard.nextInt();
        System.out.println("How many columns to you want? " );
        int col = keyboard.nextInt();
        String[][] seating = new String[row][col];
        
        for (int i = 0; i < row; i++){
            for (int j = 0; j < col; j++){
                System.out.println("Enter the name of the person for row " + (i+1) + " and column " + (j+1) + ": ");
                seating[i][j] = keyboard.next();
            }
        }
        
    }

    public static void main(String[] args) throws IOException {
        Scanner keyboard = new Scanner(System.in);
        System.out.println("How many rows to you want? " );
        int row = keyboard.nextInt();
        System.out.println("How many columns to you want? " );
        int col = keyboard.nextInt();
        String[][] seating = new String[row][col];
        
        seating = enterStudents(keyboard, row, col);
        printStudents(seating);
    }
    public static String[][] enterStudents(Scanner keyboard,int row,int col){
        String[][] seating = new String[row][col];
        for (int i = 0; i < row; i++){
            for (int j = 0; j < col; j++){
                System.out.println("Enter the name of the person for row " + (i+1) + " and column " + (j+1) + ": ");
                seating[i][j] = keyboard.next();
            }
        }
        return seating;
    }
    public static String[][] clearStudents(int r,int c){
        String[][] e = new String[r][c];
        return e;
    }
    public static void printStudents(String[][] stud){
        for (String[] row : stud) {
            for (String student : row) {
                System.out.print(student + "\t");
            }
            System.out.println();
        }
    }
    public static changeStudent(String[][] arg){
        //
    }
}