import matplotlib.pyplot as plt
import numpy as np
import measurment
import sys


# Plot given measurments samples
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
    plt.show()


def main():
    measurement_file = sys.argv[1]
    with open(measurement_file, 'r') as ifile:
        measurments = measurment.create_measurements_list(ifile.readlines())
        plot_measurements(measurments)
        measurment.print_measurements_avg(measurments)


if __name__ == "__main__":
    main()
