import matplotlib.pyplot as plt
import measurement
import sys

# Run python plotter.py <results_file> where results file is the output of server.rs (bw) or client.rs (latency)
# Shows sampled values and computes bandwidth

CONST_LINESTYLES=['-', '--', '-.', ':', '-']
CONST_DEFAULT_MEASUREMENTS = (
    '/home/lorenzo/thesis/experiments/20181126/latency_benchmark/latency_euler_n2_boost.txt||euler_n2,'
    '/home/lorenzo/thesis/experiments/20181126/latency_benchmark/latency_fdr_n2_boost.txt||fdr_n2,'
    '/home/lorenzo/thesis/experiments/20181126/latency_benchmark/latency_euler_n1_boost.txt||euler_n1,'
    '/home/lorenzo/thesis/experiments/20181126/latency_benchmark/latency_fdr_n1_boost.txt||fdr_n1')
CONST_DEFAULT_PROGRAM = 'rust-tcp-latency'


# Plot given measurments samples
def plot_cdf(measurements, label, i=-1):
    x_axis = []
    y_axis = []
    for sample in measurements:
        x_axis.append(sample.value)
        y_axis.append(sample.percentage)

    plt.plot(x_axis, y_axis, linestyle=CONST_LINESTYLES[i], label=label)
    if i == -1:
        plt.grid(True)
        plt.loglog()
        plt.show()


# Measurements file names should be in the format: <<file>||<label>>,<<file>||<label>> etc
def plot_measurements(measurement_files, program):
    i = 0
    for measurement_file in measurement_files.split(','):
        file_name = measurement_file.split('||')[0]
        label = measurement_file.split('||')[1]
        with open(file_name, 'r') as ifile:
            measurments = measurement.create_measurements_list(ifile.readlines())
            plot_cdf(measurments, label, i)
        i += 1

    plt.grid(True)
    plt.title('Network effective bandwidth' if program == 'rust-tcp-bw' else 'Latency')
    plt.xlabel('MB/s' if program == 'rust-tcp-bw' else 'ns')
    plt.ylabel('%')
    plt.loglog()
    plt.legend(loc='lower left')
    plt.show()


def main():
    measurement_files = sys.argv[1] if len(sys.argv) > 1 else CONST_DEFAULT_MEASUREMENTS
    program = sys.argv[2] if len(sys.argv) > 2 else CONST_DEFAULT_PROGRAM
    plot_measurements(measurement_files, program)


if __name__ == "__main__":
    main()
