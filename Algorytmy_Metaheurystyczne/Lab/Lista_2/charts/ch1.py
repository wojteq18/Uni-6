import re
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Twoje dane wejściowe
data = """
Temperatura początkowa:    100.0, cooling_rate: 0.800 => Dystans: 13357093
Temperatura początkowa:   1000.0, cooling_rate: 0.800 => Dystans: 12993569
Temperatura początkowa:  10000.0, cooling_rate: 0.800 => Dystans: 12748129
Temperatura początkowa: 100000.0, cooling_rate: 0.800 => Dystans: 12752955
Temperatura początkowa:    100.0, cooling_rate: 0.845 => Dystans: 13067975
Temperatura początkowa:   1000.0, cooling_rate: 0.845 => Dystans: 12576241
Temperatura początkowa:  10000.0, cooling_rate: 0.845 => Dystans: 12326843
Temperatura początkowa: 100000.0, cooling_rate: 0.845 => Dystans: 12318561
Temperatura początkowa:    100.0, cooling_rate: 0.890 => Dystans: 12686353
Temperatura początkowa:   1000.0, cooling_rate: 0.890 => Dystans: 12018132
Temperatura początkowa:  10000.0, cooling_rate: 0.890 => Dystans: 11677067
Temperatura początkowa: 100000.0, cooling_rate: 0.890 => Dystans: 11556898
Temperatura początkowa:    100.0, cooling_rate: 0.935 => Dystans: 11604213
Temperatura początkowa:   1000.0, cooling_rate: 0.935 => Dystans: 10741316
Temperatura początkowa:  10000.0, cooling_rate: 0.935 => Dystans: 10315544
Temperatura początkowa: 100000.0, cooling_rate: 0.935 => Dystans: 10246345
Temperatura początkowa:    100.0, cooling_rate: 0.980 => Dystans: 8414255
Temperatura początkowa:   1000.0, cooling_rate: 0.980 => Dystans: 7343838
Temperatura początkowa:  10000.0, cooling_rate: 0.980 => Dystans: 6863995
Temperatura początkowa: 100000.0, cooling_rate: 0.980 => Dystans: 6876597
"""

# Parsowanie danych
parsed_data = []
pattern = r"Temperatura początkowa:\s+([\d\.]+),\s+cooling_rate:\s+([\d\.]+)\s+=>\s+Dystans:\s+(\d+)"

for line in data.strip().split('\n'):
    match = re.search(pattern, line)
    if match:
        temp = float(match.group(1))
        cr = float(match.group(2))
        dist = int(match.group(3))
        parsed_data.append({'Temperatura początkowa': temp, 'Cooling Rate': cr, 'Dystans': dist})

# Tworzenie DataFrame
df = pd.DataFrame(parsed_data)

# Przekształcenie tabeli na format macierzowy (pivot) potrzebny do heatmapy
pivot_df = df.pivot(index="Cooling Rate", columns="Temperatura początkowa", values="Dystans")

# Tworzenie wykresu
plt.figure(figsize=(10, 6))

# Zastosowanie ',d' dodaje separatory tysięcy dla duzych liczb
ax = sns.heatmap(pivot_df, annot=True, fmt=",d", cmap="YlGnBu", cbar_kws={'label': 'Dystans'})

# Odwrócenie osi Y
ax.invert_yaxis()

# Ustawienie tytułów i etykiet - zmieniłem tytuł na "Kanada" ze względu na dużą instancję
plt.title("Egipt", fontsize=16, pad=15)
plt.xlabel("Temperatura początkowa", fontsize=12)
plt.ylabel("Cooling Rate", fontsize=12)

# Wyświetlenie wykresu
plt.tight_layout()
plt.show()