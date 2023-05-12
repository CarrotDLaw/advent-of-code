import java.io.*;
import java.util.*;

public class AoC2016Day03 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static List<List<Integer>> numList = new ArrayList<>();

  public static void listSetup() {
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2016Day03.txt")))) {
      while (fileSc.hasNextLine()) {
        String[] threeNumStrArr = fileSc.nextLine().trim().replaceAll(" +", " ").split(" ");
        // deal with empty str
        if (threeNumStrArr[0].equals("")) {
          break;
        }
        int side1 = Integer.parseInt(threeNumStrArr[0]);
        int side2 = Integer.parseInt(threeNumStrArr[1]);
        int side3 = Integer.parseInt(threeNumStrArr[2]);
        numList.add(Arrays.asList(side1, side2, side3));
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
  }

  public static void day03() {
    int validTriangle1 = 0;
    int validTriangle2 = 0;
    for (int i = 0; i < 3; i++) {
      for (int j = 0; j < (numList.size() - 2); j += 3) {
        int[] triangle = new int[3];
        for (int k = 0; k < 3; k++) {
          triangle[k] = numList.get(j + k).get(i);
        }
        Arrays.sort(triangle);
        System.out.println(Arrays.toString(triangle));
        if (triangle[0] + triangle[1] > triangle[2]) {
          validTriangle2++;
        }
      }
    }
    System.out.println(numList);
    for (List<Integer> triangle : numList) {
      Collections.sort(triangle);
      if (triangle.get(0) + triangle.get(1) > triangle.get(2)) {
        validTriangle1++;
      }
    }
    System.out.println(validTriangle1);
    System.out.println(validTriangle2);
  }

  public static void main(String[] args) {
    listSetup();
    day03();
  }
}
