import matplotlib.pyplot as plt
import pandas as pd

# 1. Przygotowanie danych
data = {
    'Państwo': ['Oman', 'Dżibuti', 'Irlandia', 'Egipt', 'Tanzania', 'Kanada'],
    'avg_res': [93088.56, 6738.29, 225023.76, 184201.66, 193849.79, 1414883.52],
    'avg_num_steps': [532.51, 12.00, 2161.57, 1894.33, 784.04, 1146.57],
    'best_res': [92155, 6656, 224454, 183905, 192973, 1410394]
}

df = pd.DataFrame(data)

# 2. Funkcja do generowania wykresu
def plot_metric(metric_name, title, ylabel, color):
    # Sortowanie danych malejąco dla lepszej czytelności
    df_sorted = df.sort_values(by=metric_name, ascending=False)
    
    plt.figure(figsize=(10, 6))
    bars = plt.bar(df_sorted['Państwo'], df_sorted[metric_name], color=color)
    
    plt.title(title, fontsize=14)
    plt.ylabel(ylabel, fontsize=12)
    plt.xticks(rotation=45)
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    
    # Dodanie etykiet z wartościami nad słupkami
    for bar in bars:
        yval = bar.get_height()
        # Formatowanie liczb: spacja jako separator tysięcy, 2 miejsca po przecinku
        plt.text(bar.get_x() + bar.get_width()/2, yval, f'{yval:,.2f}'.replace(',', ' '), 
                 va='bottom', ha='center', fontsize=9)

    plt.tight_layout()
    plt.show()

# 3. Wywołanie generowania wykresów
# Wykres 1: Średnie rozwiązanie
plot_metric('avg_res', 'Średnie rozwiązanie (avg_res)', 'Wartość', 'skyblue')

# Wykres 2: Średnia liczba kroków
plot_metric('avg_num_steps', 'Średnia liczba kroków (avg_num_steps)', 'Liczba kroków', 'salmon')

# Wykres 3: Najlepszy rezultat
plot_metric('best_res', 'Najlepszy rezultat (best_res)', 'Wartość', 'lightgreen')