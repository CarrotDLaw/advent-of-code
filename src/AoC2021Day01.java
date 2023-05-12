import java.util.*;
import java.io.*;

public class AoC2021Day01 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public AoC2021Day01() {}

  public static void day01() {
    List<Integer> sonarList = new ArrayList<>();
    String listDir = inputFolderDir.concat("AoC2021Day01.txt");
    try (BufferedReader sonarReport = new BufferedReader(new FileReader(listDir))) {
      String sonarString = sonarReport.readLine();
      while (sonarString != null) {
        sonarList.add(Integer.parseInt(sonarString));
        sonarString = sonarReport.readLine();
      }
    } catch (IOException e) {
      e.printStackTrace();
    }
    int sum1 = 0;
    int sum2 = 0;
    for (int i = 0; i + 1 < sonarList.size(); i++) { // part 1
      sum1 = (sonarList.get(i + 1) > sonarList.get(i)) ? sum1 + 1 : sum1;
    }
    for (int i = 0; i + 3 < sonarList.size(); i++) { // part 2
      sum2 =
          ((sonarList.get(i + 1) + sonarList.get(i + 2) + sonarList.get(i + 3)) > (sonarList.get(i)
              + sonarList.get(i + 1) + sonarList.get(i + 2))) ? sum2 + 1 : sum2;
    }
    System.out.println(sum1);
    System.out.println(sum2);
  }

  public static void main(String[] args) {
    day01();
  }
}
