package src;

import java.io.*;
import java.nio.file.*;
import java.util.*;

public class main {
    private static final String OUsrcTPUT_FILE = "./output.txt";
    private static final String INPUT_FILE = "./input.m.txt";
    private static final String OUTPUT_FILE = "./output.txt";;

    public static void main(String[] args) throws IOException {
        long start = System.currentTimeMillis();
        code();
        System.out.println("Time: " + (System.currentTimeMillis() - start) + "ms");
    }

    private static void code() throws IOException {
        List<Integer> grades = new ArrayList<>();
        Path path = Paths.get(INPUT_FILE);
        List<String> lines = Files.readAllLines(path);

        for (String line : lines) {
            String[] parts = line.split("\\s+");
            grades.add(Integer.parseInt(parts[1]));
        }

        double avg = grades.stream().mapToInt(Integer::intValue).average().orElse(0.0);
        System.out.println("Average: " + avg);

        int[] rowsCols = minRowsCols(grades.size());
        int rows = rowsCols[0];
        int cols = rowsCols[1];

        BufferedWriter output = new BufferedWriter(new FileWriter(OUTPUT_FILE));
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                int index = i * cols + j;
                if (index < grades.size()) {
                    output.write(grades.get(index) + " ");
                }
            }
            output.newLine();
        }
        output.close();
    }

    private static int[] minRowsCols(int amount) {
        double sqrt = Math.sqrt(amount);
        int rows = (int) Math.ceil(sqrt);
        int cols = rows * (rows - 1) >= amount ? rows - 1 : rows;
        return new int[]{rows, cols};
    }
}