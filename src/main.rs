use std::io::{BufRead, Write};

/****************************************
 *      Super Simple Rust Webserver
 * --------------------------------------
 * This code was written by Laurence Tratt
 * in a computerphile video published on 
 * Feb 22 2024 called "Coding a Web Server
 * in 25 lines - Computerphile". I have
 * gone through and commented it based on
 * how he describes it in the video, to
 * have as a resource to look back on.
 * --------------------------------------
 *      Aiden Manuel - Feb 25 2024
 ****************************************/

fn main() {
    // This is our listener, which is using TCP to listen for connection
    // requests at some IP address on some port
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    
    // As something comes in, we're going to listen to what they have
    // to say, and potentially give something back to them
    for mut stream in listener.incoming().flatten() {
        // Inside this loop, someone has connected.
        
        // You can kind of think of this line of code as if it were
        // a scanner in Java. That's basically what it's doing.
        let mut rdr = std::io::BufReader::new(&mut stream);

        /* [[[[[[[[[[[[[[ THE LISTEN LOOP ]]]]]]]]]]]]] */
        // This loop will get every string that the listener
        // hears, and print them to the terminal. If it hears
        // an empty line, we break out of the loop.
        loop {
            let mut l = String::new();
            rdr.read_line(&mut l).unwrap();
            if l.trim().is_empty() {break;}
            print!("{l}");
        }
        /* [[[[[[[[[[[[[[[[[[[[[[|]]]]]]]]]]]]]]]]]]]]] */

        /*         --===IMPORTANT TO NOTE===--
         * If we look at the output of that listen, we will
         * see the first line when someone tries to connect
         * to the IP and port (try in browser) is:
         *          GET / HTTP/1.1
         * "GET" means they are requesting some resource.
         * 
         * "HTTP/1.1" is the HTTP protocol being used.
         * 
         * That lonely "/" character is actually what we
         * call a resource, and in this case points to
         * nothing. However, if we had some url with pages
         * on it, that resource would be asking for some
         * specific page.
         */

        /* [[[[[[[[[[[[[[[[ THE RESPONSE ]]]]]]]]]]]]]]] */
        // This is the section where we output our response
        // to the client. In this case we are writing "Hello!"
        // The start of the string "HTTP/1.1 200 OK\r\n\r\n"
        // is just to tell HTTP that we received the request
        // okay and we are sending something back.
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").unwrap();
        /* [[[[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]] */
    }
}