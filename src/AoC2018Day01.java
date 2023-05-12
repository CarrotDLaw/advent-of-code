import java.io.*;
import java.util.*;

public class AoC2018Day01 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static List<Integer> frequencyList = new ArrayList<>();

  public static void listSetup() {
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2018Day01.txt")))) {
      while (fileSc.hasNextLine()) {
        String str = fileSc.nextLine();
        frequencyList.add(Integer.parseInt(str));
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
  }

  public static void day01() {
    Set<Integer> frequencyCheckList = new HashSet<>();
    int sum1 = 0;
    int sum2 = 0;
    for (int i : frequencyList) { // part 1
      sum1 += i;
    }
    System.out.println(sum1);
    for (int j = 0;; j++) { // part 2
      int i = frequencyList.get(j);
      sum2 += i;
      if (frequencyCheckList.contains(sum2)) {
        System.out.println(sum2);
        return;
      }
      frequencyCheckList.add(sum2);
      if (j + 1 == frequencyList.size()) {
        frequencyList.addAll(frequencyList);
      }
    }
  }

  public static void main(String[] args) {
    listSetup();
    day01();
  }
}
