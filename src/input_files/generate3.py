import random
import os
import sys

names = [
    "Peter Jansen, Jan",
    "De Transparante Broker",
    "De Transparante Detective",
    "NIXZ",
    "Cafe Seven",
    "Ponypark Slagharen",
    "Rusty Krab",
    "Koolmees, Wouter",
    "Pizzaria Tricolore",
    "Pakjesboot 12",
    "Euromast",
    "Binnenhof",
    "Kasteel de Haar"
]

d = {}
def populate_dist_matrix():
    rand = random.Random(123)
    for name in names:
        if name not in d:
            d[name] = {}
        for name2 in names:
            if name2 not in d:
                d[name2] = {}
            if name2 == name:
                d[name][name2] = 0
                d[name2][name] = 0
                continue
            if name2 not in d[name]:
                r = rand.randint(10, 100)
                d[name][name2] = r
                d[name2][name] = r
    lines = []
    for s, row in sorted(d.items()):
        for t, val in sorted(d[s].items()):
            lines.append(f"{s};{t};{val}")
    with open("src/input_files/locations.txt", 'w', newline='') as f:
        f.write("\n".join(names))
    with open("src/input_files/from-to.txt", 'w', newline='') as f:
        f.write("\n".join(lines))

def tsp(visited: list[str]):
    if len(visited) == len(names):
        return d[visited[-1]][visited[0]]
    
    best = sys.maxsize
    for name in names:
        if name in visited:
            continue
        new_visited = visited[:]
        new_visited.append(name)
        best = min(best, tsp(new_visited) + d[visited[-1]][name])
    return best

def read_input():
    d_raw = []
    with open("src/input_files/input3.txt", "r") as f:
        d_raw = [x.strip() for x in f.readlines()]
    header = d_raw[0].split(";")[1:]
    d = {}
    for row in d_raw[1:]:
        split_row = row.split(";")
        f = split_row[0]
        d[f] = {h: {} for h in header}
        for (idx, value) in enumerate(split_row[1:]):
            print(f, header[idx], value)
            d[f][header[idx]] = int(value)
    return d
    
if __name__ == "__main__":
    
    with open("src/input_files/locations.txt", 'w', newline='') as f:
        f.write("\n".join(names))