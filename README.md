<img src="https://github.com/MKP157/Proxide/blob/main/resources/PrOxide_Logo.png" height="150" alt="">

# Proxide: A Bare-bones HTTP Server written in Rust
For our Networking (CS 3893) class, our team decided to see how easily we could implement 
an HTTP server in the language of our choosing. We settled on Rust, as we were all new to it,
and we were eager to teach ourselves its inner workings and decided to learn through as much 
practice as possible.

## Team Members
- Matthew Kenneth Peterson
- Joshua Hickman
- Aiden Manuel

## How it Works
Part of the challenge we had assigned ourselves was to implement our web server as simply as
possible. In doing so, we downright abused many modern web technologies to serves our purposes.
Thus, it operates under a very simplistic control cycle:

First, a `TCPListener` instance from the standard library is set up to listen to all 
incoming HTTP requests (as HTTP uses TCP as its underlying protocol) over **port 9999** of
the local machine. Once one is received, the listener will assign the task of sending out 
a response to one of the available `Threads` from the predefined `ThreadPool`. It does this 
by unwrapping the HTTP request from plain text, and extracting the HTTP resource that the
client is requesting. With the resource in hand, it cross-references it to find a matching 
entry in "proxide_manifest.csv" (which is converted into a quick-access Hashmap before booting, 
for performance reasons). If found, it will return that file as a stream of bytes, over a 
`TCPStream`. If the requested resource does not in fact exist, however, then Proxide 
will route you to its 404 page.

## Running Proxide
The only prerequisite to using Proxide is having Rust installed. 
If you don't have it yet, the Rust installer can be [found here](https://www.rust-lang.org/tools/install).

To run Proxide, navigate to the folder containing the project's `Cargo.toml` file 
with your system's terminal, and type `cargo run`. This will both compile the project,
and run it directly afterward. If no changes to the code are made, then Cargo won't
recompile and simply run instead.

To run a build with compilation optimizations, run `cargo run --release`.

Once Proxide is running, nothing will actually happen until you try and connect to it. 
From your web browser of choice, navigate to http://127.0.0.1:9999/. Whatever is defined 
within the `home.html` file in the resources folder should show up here; as provided, the
source code in this repository has a simple HTTP site, which we used as a live demonstration
when presenting our project to the class. As you navigate your Proxide website, watch the 
terminal as incoming requests are made by the client. It's fascinating!
