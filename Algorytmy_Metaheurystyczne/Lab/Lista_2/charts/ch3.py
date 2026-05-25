import re
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

data = """
Tabu tenure: 10, max_stagnation:  50 => Dystans: 9565
Tabu tenure: 20, max_stagnation:  50 => Dystans: 9696
Tabu tenure: 30, max_stagnation:  50 => Dystans: 9734
Tabu tenure: 40, max_stagnation:  50 => Dystans: 9655
Tabu tenure: 50, max_stagnation:  50 => Dystans: 9560
Tabu tenure: 10, max_stagnation: 100 => Dystans: 9641
Tabu tenure: 20, max_stagnation: 100 => Dystans: 9489
Tabu tenure: 30, max_stagnation: 100 => Dystans: 9594
Tabu tenure: 40, max_stagnation: 100 => Dystans: 9659
Tabu tenure: 50, max_stagnation: 100 => Dystans: 9666
Tabu tenure: 10, max_stagnation: 150 => Dystans: 9691
Tabu tenure: 20, max_stagnation: 150 => Dystans: 9660
Tabu tenure: 30, max_stagnation: 150 => Dystans: 9664
Tabu tenure: 40, max_stagnation: 150 => Dystans: 9679
Tabu tenure: 50, max_stagnation: 150 => Dystans: 9594
Tabu tenure: 10, max_stagnation: 200 => Dystans: 9584
Tabu tenure: 20, max_stagnation: 200 => Dystans: 9641
Tabu tenure: 30, max_stagnation: 200 => Dystans: 9694
Tabu tenure: 40, max_stagnation: 200 => Dystans: 9701
Tabu tenure: 50, max_stagnation: 200 => Dystans: 9677
"""

# Parsowanie danych
parsed_data = []
# Zaktualizowane wyrażenie regularne
pattern = r"Tabu tenure:\s+(\d+),\s+max_stagnation:\s+(\d+)\s+=>\s+Dystans:\s+(\d+)"

for line in data.strip().split('\n'):
    match = re.search(pattern, line)
    if match:
        tenure = int(match.group(1))
        stag = int(match.group(2))
        dist = int(match.group(3))
        parsed_data.append({'Tabu tenure': tenure, 'Max stagnation': stag, 'Dystans': dist})

# Tworzenie DataFrame
df = pd.DataFrame(parsed_data)

# Przekształcenie tabeli
pivot_df = df.pivot(index="Max stagnation", columns="Tabu tenure", values="Dystans")

# Tworzenie wykresu
plt.figure(figsize=(10, 6))

ax = sns.heatmap(pivot_df, annot=True, fmt="d", cmap="YlGnBu", cbar_kws={'label': 'Dystans'})

# Odwrócenie osi Y
ax.invert_yaxis()

# Ustawienie tytułów i etykiet
plt.title("Tabu Search results - Qatar", fontsize=16, pad=15)
plt.xlabel("Tabu tenure", fontsize=12)
plt.ylabel("Max stagnation", fontsize=12)

# Wyświetlenie wykresu
plt.tight_layout()
plt.show()