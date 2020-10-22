extern crate getopts;
use getopts::Options;
use std::env;
use std::net::TcpStream;
use std::io;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::vec::Vec;
use openssl::ssl::{SslMethod, SslConnector};

fn mean(ar: &Vec<Duration>) -> Duration {
    let sum: Duration = ar.iter().sum();
    sum / ar.len() as u32
}

fn median(mut ar: Vec<Duration>) -> Duration {
    ar.sort();
    let mid = ar.len() as u32 / 2;
    ar[mid as usize]
}

fn connect(url: String, num_requests: u32, path: String) -> Result<(), io::Error>{
    let url_with_socket = format!("{}:443", url);
    let mut request_times: Vec<Duration> = Vec::new();
    let mut num_succeed:u32 = 0;
    let mut bytes: Vec<usize> = Vec::new();
    let mut error_codes: Vec<String> = Vec::new();
    // let mut buf;
    // let host = format!("Host: {}", url);
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, url_with_socket);

    for _num in 0..num_requests{
        let start_time = Instant::now();
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let stream = TcpStream::connect(url_with_socket.clone())?;
        let mut stream = connector.connect(&url, stream).unwrap();
        let mut request_data = String::new();
        request_data.push_str(&request);
        
        println!("request_data = {:?}", request_data);
        let _request = stream.write_all(request_data.as_bytes())?;
        let mut buf = vec![];
        let result = stream.read_to_end(&mut buf).unwrap();
        bytes.push(result);
        let end_time = Instant::now();
        request_times.push(end_time.duration_since(start_time));
        num_succeed = num_succeed + 1;
        let response = String::from_utf8_lossy(&buf);
        let mut code = String::new();
        code.push_str(&response.split("\r\n").collect::<Vec<&str>>()[0][9..12]);

        if &code[0..1] != "2"{
            error_codes.push(code.clone());
        }
        println!("buf = {}", String::from_utf8_lossy(&buf));
        println!("Code = {}", code);
    }

    println!("Number of requests: {}", num_requests);
    println!("Fastest time: {:?}", request_times.iter().max().unwrap());
    println!("Slowest time: {:?}", request_times.iter().min().unwrap());
    println!("Mean time: {:?} and Median time: {:?}", mean(&request_times), median(request_times));
    println!("Percentage of requests that succeeded: {}", (num_succeed * 100) / num_requests);
    println!("Any error codes returned that weren't a success: {:?}", error_codes);
    println!("Size in bytes of the smallest response: {:?}", bytes.iter().min().unwrap());
    println!("Size in bytes of the largest response: {:?}", bytes.iter().max().unwrap());
    Ok(())
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("", "url", "Give the url", "URL");
    opts.optopt("", "profile", "An integer for the number of requests to be made", "Num_hits");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_str("url").is_none(){
        print_usage(&program, opts);
        return
    }
    let url = matches.opt_str("url").unwrap();
    let target_url:Vec<&str> = url.split("/").collect();
    let path = format!("/{}", target_url[1..].join("/"));

    let num_profile_hits = match matches.opt_str("profile"){
        Some(nums) => {
            match nums.parse::<u32>(){
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a positive integer value for profile argument");
                    print_usage(&program, opts);
                    return
                }
                
            }
        },
        None => {
            print_usage(&program, opts);
            return
        }
    };

    let _input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    println!("{}", target_url[0]);
    
    match connect(target_url[0].to_string(), num_profile_hits, path){
        Ok(()) => (),
        Err(e) => println!("Error - {}", e),
    };

}