import matplotlib.pyplot as plt
import measurement
import sys

# Run python plotter.py <results_file> where results file is the output of server.rs (bw) or client.rs (latency)
# Shows sampled values and computes bandwidth


# Plot given measurments samples
def plot_measurements(measurements, program):
    x_axis = []
    y_axis = []
    for sample in measurements:
        x_axis.append(sample.value)
        y_axis.append(sample.percentage)

    plt.plot(x_axis, y_axis)
    plt.grid(True)
    plt.title('Network effective bandwidth' if program == 'rust-tcp-bw' else 'Latency')
    plt.xlabel('MB/s' if program == 'rust-tcp-bw' else 'ns')
    plt.ylabel('%')
    plt.loglog()
    plt.show()


def main():
    measurement_file = sys.argv[1]
    program = sys.argv[2]
    with open(measurement_file, 'r') as ifile:
        measurments = measurement.create_measurements_list(ifile.readlines())
        plot_measurements(measurments, program)


if __name__ == "__main__":
    main()
