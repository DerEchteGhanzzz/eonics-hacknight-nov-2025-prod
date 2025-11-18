import random
import os

if __name__ == "__main__":
    sizes = [
        "Small",
        "Medium",
        "Large",
        "American"
    ]

    result = []
    for _ in range(1000):
        result.append(random.choice(sizes))

    with open("src/input_files/input1.txt", "w", newline='') as f:
        f.writelines("\n".join(result))