package day_1.others;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.Vector;

public class Day1 {

    public static int part1(Vector<Integer> input) {
        int sum = 0;
        for (int i = 1; i < input.size(); i++) {
            if (input.get(i) > input.get(i - 1)) {
                sum++;
            }
        }
        return sum;
    }

    public static int part2(Vector<Integer> input){
        int sum = 0;
        for(int i = 3; i < input.size(); i++){
            if(input.get(i) > input.get(i-3)){
                sum++;
            }}
        return sum;
        
    }

    public static void main(String[] args) throws FileNotFoundException {
        File file = new File("D:\\Documents\\AoC2021\\day_1\\input.txt");
        // File file = new File("input.txt"); // why this no work??
        Scanner filScanner = new Scanner(file);
        Vector<Integer> input = new Vector<Integer>();

        while (filScanner.hasNextLine()) {
            String line = filScanner.nextLine();
            input.add(Integer.parseInt(line));
        }
        filScanner.close();

        System.out.println(part1(input));
        System.out.println(part2(input));
    }
}