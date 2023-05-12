import java.util.*;
import java.io.*;

public class AoC2020Day01 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day01() {
    List<Integer> entryList = new ArrayList<>();
    boolean equals2020 = false;
    int product1 = 0;
    int product2 = 0;
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2020Day01.txt")))) {
      while (fileSc.hasNextLine()) {
        entryList.add(Integer.parseInt(fileSc.nextLine()));
      }
    } catch (FileNotFoundException e) {
      System.out.println("An error occurred.");
      e.printStackTrace();
    }
    for (int i = 0; i < entryList.size(); i++) {
      for (int j = i + 1; j < entryList.size(); j++) {
        for (int k = j + 1; k < entryList.size(); k++) {
          equals2020 =
              (entryList.get(i) + entryList.get(j) + entryList.get(k) == 2020) ? true : false;
          if (equals2020) {
            product2 = entryList.get(i) * entryList.get(j) * entryList.get(k); // part 2
            equals2020 = false;
          }
        }
        equals2020 = (entryList.get(i) + entryList.get(j) == 2020) ? true : false;
        if (equals2020) {
          product1 = entryList.get(i) * entryList.get(j); // part 1
          equals2020 = false;
        }
      }
    }
    System.out.println(product1);
    System.out.println(product2);
  }

  public static void main(String[] args) {
    day01();
  }
}
