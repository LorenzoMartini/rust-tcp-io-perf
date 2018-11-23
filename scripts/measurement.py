# Helper to wrap utilities to parse measurments


# Represent a measurement line output by hdrist cdfthe server/client.
class Measurement:

    value = 0
    percentage = 0

    def __init__(self, oline):
        parsed_line = oline.rstrip('\n').replace('(', '').replace(')', '').split(', ')
        self.value = int(parsed_line[0])
        self.percentage = float(parsed_line[1])


# Returns list of measurements from program stdout
def create_measurements_list(output):
    measurements = []
    for line in output:
        # Debug output
        if len(line) > 1 and line[0] == '(':
            measurements.append(Measurement(line))
    return measurements
