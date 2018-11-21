# rust-tcp-ayce
Send big stuff over the network with Rust tcp sockets

## Instructions

### Running .rs directly
#### Prerequisites
- Have Rust and Cargo correctly installed on your machine
#### Instructions

- Go on the machine where you wanna launch the server (or ssh into it)
- Open a terminal
- `cd` into the inner `rust-tcp-ayce` folder
- Run `cargo run --bin server --release` (or compile and run, meaning `cargo build --bin server --release` and once compiled `./target/release/server`. You can specify the port you wanna listen on with `-p <port_number>`


- Go on the machine where you wanna launch the client (or ssh into it)
- Open a terminal
- `cd` into the inner `rust-tcp-ayce` folder
- Run `cargo run --bin client --release` (or compile and run, meaning `cargo build --bin client --release` and once compiled `./target/release/client`. You can specify a bunch of parameters. Run the program with the `-h` option to see available params

If you want to test the two-way communication, then setup 2 servers and then run the 2 clients together.

### Running via scripts

I have provided scripts to run the benchmarks automatically.

#### runner.py
This scripts will run the benchmark remotely with a client and a server

#### runner_bidirectional.py
This scripts will run the benchmark remotely with 2 clients and 2 servers (a client and a server on each of the 2 machines) in order to test bidirectional communication.
