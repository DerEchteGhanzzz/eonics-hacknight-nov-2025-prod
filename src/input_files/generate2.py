import random
import os

if __name__ == "__main__":
    sizes = [
        "Small",
        "Medium",
        "Large",
        "XtraLarge",
        "Huge",
        "XtraHuge",
        "Obese",
        "AmericaMedium"
        "AmericaMedium"
    ]

    print(os.getcwd())
    result = []
    for _ in range(1000):
        result.append(random.choice(sizes))

    with open("src/input_files/input2.txt", "w") as f:
        f.writelines("\n".join(result))