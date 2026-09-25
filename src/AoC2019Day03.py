import sys

DIR: dict[str, tuple[int, int]] = {"U": (0, 1), "R": (1, 0), "D": (0, -1), "L": (-1, 0)}


class Segment:
    def __init__(
        self, start: tuple[int, int], direction: str, length: int, start_steps: int
    ) -> None:
        self.__start_x, self.__start_y = start
        self.__end_x, self.__end_y = (
            start[0] + DIR.get(direction, (0, 0))[0] * length,
            start[1] + DIR.get(direction, (0, 0))[1] * length,
        )
        self.__start_steps: int = start_steps

    def get_start_steps(self) -> int:
        return self.__start_steps

    def get_start(self) -> tuple[int, int]:
        return (self.__start_x, self.__start_y)

    def get_end(self) -> tuple[int, int]:
        return (self.__end_x, self.__end_y)

    def get_start_x(self) -> int:
        return self.__start_x

    def get_start_y(self) -> int:
        return self.__start_y

    def get_min_x(self) -> int:
        return min(self.__start_x, self.__end_x)

    def get_max_x(self) -> int:
        return max(self.__start_x, self.__end_x)

    def get_min_y(self) -> int:
        return min(self.__start_y, self.__end_y)

    def get_max_y(self) -> int:
        return max(self.__start_y, self.__end_y)

    def is_horizontal(self) -> bool:
        return self.__start_y == self.__end_y

    def is_vertical(self) -> bool:
        return self.__start_x == self.__end_x


def parse_wire(wire: list[str]) -> list[Segment]:
    segments: list[Segment] = []
    current_pos: tuple[int, int] = (0, 0)
    steps: int = 0

    for segment in wire:
        direction: str = segment[0]
        length: int = int(segment[1:])
        segments.append(Segment(current_pos, direction, length, steps))
        current_pos = segments[-1].get_end()
        steps += length

    return segments


def find_intersection(
    segments_1: list[Segment], segments_2: list[Segment]
) -> tuple[int, int]:
    min_combined_steps: int = sys.maxsize
    intersections: list[tuple[int, int]] = []

    for seg_1 in segments_1:
        for seg_2 in segments_2:
            if seg_1.is_horizontal() and seg_2.is_horizontal():
                continue

            if seg_1.is_vertical() and seg_2.is_vertical():
                continue

            h_seg: Segment = seg_1 if seg_1.is_horizontal() else seg_2
            v_seg: Segment = seg_1 if seg_1.is_vertical() else seg_2

            if (
                h_seg.get_min_x() <= v_seg.get_start_x() <= h_seg.get_max_x()
                and v_seg.get_min_y() <= h_seg.get_start_y() <= v_seg.get_max_y()
            ):
                intersection: tuple[int, int] = (
                    v_seg.get_start_x(),
                    h_seg.get_start_y(),
                )
                intersections.append(intersection)

                seg_1_steps: int = (
                    seg_1.get_start_steps()
                    + abs(intersection[0] - seg_1.get_start_x())
                    + abs(intersection[1] - seg_1.get_start_y())
                )
                seg_2_steps: int = (
                    seg_2.get_start_steps()
                    + abs(intersection[0] - seg_2.get_start_x())
                    + abs(intersection[1] - seg_2.get_start_y())
                )
                min_combined_steps = min(min_combined_steps, seg_1_steps + seg_2_steps)

    min_mahattan_distance: int = min((abs(x) + abs(y)) for (x, y) in intersections)
    return (min_mahattan_distance, min_combined_steps)


if __name__ == "__main__":
    with open("src/input/AoC2019Day03.txt") as f:
        lines: list[str] = f.read().splitlines()

    wire_1: list[str] = lines[0].split(",")
    wire_2: list[str] = lines[1].split(",")

    segments_1: list[Segment] = parse_wire(wire_1)
    segments_2: list[Segment] = parse_wire(wire_2)

    print(find_intersection(segments_1, segments_2))
