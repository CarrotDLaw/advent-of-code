import java.util.*;
import java.io.*;

public class AoC2019Day02 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");
  static List<Integer> intcodeList = new ArrayList<>();

  public static void intcodeListSetup() {
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2019Day02.txt")))) {
      String[] intcodeStrArray = fileSc.nextLine().split(",");
      for (String str : intcodeStrArray) {
        intcodeList.add(Integer.parseInt(str));
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
  }

  public static int intcodeProcessor(List<Integer> inputList, int value1, int value2) {
    inputList.set(1, value1);
    inputList.set(2, value2);
    for (int opcodeIndex = 0; opcodeIndex < inputList.size(); opcodeIndex += 4) {
      int opcode = inputList.get(opcodeIndex);
      if (opcode == 1) {
        inputList.set(inputList.get(opcodeIndex + 3), inputList.get(inputList.get(opcodeIndex + 1))
            + inputList.get(inputList.get(opcodeIndex + 2)));
      } else if (opcode == 2) {
        inputList.set(inputList.get(opcodeIndex + 3), inputList.get(inputList.get(opcodeIndex + 1))
            * inputList.get(inputList.get(opcodeIndex + 2)));
      } else if (opcode == 99) {
        break;
      } else {
        break;
      }
    }
    return inputList.get(0);
  }

  public static void day02() {
    int requiredOutput = 19_690_720;
    System.out.println(intcodeProcessor(new ArrayList<>(intcodeList), 12, 2)); // part 1
    for (int noun = 0; noun <= 99; noun++) { // part 2
      for (int verb = 0; verb <= 99; verb++) {
        int output = intcodeProcessor(new ArrayList<>(intcodeList), noun, verb);
        if (output == requiredOutput) {
          System.out.println(100 * noun + verb);
          return;
        }
      }
    }
  }

  public static void main(String[] args) {
    intcodeListSetup();
    day02();
  }
}
