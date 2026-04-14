import matplotlib.pyplot as plt
import pandas as pd

# 1. Przygotowanie nowych danych
data = {
    'Państwo': ['Oman', 'Dżibuti', 'Irlandia', 'Egipt', 'Tanzania', 'Kanada'],
    'avg_res': [140871.20, 11354.00, 405960.40, 296658.20, 320460.60, 2251333.00],
    'avg_num_steps': [3010.20, 28.80, 12356.60, 10888.80, 4497.80, 7219.80],
    'best_res': [136735, 8644, 393356, 276573, 287197, 2196570]
}

df = pd.DataFrame(data)

# 2. Funkcja pomocnicza do tworzenia wykresów
def plot_metric(metric_name, title, ylabel, color):
    # Sortowanie danych malejąco dla lepszej prezentacji zależności
    df_sorted = df.sort_values(by=metric_name, ascending=False)
    
    plt.figure(figsize=(10, 6))
    bars = plt.bar(df_sorted['Państwo'], df_sorted[metric_name], color=color)
    
    plt.title(title, fontsize=14)
    plt.ylabel(ylabel, fontsize=12)
    plt.xticks(rotation=45)
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    
    # Dodanie etykiet nad słupkami
    for bar in bars:
        yval = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2, yval, f'{yval:,.2f}'.replace(',', ' '), 
                 va='bottom', ha='center', fontsize=9)

    plt.tight_layout()
    plt.show()

# 3. Generowanie trzech wykresów
# Wykres 1: Średnie rozwiązanie (zgodnie z intuicją - największe dla Kanady)
plot_metric('avg_res', 'Średnie rozwiązanie (avg_res)', 'Wartość', 'skyblue')

# Wykres 2: Średnia liczba kroków (zależy od złożoności problemu)
plot_metric('avg_num_steps', 'Średnia liczba kroków (avg_num_steps)', 'Liczba kroków', 'salmon')

# Wykres 3: Najlepszy rezultat
plot_metric('best_res', 'Najlepszy rezultat (best_res)', 'Wartość', 'lightgreen')