#!/usr/bin/env python3

import argparse
import copy
import itertools
import os
import random


THEMES = {
    "cc242": {
        "background": "#faf4e4",
        "palette": ["#bbd444", "#fcd744", "#fa7b53", "#423c6f"],
        "stroke": "black",
    },
    "cc239": {
        "palette": ["#e3dd34", "#78496b", "#f0527f", "#a7e0e2"],
        "background": "#e0eff0",
    },
    "cc234": {
        "palette": ["#ffce49", "#ede8dc", "#ff5736", "#ff99b4"],
        "background": "#f7f4ed",
    },
    "cc232": {
        "palette": ["#5c5f46", "#ff7044", "#ffce39", "#66aeaa"],
        "background": "#e9ecde",
    },
    "cc238": {
        "palette": ["#553c60", "#ffb0a0", "#ff6749", "#fbe090"],
        "background": "#f5e9de",
    },
    "cc242": {
        "palette": ["#bbd444", "#fcd744", "#fa7b53", "#423c6f"],
        "background": "#faf4e4",
    },
    "cc245": {
        "palette": ["#0d4a4e", "#ff947b", "#ead3a2", "#5284ab"],
        "background": "#f6f4ed",
    },
    "cc273": {
        "palette": ["#363d4a", "#7b8a56", "#ff9369", "#f4c172"],
        "background": "#f0efe2",
    },
    "acid": {
        "background": "rgb(0, 128, 0)",
        "palette": [
            "rgb(60, 158, 11)",
            "rgb(98, 190, 23)",
            "rgb(135, 222, 35)",
            "rgb(172, 255, 47)",
        ],
        "stroke": "black",
    },
}


