import java.io.*;
import java.util.*;

public class AoC2021Day03 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day03() {
    List<Integer> binDigitList = null;
    List<String> OGenRating = new ArrayList<>();
    List<String> CO2ScrRating = new ArrayList<>();
    String listDir = inputFolderDir.concat("AoC2021Day03.txt");
    int[] gammaRate = null;
    int[] epsilonRate = null;
    String gammaRateStr = "";
    String epsilonRateStr = "";
    int digitLength = 1;
    for (int i = 0; i < digitLength; i++) { // part 1
      try (BufferedReader binDigits = new BufferedReader(new FileReader(listDir))) {
        binDigitList = new ArrayList<>();
        int numOfZero = 0;
        int numOfOne = 0;
        String digit = binDigits.readLine();
        digitLength = i == 0 ? digit.length() : digitLength;
        gammaRate = i == 0 ? new int[digit.length()] : gammaRate;
        epsilonRate = i == 0 ? new int[digit.length()] : epsilonRate;
        while (digit != null) {
          binDigitList.add(Character.getNumericValue(digit.charAt(i)));
          digit = binDigits.readLine();
        }
        for (int j : binDigitList) {
          if (j == 0) {
            numOfZero++;
          } else {
            numOfOne++;
          }
        }
        gammaRate[i] = (numOfOne > numOfZero) ? 1 : 0;
        epsilonRate[i] = 1 - gammaRate[i];
      } catch (IOException e) {
        e.printStackTrace();
      }
    }
    for (int i : gammaRate) {
      gammaRateStr += Integer.toString(i);
      epsilonRateStr += Integer.toString(1 - i);
    }
    System.out.println(Integer.parseInt(gammaRateStr, 2) * Integer.parseInt(epsilonRateStr, 2));
    try (Scanner fileSc = new Scanner(new File(AoC2021Day03.inputFolderDir.concat("AoC2021Day03.txt")))) {
      while (fileSc.hasNextLine()) {
        String str = fileSc.nextLine();
        OGenRating.add(str);
        CO2ScrRating.add(str);
      }
      Collections.sort(OGenRating);
      Collections.sort(CO2ScrRating);
    } catch (FileNotFoundException e) {
      System.out.println("An error occurred.");
      e.printStackTrace();
    }
    for (int i = 0; (OGenRating.size() != 1 || CO2ScrRating.size() != 1)
        && i < OGenRating.get(0).length() && i < CO2ScrRating.get(0).length(); i++) { // part 2
      int numOfOne1 = 0;
      int numOfZero1 = 0;
      int numOfOne2 = 0;
      int numOfZero2 = 0;
      char rmFlag1 = '0';
      char rmFlag2 = '0';
      for (String str : OGenRating) {
        if (str.charAt(i) == '0') {
          numOfZero1++;
        } else if (str.charAt(i) == '1') {
          numOfOne1++;
        }
      }
      for (String str : CO2ScrRating) {
        if (str.charAt(i) == '0') {
          numOfZero2++;
        } else if (str.charAt(i) == '1') {
          numOfOne2++;
        }
      }
      rmFlag1 = numOfOne1 >= numOfZero1 ? '0' : '1';
      rmFlag2 = numOfOne2 >= numOfZero2 ? '1' : '0';
      for (int k = 0; k < OGenRating.size() && OGenRating.size() != 1; k++) {
        if (OGenRating.get(k).charAt(i) == rmFlag1) {
          OGenRating.remove(k);
          k--;
        }
      }
      for (int k = 0; k < CO2ScrRating.size() && CO2ScrRating.size() != 1; k++) {
        if (CO2ScrRating.get(k).charAt(i) == rmFlag2) {
          CO2ScrRating.remove(k);
          k--;
        }
      }
    }
    System.out
        .println(Integer.parseInt(OGenRating.get(0), 2) * Integer.parseInt(CO2ScrRating.get(0), 2));
  }
  
  public static void main(String[] args) {
    day03();
  }
}
