import java.io.*;
import java.util.*;

public class AoC2017Day04 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static List<List<String>> passphraseList = new ArrayList<List<String>>();

  public static int passphraseValidation(List<List<String>> inputList) {
    int validCount = 0;
    for (List<String> passphrase : passphraseList) {
      boolean invalid = false;
      for (int i = 0; i < passphrase.size(); i++) {
        for (int j = i + 1; j < passphrase.size(); j++) {
          if (passphrase.get(i).equals(passphrase.get(j))) {
            invalid = true;
            break;
          }
        }
        if (invalid) {
          break;
        }
      }
      if (invalid) {
        continue;
      }
      validCount++;
    }
    return validCount;
  }

  public static void day04() {
    int valid1 = 0, valid2 = 0;
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2017Day04.txt")))) {
      while(fileSc.hasNextLine()) {
        passphraseList.add(new ArrayList<>(Arrays.asList(fileSc.nextLine().split(" "))));
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    valid1 = passphraseValidation(new ArrayList<>(passphraseList));
    System.out.println(valid1); // part 1
    for (List<String> passphrase : passphraseList) {
      for (int i = 0; i < passphrase.size(); i++) {
        String str = passphrase.get(i);
        char[] charArray = str.toCharArray();
        Arrays.sort(charArray);
        String sortedStr = String.valueOf(charArray);
        passphrase.set(i, sortedStr);
      }
    }
    valid2 = passphraseValidation(new ArrayList<>(passphraseList));
    System.out.println(valid2); // part 2
  }

  public static void main(String[] args) {
    day04();
  }
}
