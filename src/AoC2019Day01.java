import java.util.*;
import java.io.*;

public class AoC2019Day01 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day01() {
    int fuelRequirement1 = 0;
    int fuelRequirement2 = 0;
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2019Day01.txt")))) {
      while (fileSc.hasNextLine()) { // part 1
        fuelRequirement1 += Integer.parseInt(fileSc.nextLine()) / 3 - 2;
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2019Day01.txt")))) {
      while (fileSc.hasNextLine()) { // part 2
        int i = Integer.parseInt(fileSc.nextLine()) / 3 - 2;
        int j = i;
        while (i / 3 - 2 > 0) {
          i = i / 3 - 2;
          j += i;
        }
        fuelRequirement2 += j;
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    System.out.println(fuelRequirement1);
    System.out.println(fuelRequirement2);
  }

  public static void main(String[] args) {
    day01();
  }
}
