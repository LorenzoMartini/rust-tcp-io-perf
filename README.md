# rust-tcp-ayce

The purpose of this is to benchmark how much bandwidth we effectively have available on a communication channel.
To test that, we run a server and a client at the two ends of the channel, and start sending big quantities of data from one end to the other. On the server side we can then measure how much data we have received in how much time and derive an approximation of the bandwidth.

We take TCP sockets and Rust to benchmark this (only measuring during stable state).

## Instructions

### Running .rs directly
#### Prerequisites
- Have Rust and Cargo correctly installed on your machine
#### Instructions

1) Run server
- Go on the machine where you wanna launch the server (or ssh into it)
- Open a terminal
- `cd` into the inner `rust-tcp-ayce` folder
- Run `cargo run --bin server --release` (or compile and run, meaning `cargo build --bin server --release` and once compiled `./target/release/server`. You can specify the port you wanna listen on with `-p <port_number>`

2) Run client
- Go on the machine where you wanna launch the client (or ssh into it)
- Open a terminal
- `cd` into the inner `rust-tcp-ayce` folder
- Run `cargo run --bin client --release` (or compile and run, meaning `cargo build --bin client --release` and once compiled `./target/release/client`. You can specify a bunch of parameters. Run the program with the `-h` option to see available params. Make sure you specify the right address and port to connect to the server, using parameters `-a <address> -p <port>`.

You should see the client tracking progress, and when he's done you should see the server printing all the rounds of data in format [n_bytes,time,time_us], followed by a summary with the bandwidth information.

If you want to test the two-way communication, then setup 2 servers and then run the 2 clients together.

### Running via scripts

I have provided scripts to run the benchmarks automatically, <strong>runner.py</strong> and <strong>runner_bidirectional.py</strong>.
For these scripts you will need to edit the configuration file. Change values in `default_config.config` or create a config file in the same format with the same keys and change values

The config file is important because it will contain a bunch of things like machines names, ssh keys, username to use for the ssh, etc.

#### Prerequisites
- Make sure you can ssh into the machines you wanna use for your benchmark
- Make sure you have set up the ssh connections correctly and have the machines in your known host and have the ssh keys somewhere in your pc (or the machine you will start the script from).

#### Instructions
For both scripts, run: `python <program> <config_file_location>`.

##### Scripts
- <strong>runner.py</strong>: This script will run the benchmark remotely with a client and a server
- <strong>runner_bidirectional.py</strong>: This script will run the benchmark remotely with 2 clients and 2 servers (a client and a server on each of the 2 machines) in order to test bidirectional communication.

If in your config file you specify PLOT=1 you will also see a summary plot of the samples. See the <strong>Visualizing results<\strong> chapter for details.

### Visualizing results

#### Prerequisites
- Have matplotlib installed

#### Description

There is an additional runnable tool which is <string>plotter.py<\strong>. It takes the output of server.rs as input and produces a plot of the bandwidth of single samples. This is not very reliable usually, especially when testing two-way communications, given that we may see higher spikes only because we had more stuff waiting to be processed in our server than what was effectively sent in that time frame.
  
#### Instructions
- `python plotter.py <file_with_server.rs_output>.`

Get the file you need by simply redirecting the output of `server.rs` to a file.
