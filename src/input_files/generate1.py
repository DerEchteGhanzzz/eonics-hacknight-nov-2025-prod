import random
import os

if __name__ == "__main__":
    ingredients = [
        "Sauce",
        "Mozzerella",
        "Pepperoni",
        "Gorgonzola",
        "Fontina",
        "Parmesan",
        "Artichokes",
        "Mushrooms",
        "Ham",
        "Olives",
        "Pineapple",
        "Spinach",
        "Tuna",
        "Onion",
        "Bacon"
    ]

    print(os.getcwd())
    result = []
    for _ in range(1000):
        result.append(random.choice(ingredients))

    with open("src/input_files/input1.txt", "w") as f:
        f.writelines("\n".join(result))