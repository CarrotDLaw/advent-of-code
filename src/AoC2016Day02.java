import java.io.*;
import java.util.*;

public class AoC2016Day02 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static int[][] keypad1 = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
  static char[][] keypad2 = {{'0', '0', '1', '0', '0'}, {'0', '2', '3', '4', '0'},
      {'5', '6', '7', '8', '9'}, {'0', 'A', 'B', 'C', '0'}, {'0', '0', 'D', '0', '0'}};
  static List<String> instructionList = new ArrayList<>();

  public AoC2016Day02() {}

  public static void day02() {
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2016Day02.txt")))) {
      while (fileSc.hasNextLine()) {
        instructionList.add(fileSc.nextLine());
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    int row1 = 1, column1 = 1;
    int row2 = 2, column2 = 0;
    int bathroomCode1 = 0;
    String bathroomCode2 = "";
    for (String insStr : instructionList) {
      for (char i : insStr.toCharArray()) {
        if (i == 'U') {
          row1--;
        } else if (i == 'D') {
          row1++;
        } else if (i == 'L') {
          column1--;
        } else if (i == 'R') {
          column1++;
        }
        if (row1 < 0 || row1 > 2) {
          row1 = (row1 + 1) / 2;
        } else if (column1 < 0 || column1 > 2) {
          column1 = (column1 + 1) / 2;
        }
        int rowTemp = row2, columnTemp = column2;
        if (i == 'U') {
          rowTemp--;
        } else if (i == 'D') {
          rowTemp++;
        } else if (i == 'L') {
          columnTemp--;
        } else if (i == 'R') {
          columnTemp++;
        }
        if (rowTemp >= 0 && rowTemp <= 4 && columnTemp >= 0 && columnTemp <= 4
            && keypad2[rowTemp][columnTemp] != '0') {
          row2 = rowTemp;
          column2 = columnTemp;
        }
      }
      bathroomCode1 = bathroomCode1 * 10 + keypad1[row1][column1]; // part 1
      bathroomCode2 += keypad2[row2][column2]; // part 2
    }
    System.out.println(bathroomCode1);
    System.out.println(bathroomCode2);
  }

  public static void main(String[] args) {
    day02();
  }
}
