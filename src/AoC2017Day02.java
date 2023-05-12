import java.io.*;
import java.util.*;

public class AoC2017Day02 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day02() {
    List<List<Integer>> spreadsheet = new ArrayList<List<Integer>>();
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2017Day02.txt")))) {
      for (int rowNum = 0; fileSc.hasNextLine(); rowNum++) {
        spreadsheet.add(new ArrayList<>());
        String[] row = fileSc.nextLine().split("	");
        for (String str : row) {
          spreadsheet.get(rowNum).add(Integer.parseInt(str));
        }
        Collections.sort(spreadsheet.get(rowNum));
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    int checksum1 = 0;
    int checksum2 = 0;
    for (int rowNum = 0; rowNum < spreadsheet.size(); rowNum++) {
      checksum1 += spreadsheet.get(rowNum).get(spreadsheet.get(rowNum).size() - 1)
          - spreadsheet.get(rowNum).get(0);
    }
    System.out.println(checksum1);
    for (int rowNum = 0; rowNum < spreadsheet.size(); rowNum++) {
      for (float dividend : spreadsheet.get(rowNum)) {
        for (float divisor : spreadsheet.get(rowNum)) {
          if (dividend == divisor) {
            continue;
          }
          checksum2 += dividend % divisor == 0 ? dividend / divisor : 0;
        }
      }
    }
    System.out.println(checksum2);
  }

  public static void main(String[] args) {
    day02();
  }
}
