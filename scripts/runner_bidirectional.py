import runner
import time
import measurement
import plotter

# Run python runner_bidirectional.py <config_file_path> to execute. Config file must be like 'default_config.config'
# It's important to include the appropriate ssh keys to be able to run ssh correctly.

# Similar to runner, but runs benchmark both ways: have a sender and a receiver on each of the 2 machines and
# test 2-way communication.
# Note this makes sense only if PROGRAM=rust-tcp-bw


# Run both servers and both clients and returns stdout of both
def run_remote(server, client, server2, client2, server_address_alias, server2_address_alias):
    _, sout, serr = server.exec_command(runner.run_server_command())
    _, sout2, serr2 = server2.exec_command(runner.run_server_command())
    time.sleep(5)
    _, cout, cerr = client.exec_command(runner.run_client_command(server_address_alias))
    _, cout2, cerr2 = client2.exec_command(runner.run_client_command(server2_address_alias))

    runner.print_and_collect_out(cout, 'client1')
    runner.print_and_collect_out(cout2, 'client2')

    out1 = runner.print_and_collect_out(sout, 'server1')
    out2 = runner.print_and_collect_out(sout2, 'server2')

    for line in serr:
        print(line)
    for line in serr2:
        print(line)

    return out1, out2


# Does the job of connecting, compiling and analyzing output
def run(server_address, client_address, server_address_alias, client_address_alias):
    server, client = None, None
    output, output2 = None, None
    try:
        server, client = runner.connect_remote(server_address, client_address)
        server2, client2 = runner.connect_remote(client_address, server_address)
        if runner.compile_source(server, client) != 0:
            print("Compiling error")
            return
        time.sleep(2)
        output, output2 = run_remote(server, client, server2, client2, server_address_alias, client_address_alias)
    finally:
        if server:
            server.close()
        if client:
            client.close()

    if not output:
        print("No Output... Something went wrong")
        return
    if not output2:
        print("No Output2... Something went wrong")
        return

    measurements = measurement.create_measurements_list(output)
    measurements2 = measurement.create_measurements_list(output2)

    if runner.CONFIG['PLOT'] == '1':
        plotter.plot_measurements(measurements, runner.CONFIG['PROGRAM'])
        plotter.plot_measurements(measurements2, runner.CONFIG['PROGRAM'])


def main():
    if runner.CONFIG['PROGRAM'] != 'rust-tcp-bw':
        print('This script works only for rust-tcp-bw')
        exit(-1)
    run(runner.CONFIG['SERVER_ADDRESS'], runner.CONFIG['CLIENT_ADDRESS'],
        runner.CONFIG['SERVER_ADDRESS_ALIAS'], runner.CONFIG['CLIENT_ADDRESS_ALIAS'])


if __name__ == "__main__":
    main()
