import java.io.*;
import java.util.*;

public class AoC2020Day02 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day02() {
    List<Integer> minList = new ArrayList<>();
    List<Integer> maxList = new ArrayList<>();
    List<Character> letterList = new ArrayList<>();
    List<String> pwList = new ArrayList<>();
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2020Day02.txt")))) {
      while (fileSc.hasNextLine()) {
        String[] str = fileSc.nextLine().split(" ");
        minList.add(Integer.parseInt(str[0].split("-")[0]));
        maxList.add(Integer.parseInt(str[0].split("-")[1]));
        letterList.add(str[1].charAt(0));
        pwList.add(str[2]);
      }
    } catch (FileNotFoundException e) {
      System.out.println("An error occurred.");
      e.printStackTrace();
    }
    int validPW1 = 0;
    int validPW2 = 0;
    for (int i = 0; i < pwList.size(); i++) { // part 1
      int count = 0;
      for (int j = 0; j < pwList.get(i).length(); j++) {
        count += (pwList.get(i).charAt(j) == letterList.get(i)) ? 1 : 0;
      }
      validPW1 += (count >= minList.get(i) && count <= maxList.get(i)) ? 1 : 0;
    }
    System.out.println(validPW1);
    for (int i = 0; i < pwList.size(); i++) { // part 2
      validPW2 += ((pwList.get(i).charAt(minList.get(i) - 1) == letterList.get(i)
          && pwList.get(i).charAt(maxList.get(i) - 1) != letterList.get(i))
          || (pwList.get(i).charAt(minList.get(i) - 1) != letterList.get(i)
              && pwList.get(i).charAt(maxList.get(i) - 1) == letterList.get(i))) ? 1 : 0;
    }
    System.out.println(validPW2);
  }

  public static void main (String[] args) {
    day02();
  }
}