class Lines:
    def __init__(self):
        self.lines = []

    def add(self, l):
        self.lines.append(l)

    def bbox(self):
        (l, t, r, b) = (1e30, 1e30, 1e-30, 1e-30)

        for line in self.lines:
            for x, y in line:
                l = min(x, l)
                r = max(x, r)
                t = min(y, t)
                b = max(y, b)

        return (l, t, r, b)

    def xform(self, ncx, ncy, w, h):
        (l, t, r, b) = self.bbox()
        cx = (l + r) / 2
        cy = (t + b) / 2

        f = min(w / (r - l), h / (b - t))

        for line in self.lines:
            for i in range(len(line)):
                x, y = line[i]

                x = (x - cx) * f + ncx
                y = (y - cy) * f + ncy

                line[i] = (x, y)

    def __iter__(self):
        return iter(self.lines)

    @staticmethod
    def random_building(width, height, nlines):
        lines = Lines()

        for _ in range(nlines):
            line = [(width // 2, 0)]

            while True:
                x, y = line[-1]

                h = random.randrange(1, 9)
                line.append((x, min(y + h, height - 1)))

                if y + h >= height:
                    break

                ey = line[-1][1]
                if random.random() > 0.5:
                    if x + 1 < width:
                        line.append((x + 1, ey))
                else:
                    if x - 1 >= 0:
                        line.append((x - 1, ey))

            lines.add(line)

        return lines


class Grid:
    def __init__(self, lines):
        self.bbox = lines.bbox()
        (l, t, r, b) = self.bbox

        self.lines_points = set()
        for line in lines:
            px, py = line[0]
            for x, y in line[1:]:
                assert x == px or y == py

                if x == px:
                    yy = py
                    d = 1 if y >= py else -1
                    while yy != y:
                        yy += d
                        self.lines_points.add(((x, yy - d), (x, yy)))
                else:
                    xx = px
                    d = 1 if x >= px else -1
                    while xx != x:
                        xx += d
                        self.lines_points.add(((xx - d, y), (xx, y)))

                px, py = x, y

        # force lines at borders
        for y in range(t - 1, b + 3):
            self.lines_points.add(((l - 2, y - 1), (l - 2, y)))
            self.lines_points.add(((r + 2, y - 1), (r + 2, y)))
        for x in range(l - 1, r + 3):
            self.lines_points.add(((x - 1, t - 2), (x, t - 2)))
            self.lines_points.add(((x - 1, b + 2), (x, b + 2)))

        # add extra padding to force background connection
        self.grid = []
        for _ in range(t - 2, b + 2):
            self.grid.append([-1 for _ in range(l - 2, r + 2)])

        self.max_area_id = 0
        self.areas = {}
        self.boundaries = {}

        width, height = len(self.grid[0]), len(self.grid)
        for y in range(height):
            for x in range(width):
                if self.grid[y][x] != -1:
                    continue

                self.areas[self.max_area_id] = self.flood_fill(x, y, self.max_area_id)
                self.boundaries[self.max_area_id] = self.find_boundary(
                    self.areas[self.max_area_id]
                )
                self.max_area_id += 1

    def line_between(self, p1, p2):
        p1 = (p1[0] + self.bbox[0] - 2, p1[1] + self.bbox[1] - 2)
        p2 = (p2[0] + self.bbox[0] - 2, p2[1] + self.bbox[1] - 2)

        if p1[0] == p2[0]:
            y = max(p1[1], p2[1])
            lp1 = (p1[0], y)
            lp2 = (p2[0] + 1, y)
        elif p1[1] == p2[1]:
            x = max(p1[0], p2[0])
            lp1 = (x, p1[1])
            lp2 = (x, p1[1] + 1)
        else:
            raise NotImplementedError()

        return (lp1, lp2) in self.lines_points or (lp2, lp1) in self.lines_points

    def flood_fill(self, x, y, area_id):
        cells = []

        stack = [(x, y)]
        while stack:
            x, y = stack.pop()
            if self.grid[y][x] != -1:
                continue

            self.grid[y][x] = area_id
            cells.append((x, y))

            for p in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
                if not self.line_between((x, y), p):
                    stack.append(p)

        return cells

    def areas_neighbors(self, remove_background=True):
        out = {}
        for area_id, pts in self.areas.items():
            if remove_background and area_id == 0:
                continue

            neighbors = set()
            for x, y in pts:
                neighbors |= {
                    self.grid[y + ny][x + nx]
                    for nx, ny in itertools.product((-1, 0, 1), repeat=2)
                }
            neighbors.remove(area_id)
            if remove_background and 0 in neighbors:
                neighbors.remove(0)
            out[area_id] = neighbors
        return out

    def find_boundary(self, area):
        boundary_lines = {}
        for x, y in area:
            edges = [
                ((x, y), (x + 1, y)),
                ((x + 1, y), (x + 1, y + 1)),
                ((x + 1, y + 1), (x, y + 1)),
                ((x, y + 1), (x, y)),
            ]

            for edge in edges:
                if edge in boundary_lines:
                    boundary_lines[edge] = False
                    continue

                if (edge[1], edge[0]) in boundary_lines:
                    boundary_lines[(edge[1], edge[0])] = False
                    continue

                boundary_lines[edge] = True

        starting_edge = next(
            e for e, is_boundary in boundary_lines.items() if is_boundary
        )

        current_edge = starting_edge
        boundary_lines[current_edge] = False

        boundary = [current_edge[0]]
        while True:
            next_edge = next(
                e
                for e, is_boundary in boundary_lines.items()
                if is_boundary and e[0] == current_edge[1]
            )

            # this is not correct in the general case, but in our setup it works
            # fine given that the boundary is processed in order
            if len(boundary) >= 2:
                if (
                    boundary[-1][0] == next_edge[0][0]
                    and boundary[-2][0] == next_edge[0][0]
                ) or (
                    boundary[-1][1] == next_edge[0][1]
                    and boundary[-2][1] == next_edge[0][1]
                ):
                    boundary.pop()

            boundary.append(next_edge[0])
            boundary_lines[next_edge] = False

            if next_edge[1] == starting_edge[0]:
                boundary.append(starting_edge[0])
                break

            current_edge = next_edge

        return boundary

    def debug_print(self):
        for l in self.grid:
            for aid in l:
                if aid == 0:
                    print(" ", end="")
                    continue
                print(chr(ord("a") + aid), end="")
            print()


def main():
    lines = Lines()
    grid_lines = Lines()
    grid_styles = []

    parser = argparse.ArgumentParser()
    parser.add_argument("--rows", default=2, type=int)
    parser.add_argument("--columns", default=8, type=int)
    parser.add_argument("--width", default=1920, type=int)
    parser.add_argument("--height", default=1080, type=int)
    parser.add_argument("-t", "--theme", choices=list(THEMES.keys()))
    parser.add_argument("-o", "--output", default="buildings.svg")
    parser.add_argument("--with-lines-only", action="store_true")
    parser.add_argument("--print-in-terminal", action="store_true")
    args = parser.parse_args()

    theme = THEMES[args.theme] if args.theme else THEMES[random.choice(list(THEMES))]

    buildingw = args.width / args.columns
    buildingh = args.height / args.rows

    for r in range(args.rows):
        for c in range(args.columns):
            ll, gl, gs = render_building(
                Lines.random_building(width=21, height=25, nlines=5),
                theme["palette"],
                args.print_in_terminal,
            )

            x = (c + 0.5) * buildingw
            y = (r + 0.5) * buildingh

            ll.xform(x, y, buildingw * 0.8, buildingh * 0.8)
            gl.xform(x, y, buildingw * 0.8, buildingh * 0.8)

            for line in ll:
                lines.add(line)
            for line in gl:
                grid_lines.add(line)

            grid_styles.extend(gs)

    if args.with_lines_only:
        dump_svg(
            "{}-lines.svg".format(os.path.splitext(args.output)[0]),
            lines,
            (args.width, args.height),
        )

    dump_svg(
        args.output,
        grid_lines,
        (args.width, args.height),
        background=theme["background"],
        styles=grid_styles,
    )


def render_building(lines, palette, log):
    # add bottom line
    (l, _, r, b) = lines.bbox()
    lines.add([(x, b) for x in range(l, r + 1)])

    grid = Grid(lines)

    if log:
        grid.debug_print()

    grid_colors = colorize(grid.areas_neighbors())

    grid_lines = Lines()
    grid_styles = []
    for area_id, path in grid.boundaries.items():
        if area_id == 0:
            continue

        grid_lines.add(path)

        (color_ix,) = grid_colors[area_id]
        grid_styles.append(("black", palette[color_ix]))

    return (lines, grid_lines, grid_styles)


def dump_svg(
    filename, data, dimensions, stroke_width=1, background="white", styles=None
):
    if styles is None:
        styles = itertools.repeat(("black", "none"))

    width, height = dimensions

    with open(filename, "wt") as fp:
        print(
            """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">
<rect width="{width}" height="{height}" stroke="none" fill="{background}" />
{lines}
</svg>""".format(
                width=width,
                height=height,
                background=background,
                lines="\n".join(
                    '<polyline points="{}" stroke-width="{}" stroke="{}" fill="{}" />'.format(
                        " ".join("{},{}".format(x, y) for x, y in line),
                        stroke_width,
                        stroke,
                        fill,
                    )
                    for line, (stroke, fill) in zip(data.lines, styles)
                ),
            ),
            file=fp,
        )


def colorize(neighbors_graph):
    stack = [{area_id: {0, 1, 2, 3} for area_id in neighbors_graph}]

    while True:
        sol = stack.pop()

        cand = None
        for area_id in neighbors_graph:
            if len(sol[area_id]) != 1:
                cand = area_id
                break

        if cand is None:
            return sol

        for color in sol[cand]:
            new_sol = copy.deepcopy(sol)
            new_sol[cand] = {color}
            good = True

            for n in neighbors_graph[cand]:
                if color in new_sol[n]:
                    new_sol[n].remove(color)
                if not new_sol[n]:
                    good = False

            if good:
                stack.append(new_sol)


if __name__ == "__main__":
    main()
