import matplotlib.pyplot as plt
import numpy as np
import measurement
import sys

# Run python plotter.py <results_file> where results file is the output of server.rs.
# Shows sampled values and computes bandwidth


# Plot given measurments samples
def plot_measurements(measurements, program):
    # TODO add cdf
    x_axis = np.arange(0, len(measurements))
    y_axis = []
    for sample in measurements:
        y_axis.append((sample.n_bytes / sample.time_us) if program == 'rust-tcp-bw' else sample.time_us)

    plt.plot(x_axis, y_axis)
    plt.grid(True)
    plt.title('Network effective bandwidth' if program == 'rust-tcp-bw' else 'Latency')
    plt.xlabel('Sample #')
    plt.ylabel('MB/s' if program == 'rust-tcp-bw' else 'us')
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
