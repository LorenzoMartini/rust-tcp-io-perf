import matplotlib.pyplot as plt
import numpy as np
import measurement
import sys

# Run python plotter.py <results_file> where results file is the output of server.rs.
# Shows sampled values and computes bandwidth


# Plot given measurments samples
def plot_measurements(measurements, program):
    # TODO add cdf and specialize
    x_axis = np.arange(0, len(measurements))
    gbs = []
    for sample in measurements:
        gbs.append(sample.n_bytes / sample.time_us)

    plt.plot(x_axis, gbs)
    plt.grid(True)
    plt.title('Network effective bandwidth')
    plt.xlabel('Sample #')
    plt.ylabel('MB/s')
    plt.show()


def main():
    measurement_file = sys.argv[1]
    program = sys.argv[2]
    with open(measurement_file, 'r') as ifile:
        measurments = measurement.create_measurements_list(ifile.readlines())
        plot_measurements(measurments, program)
        measurement.print_measurements_summary(measurments, program)


if __name__ == "__main__":
    main()
