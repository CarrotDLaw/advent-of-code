import java.io.*;
import java.util.*;

public class AoC2021Day02 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day02() {
    List<String> commandList = new ArrayList<>();
    int depth = 0;
    int distance = 0;
    int aim = 0;
    String listDir = inputFolderDir.concat("AoC2021Day02.txt");
    try (BufferedReader command = new BufferedReader(new FileReader(listDir))) {
      String cmd = command.readLine();
      while (cmd != null) {
        commandList.add(cmd);
        cmd = command.readLine();
      }
      for (String str : commandList) { // part 1
        if (str.charAt(0) == 'u') {
          depth -= Character.getNumericValue(str.charAt(str.length() - 1));
        } else if (str.charAt(0) == 'd') {
          depth += Character.getNumericValue(str.charAt(str.length() - 1));
        } else {
          distance += Character.getNumericValue(str.charAt(str.length() - 1));
        }
      }
      System.out.println(distance * depth);
      depth = 0;
      distance = 0;
      aim = 0;
      for (String str : commandList) { // part 2
        if (str.charAt(0) == 'u') {
          aim -= Character.getNumericValue(str.charAt(str.length() - 1));
        } else if (str.charAt(0) == 'd') {
          aim += Character.getNumericValue(str.charAt(str.length() - 1));
        } else {
          distance += Character.getNumericValue(str.charAt(str.length() - 1));
          depth += aim * Character.getNumericValue(str.charAt(str.length() - 1));
        }
      }
      System.out.println(distance * depth);
    } catch (IOException e) {
      e.printStackTrace();
    }
  }

  public static void main(String[] args) {
    day02();
  }
}
