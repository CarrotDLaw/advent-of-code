import java.io.*;
import java.util.*;

public class AoC2020Day05 {
  static String inputFolderDir = System.getProperty("user.dir").concat("/src/input/");

  public static void day05() {
    List<String> boardingPassList = new ArrayList<>();
    List<Integer> seatID = new ArrayList<>();
    try (Scanner fileSc = new Scanner(new File(inputFolderDir.concat("AoC2020Day05.txt")))) {
      while (fileSc.hasNextLine()) {
        boardingPassList.add(fileSc.nextLine());
      }
    } catch (FileNotFoundException e) {
      System.out.println("An error occurred.");
      e.printStackTrace();
    }
    for (String boardingPass : boardingPassList) { // part 1
      String idBase2 =
          boardingPass.replace('F', '0').replace('B', '1').replace('L', '0').replace('R', '1');
      seatID.add(Integer.parseInt(idBase2, 2));
    }
    Collections.sort(seatID);
    System.out.println(Collections.max(seatID));
    for (int i = 0, j = seatID.get(0); i < seatID.size(); i++, j++) { // part 2
      if (j != seatID.get(i)) {
        System.out.println(j);
        break;
      }
    }
  }

  public static void main(String[] args) {
    day05();
  }
}
