import matplotlib.pyplot as plt
import pandas as pd

data = pd.read_csv('results.tsv')
plt.figure()
data.set_index('SIZE')
print(data)
data.plot(x='SIZE', y=['READ','WRITE','READ/WRITE'])
plt.savefig('plot.png')