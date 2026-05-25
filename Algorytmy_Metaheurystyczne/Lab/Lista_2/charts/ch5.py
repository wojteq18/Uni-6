import re
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Twoje dane wejściowe
data = """
Temperatura początkowa:    100.0, cooling_rate: 0.800 => Dystans: 33026
Temperatura początkowa:   1000.0, cooling_rate: 0.800 => Dystans: 27852
Temperatura początkowa:  10000.0, cooling_rate: 0.800 => Dystans: 29923
Temperatura początkowa: 100000.0, cooling_rate: 0.800 => Dystans: 30271
Temperatura początkowa:    100.0, cooling_rate: 0.845 => Dystans: 27573
Temperatura początkowa:   1000.0, cooling_rate: 0.845 => Dystans: 26649
Temperatura początkowa:  10000.0, cooling_rate: 0.845 => Dystans: 27812
Temperatura początkowa: 100000.0, cooling_rate: 0.845 => Dystans: 25855
Temperatura początkowa:    100.0, cooling_rate: 0.890 => Dystans: 23679
Temperatura początkowa:   1000.0, cooling_rate: 0.890 => Dystans: 22159
Temperatura początkowa:  10000.0, cooling_rate: 0.890 => Dystans: 22237
Temperatura początkowa: 100000.0, cooling_rate: 0.890 => Dystans: 21620
Temperatura początkowa:    100.0, cooling_rate: 0.935 => Dystans: 20028
Temperatura początkowa:   1000.0, cooling_rate: 0.935 => Dystans: 17999
Temperatura początkowa:  10000.0, cooling_rate: 0.935 => Dystans: 17779
Temperatura początkowa: 100000.0, cooling_rate: 0.935 => Dystans: 17892
Temperatura początkowa:    100.0, cooling_rate: 0.980 => Dystans: 13024
Temperatura początkowa:   1000.0, cooling_rate: 0.980 => Dystans: 13418
Temperatura początkowa:  10000.0, cooling_rate: 0.980 => Dystans: 13417
Temperatura początkowa: 100000.0, cooling_rate: 0.980 => Dystans: 12967
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

# Zastosowanie formatu ',d' dla czytelnego zapisu dużych liczb
ax = sns.heatmap(pivot_df, annot=True, fmt=",d", cmap="YlGnBu", cbar_kws={'label': 'Dystans'})

# Odwrócenie osi Y, aby wartości cooling_rate rosły z dołu do góry
ax.invert_yaxis()

# Ustawienie tytułów i etykiet
plt.title("Katar", fontsize=16, pad=15)
plt.xlabel("Temperatura początkowa", fontsize=12)
plt.ylabel("Cooling Rate", fontsize=12)

# Wyświetlenie wykresu
plt.tight_layout()
plt.show()