import re
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

data = """
Tabu tenure: 10, max_stagnation:  50 => Dystans: 27603
Tabu tenure: 20, max_stagnation:  50 => Dystans: 27603
Tabu tenure: 30, max_stagnation:  50 => Dystans: 27603
Tabu tenure: 40, max_stagnation:  50 => Dystans: 27603
Tabu tenure: 50, max_stagnation:  50 => Dystans: 27603
Tabu tenure: 10, max_stagnation: 100 => Dystans: 27603
Tabu tenure: 20, max_stagnation: 100 => Dystans: 27603
Tabu tenure: 30, max_stagnation: 100 => Dystans: 27603
Tabu tenure: 40, max_stagnation: 100 => Dystans: 27603
Tabu tenure: 50, max_stagnation: 100 => Dystans: 27603
Tabu tenure: 10, max_stagnation: 150 => Dystans: 27603
Tabu tenure: 20, max_stagnation: 150 => Dystans: 27603
Tabu tenure: 30, max_stagnation: 150 => Dystans: 27603
Tabu tenure: 40, max_stagnation: 150 => Dystans: 27603
Tabu tenure: 50, max_stagnation: 150 => Dystans: 27603
Tabu tenure: 10, max_stagnation: 200 => Dystans: 27603
Tabu tenure: 20, max_stagnation: 200 => Dystans: 27603
Tabu tenure: 30, max_stagnation: 200 => Dystans: 27603
Tabu tenure: 40, max_stagnation: 200 => Dystans: 27603
Tabu tenure: 50, max_stagnation: 200 => Dystans: 27603
"""

# Parsowanie danych
parsed_data = []
pattern = r"Tabu tenure:\s+(\d+),\s+max_stagnation:\s+(\d+)\s+=>\s+Dystans:\s+(\d+)"

for line in data.strip().split('\n'):
    match = re.search(pattern, line)
    if match:
        tenure = int(match.group(1))
        stag = int(match.group(2))
        dist = int(match.group(3))
        parsed_data.append({'Tabu tenure': tenure, 'Max stagnation': stag, 'Dystans': dist})

# Tworzenie DataFrame i przekształcenie na format macierzowy
df = pd.DataFrame(parsed_data)
pivot_df = df.pivot(index="Max stagnation", columns="Tabu tenure", values="Dystans")

# Tworzenie wykresu
plt.figure(figsize=(10, 6))

# Z racji braku wariancji w danych (wszystko to 27603), seaborn może zignorować mapę kolorów.
# Ustawiamy vmin i vmax delikatnie poniżej i powyżej tej wartości, by wykres wygenerował się poprawnie.
ax = sns.heatmap(pivot_df, annot=True, fmt="d", cmap="YlGnBu", cbar_kws={'label': 'Dystans'}, vmin=27600, vmax=27610)

ax.invert_yaxis()

# Ustawienie tytułów i etykiet
plt.title("Tabu Search results - Western Sahara", fontsize=16, pad=15)
plt.xlabel("Tabu tenure", fontsize=12)
plt.ylabel("Max stagnation", fontsize=12)

# Wyświetlenie
plt.tight_layout()
plt.show()