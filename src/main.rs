extern crate getopts;
use getopts::Options;
use openssl::ssl::{SslConnector, SslMethod};
use std::env;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::vec::Vec;

fn parse_url(url: String) -> (String, String) {
    let pruned_url = if let Some(u) = url.strip_prefix("http://") {
        u.to_owned()
    } else if let Some(u) = url.strip_prefix("https://") {
        u.to_owned()
    } else {
        url
    };

    let slash_loc = pruned_url.find("/");

    if let Some(idx) = slash_loc {
        (pruned_url[..idx].to_owned(), pruned_url[idx..].to_owned())
    } else {
        (pruned_url, "/".to_owned())
    }
}

fn mean(ar: &Vec<Duration>) -> Duration {
    let sum: Duration = ar.iter().sum();
    sum / ar.len() as u32
}

fn median(ar: &mut Vec<Duration>) -> Duration {
    ar.sort();
    let mid = ar.len()/ 2;
    if ar.len() % 2 == 0{
        (ar[mid] + ar[mid - 1]) / 2
    } else{
        ar[mid]
    }
}

fn connect(host: String, num_requests: u32, path: String, verbose: bool) -> Result<(), io::Error> {
    let host_with_socket = format!("{}:443", host);
    let mut request_times: Vec<Duration> = Vec::new();
    let mut num_succeed: u32 = 0;
    let mut bytes: Vec<usize> = Vec::new();
    let mut error_codes: Vec<String> = Vec::new();
    
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host_with_socket
    );
    println!("Http request -> {:?}", request);
    for num in 0..num_requests {
        
        let start_time = Instant::now();
        
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let stream = TcpStream::connect(host_with_socket.clone())?;
        let mut stream = connector.connect(&host, stream).unwrap();

        let _request = stream.write_all(request.as_bytes())?;
        let mut buf = vec![];
        let result = stream.read_to_end(&mut buf).unwrap();
        let response = String::from_utf8_lossy(&buf);

        bytes.push(result);
        let end_time = Instant::now();
        request_times.push(end_time.duration_since(start_time));
        num_succeed = num_succeed + 1;
        
        // response contains the HTTP Response status code and any status code that doesn't start
        // with 2 is an error code.
        if let Some(mut status_code) = response.strip_prefix("HTTP/1.1"){
            status_code = status_code.trim();
            if !status_code.starts_with("2"){
                error_codes.push(status_code[..3].to_string());
            }
        }

        if verbose && num == 0{
            println!("Response= {}", response);
        }
    }

    println!("\nNumber of requests: {}", num_requests);
    println!("\nTime:\n\tFastest time: {:?}", request_times.iter().max().unwrap());
    println!("\tSlowest time: {:?}", request_times.iter().min().unwrap());
    println!("\tMean time: {:?}", mean(&request_times));
    println!("\tMedian time: {:?}", median(&mut request_times));
    println!(
        "\nPercentage of requests that succeeded: {}%",
        (num_succeed * 100) / num_requests
    );
    println!(
        "\nError codes returned that weren't a success: {:?}",
        error_codes
    );
    println!("\nSize in bytes:");
    println!(
        "\tSmallest response: {:?}",
        bytes.iter().min().unwrap()
    );
    println!(
        "\tLargest response: {:?}",
        bytes.iter().max().unwrap()
    );
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
    opts.optopt(
        "",
        "profile",
        "An integer for the number of requests to be made",
        "Num_hits",
    );
    opts.optopt("v", "verbose", "Level of verbose (Either 0 or 1) Default 0 - No output", "VERBOSE");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_str("url").is_none() {
        print_usage(&program, opts);
        return;
    }

    let url = matches.opt_str("url").unwrap();
    let (target_url, path) = parse_url(url);

    let num_profile_hits = match matches.opt_str("profile") {
        Some(nums) => match nums.parse::<u32>() {
            Ok(num) => {
                if num == 0 {
                    println!("Enter a positive integer (min 1) for the profile argument");
                    print_usage(&program, opts);
                    return;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Enter a positive integer (min 1) for the profile argument");
                print_usage(&program, opts);
                return;
            }
        },
        None => 1,
    };

    let verbose = match matches.opt_str("verbose"){
        Some(val) => match val.parse::<u8>(){
            Ok(val) => match val{
                0 => false,
                1 => true,
                _ => false,
            }
            Err(_) => false
        }
        None => false
    };


    match connect(target_url, num_profile_hits, path, verbose) {
        Ok(()) => (),
        Err(e) => println!("Error - {}", e),
    };
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_median() {
        let mut a = vec![Duration::new(1,0), Duration::new(2, 0), Duration::new(4,0), Duration::new(5, 0), Duration::new(3, 0)];
        assert_eq!(median(&mut a), Duration::new(3,0));
        let mut b = vec![Duration::new(1, 0),Duration::new(2, 0),Duration::new(3, 0),Duration::new(4, 0)];
        assert_eq!(median(&mut b), Duration::new(2,0) + Duration::from_millis(500));
    }

    #[test]
    fn test_mean() {
        let a = vec![Duration::new(1,0), Duration::new(2, 0), Duration::new(4,0), Duration::new(5, 0), Duration::new(3, 0)];
        assert_eq!(mean(&a), Duration::new(3,0));
        let b = vec![Duration::new(1, 0),Duration::new(2, 0),Duration::new(3, 0),Duration::new(4, 0)];
        assert_eq!(mean(&b), Duration::new(2,0) + Duration::from_millis(500));
    }

    #[test]
    fn test_parse_url() {
        let a = parse_url("raunaqjain.com".to_string());
        assert_eq!(a, ("raunaqjain.com".to_string(), "/".to_string()));

        let b = parse_url("raunaqjain.com/resume".to_string());
        assert_eq!(b, ("raunaqjain.com".to_string(), "/resume".to_string()));

        let c = parse_url("http://raunaqjain.com".to_string());
        assert_eq!(c, ("raunaqjain.com".to_string(), "/".to_string()));

        let c = parse_url("https://raunaqjain.com/links".to_string());
        assert_eq!(c, ("raunaqjain.com".to_string(), "/links".to_string()));

        let c = parse_url("http://raunaqjain.com/links/github".to_string());
        assert_eq!(c, ("raunaqjain.com".to_string(), "/links/github".to_string()));
    }
}