import java.io.*;
import java.util.*;

public class AoC2016Day01 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static List<Character> turningList = new ArrayList<>();
  static List<Integer> stepList = new ArrayList<>();
  static Set<List<Integer>> footprints = new HashSet<List<Integer>>();
  static int needle = 0;

  public AoC2016Day01() {}

  public static void listSetup() {
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2016Day01.txt")))) {
      String[] stepStrArray = fileSc.nextLine().split(", ");
      for (String str : stepStrArray) {
        turningList.add(str.charAt(0));
        stepList.add(Integer.parseInt(str.substring(1)));
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
  }

  public static void turnLeft() {
    needle = (needle - 1 + 4) % 4;
  }

  public static void turnRight() {
    needle = (needle + 1) % 4;
  }

  public static void turn(char dir) {
    if (dir == 'L') {
      turnLeft();
    } else if (dir == 'R') {
      turnRight();
    }
  }

  public static void day01() {
    int[] compass = {+1, +1, -1, -1};
    int x = 0, y = 0, x0 = 0, y0 = 0;
    int firstDup = 0;
    boolean dupFound = false;
    for (int i = 0; i < turningList.size(); i++) {
      int steps = stepList.get(i);
      turn(turningList.get(i));
      if (needle == 0 || needle == 2) {
        y += steps * compass[needle];
        for (int j = 1; j <= steps && !dupFound; j++) {
          if (!(footprints.add(new ArrayList<>(List.of(x, y0 + j * compass[needle]))))) {
            firstDup = Math.abs(x) + Math.abs(y0 + j * compass[needle]);
            dupFound = true;
          }
        }
      } else if (needle == 1 || needle == 3) {
        x += steps * compass[needle];
        for (int j = 1; j <= steps && !dupFound; j++) {
          if (!(footprints.add(new ArrayList<>(List.of(x0 + j * compass[needle], y))))) {
            firstDup = Math.abs(x0 + j * compass[needle]) + Math.abs(y);
            dupFound = true;
          }
        }
      }
      x0 = x;
      y0 = y;
    }
    System.out.println(Math.abs(x) + Math.abs(y)); // part 1
    System.out.println(firstDup); // part 2
  }

  public static void main(String[] args) {
    listSetup();
    day01();
  }
}
