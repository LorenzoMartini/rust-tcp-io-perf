# Helper to wrap utilities to parse measurments


# Represent a measurement line output by the server, containing number of bytes processed and time it took to process
class Measurement:

    n_bytes = 0
    time_us = 0

    def __init__(self, oline):
        parsed_line = oline.rstrip('\n').replace('[', '').replace(']', '').split(',')
        self.n_bytes = int(parsed_line[0])
        self.time_us = int(parsed_line[2].replace('us', ''))


# Returns list of measurments from program stdout
def create_measurements_list(output):
    measurements = []
    for line in output:
        # Debug output
        if line[0] == '[':
            measurements.append(Measurement(line))
    return measurements


# Grabs a stable portion of the measurements and outputs the average
def print_measurements_avg(measurements):
    tot_len = len(measurements)
    i = 0

    # Compute integral in steady state
    n_bytes = 0
    time = 0
    for measurement in measurements:
        if (i > (tot_len / 3)) and (i < (tot_len * 2 / 3)):
            n_bytes += measurement.n_bytes
            time += measurement.time_us
        i += 1

    print('AVG bandwidth use: ' + str(n_bytes / time) + 'MB/s')