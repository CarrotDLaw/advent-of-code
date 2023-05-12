import java.io.*;
import java.util.*;

public class AoC2017Day01 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static List<Integer> captchaIntList = new ArrayList<>();

  public static void day01() {
    String[] captchaStr = {};
    int sum1 = 0;
    int sum2 = 0;
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2017Day01.txt")))) {
      while (fileSc.hasNextLine()) {
        String str = fileSc.nextLine();
        captchaStr = str.split("");
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    for (String str : captchaStr) {
      captchaIntList.add(Integer.parseInt(str));
    }
    int shift1 = 1;
    int shift2 = captchaIntList.size() / 2;
    for (int i = 0; i < captchaIntList.size(); i++) {
      if (captchaIntList.get(i) == captchaIntList.get((i + shift1) % captchaIntList.size())) { // part
                                                                                               // 1
        sum1 += captchaIntList.get(i);
      }
      if (captchaIntList.get(i) == captchaIntList.get((i + shift2) % captchaIntList.size())) { // part
                                                                                               // 2
        sum2 += captchaIntList.get(i);
      }
    }
    System.out.println(sum1);
    System.out.println(sum2);
  }

  public static void main(String[] args) {
    day01();
  }
}
