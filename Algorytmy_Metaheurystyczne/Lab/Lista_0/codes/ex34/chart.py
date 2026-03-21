import matplotlib.pyplot as plt

# Dane
panstwa = ['Djibouti', 'Qatar', 'Uruguay', 'Western Sahara', 'Zimbabwe']
wagi = [8718, 12237, 108140, 35609, 130424]

# Tworzenie wykresu
plt.figure(figsize=(10, 6))
plt.bar(panstwa, wagi, color='skyblue', edgecolor='navy')

# Opisy osi i tytuł
plt.ylabel('Waga znalezionego cyklu', fontsize=12)

# Dodanie wartości nad słupkami (opcjonalne, dla lepszej czytelności)
for i, v in enumerate(wagi):
    plt.text(i, v + 2000, str(v), ha='center', fontweight='bold')

# Wyświetlenie wykresu
plt.tight_layout()
plt.show()