use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Write};
use std::error::*;
//use std::hash::Hash;
use csv::Reader;
use lazy_static::lazy_static;
use threadpool::ThreadPool;

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

// This URI is whatever address the server
// will be accessed on BY THE CLIENT.
const _URI: &str = //"127.0.0.1:9999";
    "networking.mkp157.xyz";


/** read_file_as_bytes ****************************************
 * @ Matthew Kenneth Peterson
 * @ Edited Mar 17 2024
 *
 * Uses a given file path and converts the specified file
 * into a raw byte-buffer. This is how files will be served to
 * users.
 *************************************************************/
fn read_file_as_bytes(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let byte_content = fs::read(path)?;
    Ok(byte_content)
}

/** hash_manifest *********************************************
 * @ Matthew Kenneth Peterson
 * @ Edited Mar 17 2024
 *
 * Creates a hash-map for the Proxide manifest CSV. The
 * resulting hash is used by our resolver in order to validate
 * resources' existence on our web server.
 *************************************************************/
fn hash_manifest( manifest: &str )

                // Returns a hashmap vector
                -> Result<Vec<HashMap<String, String>>, Box<dyn Error>>
{
    let mut map_vec = Vec::new();
    let mut rdr = Reader::from_path(manifest)?;

    // Read CSV one line at a time, and only add a line to
    // the hash map if it has exactly 2 entries
    for result in rdr.records() {
        let record = result?;
        if record.len() != 2 {
            continue;
        }

        let mut map = HashMap::new();
        map.insert(record[0].to_string(), record[1].to_string());
        map_vec.push(map);
    }

    // Finalize and make immutable, then return to caller
    let map_vec_final = map_vec;
    println!("{:?}", map_vec_final);
    Ok(map_vec_final)
}

/** resolve ***************************************************
 * @ Matthew Kenneth Peterson
 * @ Edited Mar 17 2024
 *
 * Validates that a given HTTP resource route actually exists
 * on the server. If it indeed does, we want to return the
 * path to the matching file. However, if it doesn't, we'll
 * return an error saying the resource wasn't found.
 *************************************************************/
fn resolve( manifest: & Vec<HashMap<String, String>>,
            resource: & str )

            // Will return the file as a reader, as well as
            // an HTTP status code
            -> ( Vec<u8>, i32)
{
    for entry in manifest.iter() {
        if entry.contains_key(resource) {
            let filename = entry.get(resource).unwrap();
            println!("Data: {}", filename);

            let content = fs::read(filename).unwrap();
            return (content, 200i32)
        }
    }
    // If the server makes it down to this code block, something's gone wrong,
    // so we should send them the error page.
    let content = fs::read("./resources/404/404.html").unwrap();
    return (content, 404i32)
}

////////////////// main ///////////////////

// Generate manifest for valid files.
// Needs to be Static so that different threads can access it.
lazy_static! {
    static ref MANIFEST:Vec<HashMap<String, String>> = hash_manifest("src/proxide_manifest.csv").unwrap();
}


fn main() {


    // Allocate thread pool for simultaneous connections
    let pool = ThreadPool::new(16);

    // This is our listener, which is using TCP to listen for connection
    // requests at some IP address on some port
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    
    // As something comes in, we're going to listen to what they have
    // to say, and potentially give something back to them.
    // This listening is done on an individual thread.
    for mut stream in listener.incoming().flatten() {

        // Using "move" copies any necessary data to the newly awakened thread.
        pool.execute(move || {
            // Inside this loop, someone has connected.

            // You can kind of think of this line of code as if it were
            // a scanner in Java. That's basically what it's doing.
            let mut rdr = std::io::BufReader::new(&mut stream);

            /* [[[[[[[[[[[[[[ THE LISTEN LOOP ]]]]]]]]]]]]] */
            // This loop will get every string that the listener
            // hears, and print them to the terminal. If it hears
            // an empty line, we break out of the loop.
            let mut i = 0;
            let mut requested_resource:String = String::new();

            loop {
                let mut l = String::new();
                rdr.read_line(&mut l).unwrap();
                if l.trim().is_empty() {break;}

                if i == 0 {
                    i = 1;
                    requested_resource = l.split(" ").collect::<Vec<&str>>()[1].to_string();
                    println!("REQUESTED RESOURCE:{}", requested_resource);
                }

                print!("{l}");
            }

            // We need to resolve the actual resource-request made
            // by a user. The function "resolve" does this, and
            // stores the data corresponding to the request as an
            // array of bytes, as well as the corresponding HTTP
            // status, and returns it as a tuple.
            let http_status:i32;
            let body:Vec<u8>;
            (body, http_status) = resolve(&MANIFEST, &requested_resource);

            // We next assemble the HTTP header for the response.
            // This is done by concatenating together the mandatory
            // bits of the response with the status code as we found
            // above.
            let mut header = String::new();
            header.push_str("HTTP/1.1 ");
            header.push_str(&*http_status.to_string());


            header.push_str(match http_status {
                200 => " OK\r\n\r\n",
                _   => " NOT FOUND\r\n\r\n"
            } );

            let mut result:Vec<u8> = Vec::from(header.as_bytes());
            result = [result, body].concat();

            //println!("{:?}", result);

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
            //stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").unwrap();
            stream.write_all(&*result).unwrap();
            /* [[[[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]] */

        });
    }
}