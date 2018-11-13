import paramiko

CONST_SERVER_ADDRESS = "euler01"
CONST_CLIENT_ADDRESS = "euler02"
CONST_PORT = "7878"
CONST_USERNAME = "lmartini"
CONST_KEY_FILENAME = "/home/lorenzo/.ssh/euler0x-key"
CONST_SERVER_COMPILE = "source $HOME/.cargo/env && cd rust-tcp-ayce/rust-tcp-ayce && cargo build --bin server --release"
CONST_CLIENT_COMPILE = "source $HOME/.cargo/env && cd rust-tcp-ayce/rust-tcp-ayce && cargo build --bin client --release"
CONST_RUN_SERVER = "./rust-tcp-ayce/rust-tcp-ayce/target/release/server -k 100000"
CONST_RUN_CLIENT = "./rust-tcp-ayce/rust-tcp-ayce/target/release/client -a euler01 -k 100000"


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
    cargo_compile(server, CONST_SERVER_COMPILE)
    cargo_compile(client, CONST_CLIENT_COMPILE)


# Connects client host to given server with name derived from command line args (or default) and given id
def setup_connection(machine_address):
    ssh = paramiko.SSHClient()
    ssh.load_system_host_keys()
    ssh.connect(machine_address,
                username=CONST_USERNAME, key_filename=CONST_KEY_FILENAME)
    print('...Connected to ' + machine_address)
    return ssh


# Connect to remote machines to execute experiments
def connect_remote():
    # Connect to remote machines
    print('\nSetting up connection with servers...\n')
    server = setup_connection(CONST_SERVER_ADDRESS)
    client = setup_connection(CONST_CLIENT_ADDRESS)
    print('\nConnected to all the machines!')
    return server, client


# Run server and client and returns somthing TODO{@lmartini}
def run_remote(server, client):
    _, sout, serr = server.exec_command(CONST_RUN_SERVER)
    _, cout, cerr = client.exec_command(CONST_RUN_CLIENT)
    _ = sout.channel.recv_exit_status()
    _ = cout.channel.recv_exit_status()
    for line in serr:
        print(line)
    return sout


def main():
    server, client = None, None

    try:
        server, client = connect_remote()
        compile_source(server, client)
        for line in run_remote(server, client):
            print(line)

    finally:
        server.close()
        client.close()


if __name__ == "__main__":
    main()
