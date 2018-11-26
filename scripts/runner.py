import paramiko
import plotter
import time
import measurement
import sys

# Run python runner.py <config_file_path> to execute. Config file must be like 'default_config.config'
# It's important to include the appropriate ssh keys to be able to run ssh correctly.


CONST_SERVER_COMPILE = 'source $HOME/.cargo/env && cd rust-tcp-io-perf/{} && cargo build --bin server --release'
CONST_CLIENT_COMPILE = 'source $HOME/.cargo/env && cd rust-tcp-io-perf/{} && cargo build --bin client --release'
CONST_RUN_SERVER = './rust-tcp-io-perf/{}/target/release/server'
CONST_RUN_CLIENT = './rust-tcp-io-perf/{}/target/release/client'


# Default configuration
def default_config():
    return {
        'PROGRAM': 'rust-tcp-bw',
        'SERVER_ADDRESS': 'euler01',
        # Aliasing is necessary, the remote machines may see themselves with different addresses
        'SERVER_ADDRESS_ALIAS': 'euler01',
        'CLIENT_ADDRESS': 'euler02',
        'CLIENT_ADDRESS_ALIAS': 'euler02',
        'KBYTES': '10000',
        'ROUNDS': '10000',
        'PORT': '7878',
        'USERNAME': 'lmartini',
        'KEY_FILENAME': '/home/lorenzo/.ssh/euler0x-key',
        'PLOT': '0',
        'VERBOSE': '1'
    }


# Read config file values into a dictionary and returns it
def parse_config():
    if len(sys.argv) < 2:
        return default_config()

    config = {}
    with open(sys.argv[1], 'r') as ifile:
        for line in ifile.readlines():
            if line[0] == '#':
                continue
            split_line = line.rstrip('\n').split('=')
            config[split_line[0]] = split_line[1]
    return config


CONFIG = parse_config()


# Method to edit the fixed commands to point at the right program
def parse_command(command):
    return command.replace('{}', CONFIG['PROGRAM'])


# Returns the command to run the client with the specified server.
# Need the address the client knows of the server to connect.
def run_client_command(server_address_alias):
    return parse_command(CONST_RUN_CLIENT) + ' -a ' + server_address_alias + args()


# Returns the command to run the server
def run_server_command():
    return parse_command(CONST_RUN_SERVER) + args()


# Pack args in command line
def args():
    return ' -p ' + CONFIG['PORT'] + ' -k ' + CONFIG['KBYTES'] + ' -r ' + CONFIG['ROUNDS']


# Print given stdout iterator and collects results in a list that is returned when the program completes
def print_and_collect_out(stdout, machine_id=''):
    # See output from server. Store out to analyze it and eventually plot it later
    out = []
    for line in stdout:
        # This is necessary because for some awkward reason if std is not consumed it gets blocked
        lstrip = line.rstrip('\n')
        if CONFIG['VERBOSE'] == '1':
            print(machine_id + ': ...' + lstrip)
        out.append(lstrip)

    _ = stdout.channel.recv_exit_status()
    print(machine_id + ' finished')
    return out


# Compiles given program and creates executable
def cargo_compile(ssh, compiling_command):
    print('Compiling executable...')
    _, stdout, stderr = ssh.exec_command(compiling_command)
    exit_status = stdout.channel.recv_exit_status()

    if exit_status == 0:
        print('Compilation successful, starting...')
        return 0

    print('Error while compiling:\n')
    for line in stderr:
        print(line.strip('\n'))
    return exit_status


# Compile the executables on server and client
def compile_source(server, client):
    if (cargo_compile(server, parse_command(CONST_SERVER_COMPILE)) != 0 or
            cargo_compile(client, parse_command(CONST_CLIENT_COMPILE))):
        return -1
    return 0


# Connects client host to given server with name derived from command line args (or default) and given id
def setup_connection(machine_address):
    ssh = paramiko.SSHClient()
    ssh.load_system_host_keys()
    ssh.connect(machine_address,
                username=CONFIG['USERNAME'], key_filename=CONFIG['KEY_FILENAME'])
    print('...Connected to ' + machine_address)
    return ssh


# Connect to remote machines to execute experiments
def connect_remote(server_address, client_address):
    # Connect to remote machines
    print('\nSetting up connection with servers...\n')
    server = setup_connection(server_address)
    client = setup_connection(client_address)
    print('\nConnected to all the machines!')
    return server, client


# Run server and client and returns stdout of server if rust-tcp-bw or client if rust-tcp-latency
def run_remote(server, client, server_address_alias):
    _, sout, serr = server.exec_command(run_server_command())
    time.sleep(5)
    _, cout, cerr = client.exec_command(run_client_command(server_address_alias))

    # See output from client (in progress) and servers and make sure they are done
    client_out = print_and_collect_out(cout, 'client')
    server_out = print_and_collect_out(sout, 'server')

    # Print err
    for line in serr:
        print('Server ERR: ' + line)

    return server_out if CONFIG['PROGRAM'] == 'rust-tcp-bw' else client_out


# Does the job of connecting, compiling and analyzing output
def run(server_address, client_address, server_address_alias):
    server, client = None, None
    output = None
    try:
        server, client = connect_remote(server_address, client_address)
        if compile_source(server, client) != 0:
            print("Compiling error")
            return
        output = run_remote(server, client, server_address_alias)
    finally:
        if server:
            server.close()
        if client:
            client.close()

    if not output:
        print("No Output... Something went wrong")
        return

    measurements = measurement.create_measurements_list(output)

    if CONFIG['PLOT'] == '1':
        plotter.plot_measurements(measurements, CONFIG['PROGRAM'])


def main():
    run(CONFIG['SERVER_ADDRESS'], CONFIG['CLIENT_ADDRESS'], CONFIG['SERVER_ADDRESS_ALIAS'])


if __name__ == "__main__":
    main()
