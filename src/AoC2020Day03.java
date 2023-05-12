import java.util.*;
import java.io.*;

public class AoC2020Day03 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day03() {
    List<String> map = new ArrayList<>();
    int[][] slope = new int[][] {{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}};
    int hPosition = 0, vPosition = 0;
    long tree = 1l;
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2020Day03.txt")))) {
      while (fileSc.hasNextLine()) {
        map.add(fileSc.nextLine());
      }
    } catch (FileNotFoundException e) {
      System.out.println("An error occurred.");
      e.printStackTrace();
    }
    for (int i = 0; i < slope.length; i++) {
      int treePerSlope = 0;
      hPosition = 0;
      vPosition = 0;
      for (; vPosition < map.size(); hPosition += slope[i][0], vPosition += slope[i][1]) {
        while (hPosition >= map.get(vPosition).length()) {
          map.set(vPosition, map.get(vPosition).repeat(2));
        }
        treePerSlope += (map.get(vPosition).charAt(hPosition) == '#') ? 1 : 0;
      }
      System.out.printf("%s: %d" + System.lineSeparator(), Arrays.toString(slope[i]), treePerSlope);
      tree *= treePerSlope;
    }
    System.out.printf("Total: %d" + System.lineSeparator(), tree); // part 1 [3,1]; part 2 Total
  }

  public static void main(String[] args) {
    day03();
  }
}
