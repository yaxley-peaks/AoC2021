package day_1;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.Vector;

/**
 * Day1
 */
public class Day1 {

    public static void main(String[] args) throws FileNotFoundException {
        File file = new File("D:\\Documents\\AoC2021\\day_1\\input.txt");
        // File file = new File("input.txt");
        Scanner filScanner = new Scanner(file);
        Vector<Integer> input = new Vector<Integer>();
        
        int sum = 0;
        while (filScanner.hasNextLine()) {
            String line = filScanner.nextLine();
            input.add(Integer.parseInt(line));
        }
        for(int i = 1; i < input.size(); i++){
            if(input.get(i) > input.get(i-1)){
                sum++;
            }
        }
        System.out.println(sum);
        filScanner.close();
    }
}