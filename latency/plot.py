import matplotlib.pyplot as plt
import pandas as pd

data = pd.read_csv('results.csv')
plt.figure()
data.set_index('SIZE')
print(data)
data.plot(x='SIZE')
plt.savefig('plot.png')