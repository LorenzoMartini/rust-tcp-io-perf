import numpy as np
# Helper to wrap utilities to parse measurments


# Represent a measurement line output by the server/client,
# containing number of bytes processed and time it took to process
class Measurement:

    n_bytes = 0
    time_us = 0

    def __init__(self, oline):
        parsed_line = oline.rstrip('\n').replace('[', '').replace(']', '').split(',')
        self.n_bytes = int(parsed_line[0])
        self.time_us = int(parsed_line[2].replace('us', ''))


# Returns list of measurements from program stdout
def create_measurements_list(output):
    measurements = []
    for line in output:
        # Debug output
        if line[0] == '[':
            measurements.append(Measurement(line))
    return measurements


# TODO to be removed in exchange for a summary in rust
# Grabs a stable portion of the measurements and outputs a summary
def print_measurements_summary(measurements, program, machine_id=''):

    tot_len = len(measurements)
    i = 0

    if program == 'rust-tcp-bw':
        # Compute integral in steady state
        n_bytes = 0
        time = 0
        for measurement in measurements:
            if (i > (tot_len / 3)) and (i < (tot_len * 2 / 3)):
                n_bytes += measurement.n_bytes
                time += measurement.time_us
            i += 1

        print((machine_id + ': ' if machine_id != '' else '') + 'Estimated available bandwidth : ' +
              str(n_bytes / time) + 'MB/s')

    else:
        times_us = []
        for measurement in measurements:
            times_us.append(measurement.time_us)

        print('Roundtrip numbers:\nP99: {}us\nP95: {}us\nP75: {}us\nP50: {}us\nP25: {}us'.format(
            np.percentile(times_us, 99),
            np.percentile(times_us, 95),
            np.percentile(times_us, 75),
            np.percentile(times_us, 50),
            np.percentile(times_us, 25)
        ))
