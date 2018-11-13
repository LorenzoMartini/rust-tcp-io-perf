import matplotlib.pyplot as plt
import numpy as np


def plot_measurements(measurements):
    x_axis = np.arange(0, len(measurements))
    gbs = []
    for measurement in measurements:
        gbs.append(measurement.n_bytes / measurement.time_us)

    plt.plot(x_axis, gbs)
    plt.grid(True)
    plt.title('Network effective bandwidth')
    plt.xlabel('Sample #')
    plt.ylabel('MB/s')
    # plt.savefig('EffectiveBW.svg', format='svg')
    plt.show()
