import re
import matplotlib.pyplot as plt


with open("../../results/djibouti.txt", "r", encoding="utf-8") as f:
    text = f.read()

pattern = re.compile(r"Average result with\s+(\d+)\s+groups:\s+(\d+)")
matches = pattern.findall(text)

if len(matches) != 3:
    raise SystemExit(f"Nie znaleziono 3 wyników w oczekiwanym formacie. Znaleziono: {len(matches)}")

groups = [int(g) for g, _ in matches]
values = [int(v) for _, v in matches]

# Sortujemy po liczbie grup (żeby oś X była 1, 20, 100)
pairs = sorted(zip(groups, values), key=lambda x: x[0])
groups, values = zip(*pairs)

plt.figure(figsize=(7, 4))

labels = [f"{g}" for g in groups]
bars = plt.bar(labels, values)

plt.title("Result: Djibouti")
plt.xlabel("Amount of groups")
plt.ylabel("Value (route length)")

plt.grid(True, axis="y", linestyle="--", alpha=0.4)

# opisy słupków
for bar, v in zip(bars, values):
    plt.text(
        bar.get_x() + bar.get_width() / 2,
        bar.get_height(),
        str(v),
        ha="center",
        va="bottom"
    )

plt.tight_layout()
plt.savefig("wyniki_tsp.png", dpi=200)
plt.show()