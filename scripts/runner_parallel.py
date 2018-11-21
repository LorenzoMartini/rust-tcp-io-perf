import runner
from threading import Thread
import time
import measurment


# Run server and client and returns stdout of server
def run_remote(server, client, server2, client2, server_address, server2_address):
    print(server_address, server2_address)
    _, sout, serr = server.exec_command(runner.CONST_RUN_SERVER)
    _, sout2, serr2 = server2.exec_command(runner.CONST_RUN_SERVER)
    time.sleep(5)
    _, cout, cerr = client.exec_command(runner.run_client_command(server_address))
    _, cout2, cerr2 = client2.exec_command(runner.run_client_command(server2_address))
    out1 = []
    out2 = []

    for line in cout:
        print('client1: ...' + line.rstrip('\n'))
    for line in cout2:
        print('client2: ...' + line.rstrip('\n'))

    _ = cout.channel.recv_exit_status()
    _ = cout2.channel.recv_exit_status()
    print('clients finished!')

    for line in sout:
        l = line.rstrip('\n')
        print('server1: ...' + l)
        out1.append(l)
    for line in sout2:
        l = line.rstrip('\n')
        print('server2: ...' + l)
        out2.append(l)

    _ = sout.channel.recv_exit_status()
    _ = sout2.channel.recv_exit_status()
    print('servers finished')

    for line in serr:
        print(line)
    for line in serr2:
        print(line)

    return out1, out2


def run(server_address, client_address):
    server, client = None, None
    output, output2 = None, None
    try:
        server, client = runner.connect_remote(server_address, client_address)
        server2, client2 = runner.connect_remote(client_address, server_address)
        if runner.compile_source(server, client) != 0:
            print("Compiling error")
            return
        time.sleep(2)
        output, output2 = run_remote(server, client, server2, client2, server_address, client_address)
    finally:
        if server:
            server.close()
        if client:
            client.close()

    if not output:
        print("No Output... Weird")
        return
    if not output2:
        print("No Output2... Weird")
        return

    measurements = measurment.create_measurements_list(output)
    measurements2 = measurment.create_measurements_list(output2)

    if runner.CONST_PLOT == '1':
        plotter.plot_measurements(measurements)
        plotter.plot_measurements(measurements2)

    measurment.print_measurements_avg(measurements)
    measurment.print_measurements_avg(measurements2)


def main():
    run(runner.CONST_SERVER_ADDRESS, runner.CONST_CLIENT_ADDRESS)


if __name__ == "__main__":
    main()
