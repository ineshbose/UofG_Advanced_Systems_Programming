# Concurrent Programming in Rust

## Introduction

The Advanced Systems Programming (H/M) course uses the [Rust programming language](https://rust-lang.org/) to illustrate several advanced topics in systems programming. One such topic is concurrent programming, since a key feature needed to make effective use of a modern multicore processor is safe concurrency.

This exercise explores concurrent programming in Rust. **This is an assessed exercise that is worth 10% of the marks for this course.**

## Networked Applications and the DNS

Networked applications frequently operate in a client-server manner. Clients connect to the server, make a request, wait for a response to be received, and then disconnect. The server is generally identified by a domain name, such as `www.glasgow.ac.uk`, and clients must perform a DNS lookup to resolve that domain name to an IP address before they can establish a connection to the server.

A DNS lookup can return multiple IP addresses for a domain name. This can occur when multiple hosts serve requests for a popular service, spreading the load between them, or when a server is reachable using both IPv6 and IPv6.

Write a program, `dnslookup`, using the Rust programming language, that takes a list of one or more domain names on the command line, and performs a DNS lookup for each name. After each DNS lookup, the program should print out the list of IP addresses returned, with each address prefixed with the domain name and address type. For example, if asked to resolve the name `www.google.com`, the program might print the following:

```sh
$ ./dnslookup www.google.com
www.google.com IPv6 2a00:1450:4009:801::2004
www.google.com IPv4 172.217.23.36
$
```

The student Linux servers (`stlinux01.dcs.gla.ac.uk` to `stlinux08.dcs.gla.ac.uk`) have IPv6 enabled, and are able to resolve IPv6 addresses. Rust provides implementations of the trait `std::net::ToSocketAddrs` in the standard library that can be used to perform DNS lookups; do not use an external crate.

This part of the exercise is preparatory work. The code for the `dnslookup` program is not required to be submitted, and it is not assessed.

## Making Sequential Connections

Once a client has resolved the domain name for the server into a list of IP addresses, it tries to establish a connection. The way this is typically taught is that the client assumes the DNS lookup returns the addresses in order of preference, and tries to connect to each address in turn, stopping when a connection is successfully established.

The `dnslookup` example given previously showed an IPv6 and an IPv4 address being returned from the DNS lookup. In this case, the client would first try to connect to the IPv6 address of the server, since this was first in the list of addresses returned by the DNS lookup. Then, if the connection to the IPv6 address failed, it would try to connect to the IPv4 address.

Using the Rust programming language, write a program, `seqcon`, to perform a DNS lookup for a domain name given on the command line, and to establish a connection that server. The program should loop over the list of IP addresses returned from the DNS lookup, and try to connect to each address in turn on TCP port 80 (the HTTP port). This should continue until either a connection is successfully established, or all of the IP addresses of the server have been tried.

Once it successfully establishes a connection, the program should print of the IP address to which it connected. Then, it should send the following text to the server:

```txt
GET / HTTP/1.1
Host: hostname
```

replacing `hostname` with the DNS name of the host to which it has made the connection. Each line *must* end with a carriage return and new line ("`\r\n`"). There *must* be a blank line at the end of the request. Following this, the server should read and print all data returned from the server until the connection is closed by the server. It should then close the connection and exit. The `seqcon` program is a trivial HTTP client: it fetches and displays the main page of the specified web server.

It is likely that the code written for the `dnslookup` program will be useful when developing the `seqcon` program. This part of the exercise is preparatory work; it does not need to be submitted, and it is not assessed.

## Making Concurrent Connections

The problem with trying to connect to each address of a server in turn is that it can be slow if the server is not reachable on some addresses. For example, in some cases it can take tens of seconds for a connection request to timeout if blocked by a firewall. A better approach is to try to connect to all the addresses concurrently, proceed with the first that successfully connects, and close all the other connections. This uses more resources, as it attempts to open multiple connections in parallel, but can be much faster to connect.

Using the Rust programming language, write a program, `concon`, to perform a DNS lookup for a domain name given on the command line, and to establish a connection to that server using concurrent connection requests. The `concon` program should be structured using three types of thread that communicate using `std::sync::mpsc::channel` channels.

- The **main thread** should perform the DNS lookup for the domain name specified on the command line. It should create a single connected client thread, plus a connection attempt thread for each IP address returned by the DNS lookup. These threads should be connected using MPSC channels. *After all the threads have been created*, the main thread should send a message to each connection attempt thread instructing it to attempt to connect to one of the IP addresses returned from the DNS lookup.
- Each of the **connection attempt threads** listens on the channel connecting it to the main thread, waiting to receive a message containing an IP address to which it should connect. When such a message is received, it tries to connect to port 80 on that address using `std::net::TcpStream::connect`. If the connection succeeds, it sends the resulting `TcpStream` object down the channel to the connected client thread and then exits. If the connection does not succeed, it simply exits.
- The **connected client thread** waits for connections (i.e., `TcpStream` objects) sent from the connection attempt threads. The connected client thread prints the peer address of the first `TcpStream` it receives, then sends a GET request to the server using that TCP stream, prints the data returned, and closes the connection (as was done in the `seqcon` program). For all other connections, the connected client thread simply closes the connection without sending a GET request.

The `concon` program will use a separate channel to connect the main thread to each connection attempt thread, and a single channel (where the transmit side has been duplicated using `clone()`) to link the connection attempt threads to the connected client thread.

The goal is to demonstrate the concurrency primitives in Rust, showing how threads can be spawned and how message passing can be used to pass data between threads. **The `concon` program must be submitted for assessment.**

## Submission

The source code for the `concon` program must be submitted for assessment. Create a directory named `asp-ex2-GUID`. Put a copy of the source code for the `concon` program into this directory. Do not include compiled binaries (i.e., run `cargo clean` before copying the files into the `asp-ex2-GUID` directory). Create a zip archive of the directory, as a file called `asp-ex2-GUID.zip`. Submit the zip archive via Moodle.

As an example, if the GUID was 1234567a, the following steps would be performed to create the zip archive, after copying the source code into the `asp-ex2-1234567a` directory:

```txt
    zip -r asp-ex2-1234567a.zip asp-ex2-1234567a/
```

*Check carefully that the zip archive extracts into the correct subdirectory, contains only the requested files, and has the correct filename.*

## Assessment and Marking Scheme

This exercise is worth 10% of the mark for this course. The solution must be submitted before 10:00am on 6 March 2023. Following the University code of assessment, late submissions will be accepted for up to 5 working days beyond this due date. Late submissions will receive a two band penalty for each working day, or part thereof, the submission is late. Submissions that are received more than five working days after the due date will be awarded a band of H.

Submissions that are not made via Moodle, that have the wrong filename, that have a zip archive that extracts into the wrong directory or that otherwise do not follow the submissions instructions will be subject to a two band penalty. This penalty is in addition to any late submission penalty. *This penalty will be strictly enforced.*

Marks will be awarded based on inspection of the source code, and on the results of compiling and running the `concon` program as follows:

- Up to [2 marks] for compiling with no errors and no warnings using the version of the Rust compiler installed on the student Linux servers (`stlinux01` - `stlinux08`).

- Up to [8 marks] for successful operation of the program, including the ability to perform DNS lookups and to race IPv4 and IPv6 connection attempts. The program will be run using the following commands on one of the student Linux servers:

```txt
    unzip asp-ex2-$GUID.zip
    cd asp-ex2-$GUID
    cargo run $DNSNAME
```

where the variables `$GUID` and `$DNSNAME` will be set to the GUID and the domain name to look-up. Several domains will be tested, with a mixture of IPv4 and IPv6 addresses.

- Up to [10 marks] for the design and implementation of the main thread. This will include correct initialisation of the other threads, correct use of channels to communicate between the threads, correctly performing the DNS lookup, and correctly passing the IP addresses to the connection attempt threads.

- Up to [6 marks] for the design and implementation of the connection attempt threads. This will include correct handling the input and output channels, correctly establishing a connection and pasing the `TcpStream` to the connected client thread.

- Up to [4 marks] for the design and implementation of the connected client thread. This will include looping to receive multiple connections from the connection attempt threads, printing the peer address of the first successful connection, sending the HTTP GET request and receiving the response, and closing connections when no longer needed.

The result will be numeric mark out of 30. This numeric mark will be converted to a percentage, then the percentage mark will be converted to a band on the 22-point University of Glasgow scale using the standard translation table for the School of Computing Science. Any applicable penality for late submission and/or for not following submission instructions will then be applied, and a band will be returned. A brief written justification for the band will also be supplied.
