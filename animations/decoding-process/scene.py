from enum import Enum
from typing_extensions import runtime
from manim import *

class Grid:
    group: VGroup
    lines: VGroup
    cells: VGroup

    def __init__(self, colors):
        X = 3
        Y = 1
        self.colors = colors
        self.scale = 1

        self.lines = VGroup(
            *[
                Line((-X, Y, 0), (X, Y, 0)),
                Line((-X, -Y, 0), (X, -Y, 0)),
                Line((-Y, X, 0), (-Y, -X, 0)),
                Line((Y, X, 0), (Y, -X, 0)),
            ]
        )
        self.change_lines_color(colors[0])
        self.cells = VGroup()
        self.group = VGroup(self.lines, self.cells)

    def change_lines_color(self, color):
        self.lines.set_color(color)

    def change_scale(self, new_scale: float):
        self.scale *= new_scale
        self.group.scale(self.scale)

    def add_cell(self, cell: str, subgrid: VGroup | None):
        scale_down = 0.6
        index = 8 - len(self.cells)
        x = index % 3 - 1
        y = -(index // 3) + 1
        if cell == "000":
            c = Dot(color=self.colors[1])
        elif cell == "100":
            c = subgrid.copy()
            scale_down = 1
        elif cell == "001":
            c = Circle(color=self.colors[1])
        else:
            c = Cross(color=self.colors[1])

        c.set_x(x * 2 * self.scale)
        c.set_y(y * 2 * self.scale)
        c.scale(scale_down * self.scale)
        return c


def update_part(scene: Scene, index: int, text: VGroup):
    dt = 0.5
    scene.play(text[index].animate(run_time=dt).set_color(YELLOW))
    return text[index]

class DecodeProcess(Scene):
    def construct(self):
        matching_values = [
                "Empty: 000",
                "Circle: 001",
                "Cross: 010",
                "Both: 011",
                "Subgrid: 100",
        ]
        match_table = VGroup(
                *[
                    Text(t, font_size=30)
                    for t in matching_values
                ]
        ).arrange(DOWN, aligned_edge=LEFT)
        r = SurroundingRectangle(match_table, color=WHITE, buff=0.5, corner_radius=0.2)
        r.surround(match_table)
        match_table = VGroup(Text("Match table:", font_size=35).next_to(match_table, UP), r) + match_table
        self.play(Write(match_table))
        self.wait(1.5)

        match_table[0].set_color(BLACK)
        match_table = match_table[1:]
        self.play(match_table.animate.scale(0.8).shift(6*LEFT + 2*UP))
        
        bytes_string = "000000010010001000001100000000001000010001010000010001"
        n = len(bytes_string)
        text = VGroup(
                *[
                    Text(bytes_string[i:i+3], font_size=30)
                    for i in range(0, n, 3)
                ]
        ).arrange_in_grid(cols=(n // 3), buff=0.1)
        self.play(Write(text))
        self.play(text.animate.scale(0.7).shift(3*DOWN + 2*LEFT))

        queue = Rectangle(width=2.0, height=4.0, grid_ystep=2, color=BLUE)
        queue.shift(5*RIGHT)

        queue_title = Text("Subgrid  Queue", font_size=20, color=BLUE)
        queue_title.next_to(queue, UP)
        self.play(Write(queue_title), Write(queue))


        current_bits_index = n // 3 - 1

        subgrids = []

        for sub_index in range(n // 27 - 1, -1, -1):
            grid = Grid((WHITE, RED))
            grid.change_scale(0.5)
            self.play(Write(grid.group))
            speed = 0.5
            for j in range(24, -1, -3):
                current_bits = update_part(self, current_bits_index, text)
                current_bits_index -= 1

                sub_bits = bytes_string[sub_index * 27 + j:sub_index * 27 + j + 3]
                if sub_bits == "100":
                    current_bits.set_color(BLACK)
                    current_bits = subgrids.pop().group

                new_cell = grid.add_cell(sub_bits, current_bits)
                self.play(Transform(current_bits, new_cell, run_time=speed))
                grid.cells.add(current_bits)

            if (sub_index == 0):
                break
            self.play(grid.group.animate.scale(0.5).next_to(queue_title, 2 * DOWN))
            subgrids.append(grid)

        self.wait(1)
