import matplotlib.pyplot as plt
import pandas as pd

# 1. Przygotowanie danych
data = {
    'Państwo': ['Oman', 'Dżibuti', 'Irlandia', 'Egipt', 'Tanzania', 'Kanada'],
    'avg_res': [93848.80, 6828.20, 229621.57, 189097.36, 197495.40, 1416488.80],
    'avg_num_steps': [2788.60, 34.40, 12099.33, 10382.33, 4232.20, 6797.60],
    'best_res': [92363, 6656, 228986.44, 186587.64, 196949, 1405432]
}

df = pd.DataFrame(data)

# 2. Funkcja do generowania wykresu słupkowego
def plot_metric(metric_name, title, ylabel, color):
    # Sortowanie danych malejąco dla lepszej czytelności
    df_sorted = df.sort_values(by=metric_name, ascending=False)
    
    plt.figure(figsize=(10, 6))
    bars = plt.bar(df_sorted['Państwo'], df_sorted[metric_name], color=color)
    
    plt.title(title, fontsize=14)
    plt.ylabel(ylabel, fontsize=12)
    plt.xticks(rotation=45) # Obrócenie nazw państw dla lepszej czytelności
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    
    # Dodanie etykiet z wartościami nad słupkami
    for bar in bars:
        yval = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2, yval, f'{yval:,.2f}', 
                 va='bottom', ha='center', fontsize=9)

    plt.tight_layout()
    plt.show()

# 3. Generowanie trzech wykresów
# Wykres 1: Średnie rozwiązanie
plot_metric('avg_res', 'Średnie rozwiązanie (avg_res)', 'Wartość', 'skyblue')

# Wykres 2: Średnia ilość kroków
plot_metric('avg_num_steps', 'Średnia ilość kroków (avg_num_steps)', 'Liczba kroków', 'salmon')

# Wykres 3: Najlepszy rezultat
plot_metric('best_res', 'Najlepszy rezultat (best_res)', 'Wartość', 'lightgreen')