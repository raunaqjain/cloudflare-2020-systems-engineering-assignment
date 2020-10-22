# Systems Assignment
This is my solution for the Systems Assignment for Cloudflare 2021 New Grad position. This Rust program allows one to send HTTP GET requests multiple times and collect various metrics such as fastest time, slowest time, mean time, median time, size in bytes of smallest and largest response, and various error codes.

## Usage
To build the code use,
``` 
cargo build
```
To find the different command line arguments, run
```
cargo run main.rs -h

Options:
        --url URL       Give the url
        --profile Num_hits
                        An integer for the number of requests to be made
    -v, --verbose VERBOSE
                        Level of verbose (Either 0 or 1) Default 0
    -h, --help          print this help menu
```
Run unit tests through, 
``` 
cargo test
```

## Output on my General Assessment /links page
### Command
```
cargo run main.rs --url cloudflare-test.raunaqja.workers.dev/links --profile 1 -v 1
```
### Output
```
Http request -> "GET /links HTTP/1.1\r\nHost: cloudflare-test.raunaqja.workers.dev:443\r\nConnection: close\r\n\r\n"
Response= HTTP/1.1 200 OK
Date: Thu, 22 Oct 2020 05:39:21 GMT
Content-Type: application/json
Content-Length: 184
Connection: close
Set-Cookie: __cfduid=d6fc0cac216393dcf9651538f3bffbee31603345161; expires=Sat, 21-Nov-20 05:39:21 GMT; path=/; domain=.raunaqja.workers.dev; HttpOnly; SameSite=Lax
cf-request-id: 05f06ab3eb0000275153127000000001
Expect-CT: max-age=604800, report-uri="https://report-uri.cloudflare.com/cdn-cgi/beacon/expect-ct"
Report-To: {"endpoints":[{"url":"https:\/\/a.nel.cloudflare.com\/report?lkg-colo=16&lkg-time=1603345161"}],"group":"cf-nel","max_age":604800}
NEL: {"report_to":"cf-nel","max_age":604800}
Server: cloudflare
CF-RAY: 5e60e09978632751-IAD

[{"name":"Portfolio","url":"https://www.raunaqjain.com"},{"name":"LinkedIn","url":"https://www.linkedin.com/in/jainraunaq"},{"name":"Github","url":"https://www.github.com/raunaqjain"}]

Number of requests: 1

Time:
        Fastest time: 158.207871ms
        Slowest time: 158.207871ms
        Mean time: 158.207871ms
        Median time: 158.207871ms

Percentage of requests that succeeded: 100%

Error codes returned that weren't a success: []

Size in bytes:
        Smallest response: 865
        Largest response: 865
```

## Output on various other sites with number of requests = 10
- [Personal Website](https://www.raunaqjain.com)
```
Http request -> "GET / HTTP/1.1\r\nHost: www.raunaqjain.com:443\r\nConnection: close\r\n\r\n"

Number of requests: 10

Time:
        Fastest time: 139.892896ms
        Slowest time: 338.522947ms
        Mean time: 180.715988ms
        Median time: 161.313032ms

Percentage of requests that succeeded: 100%

Error codes returned that weren't a success: []

Size in bytes:
        Smallest response: 109318
        Largest response: 109367
```

- [Google](www.google.com)
```
Http request -> "GET / HTTP/1.1\r\nHost: www.google.com:443\r\nConnection: close\r\n\r\n"

Number of requests: 10

Time:
        Fastest time: 150.31022ms
        Slowest time: 191.96281ms
        Mean time: 160.39725ms
        Median time: 157.232126ms

Percentage of requests that succeeded: 100%

Error codes returned that weren't a success: []

Size in bytes:
        Smallest response: 49627
        Largest response: 49725
```
- [Facebook](www.facebook.com)
```
Http request -> "GET / HTTP/1.1\r\nHost: www.facebook.com:443\r\nConnection: close\r\n\r\n"

Number of requests: 10

Time:
        Fastest time: 88.601193ms
        Slowest time: 133.637595ms
        Mean time: 97.11485ms
        Median time: 92.640969ms

Percentage of requests that succeeded: 100%

Error codes returned that weren't a success: ["302", "302", "302", "302", "302", "302", "302", "302", "302", "302"]

Size in bytes:
        Smallest response: 404
        Largest response: 404
```
- [Rust-lang](www.rust-lang.org)
```
Http request -> "GET / HTTP/1.1\r\nHost: www.rust-lang.org:443\r\nConnection: close\r\n\r\n"

Number of requests: 10

Time:
        Fastest time: 105.849415ms
        Slowest time: 202.218346ms
        Mean time: 122.887386ms
        Median time: 113.823975ms

Percentage of requests that succeeded: 100%

Error codes returned that weren't a success: []

Size in bytes:
        Smallest response: 20085
        Largest response: 20085
```
- [Reddit's popular page](www.reddit.com/r/popular)
```
Http request -> "GET /r/popular HTTP/1.1\r\nHost: www.reddit.com:443\r\nConnection: close\r\n\r\n"

Number of requests: 10

Time:
        Fastest time: 1.758914712s
        Slowest time: 10.637291288s
        Mean time: 2.861235058s
        Median time: 2.070298994s

Percentage of requests that succeeded: 100%

Error codes returned that weren't a success: []

Size in bytes:
        Smallest response: 138090
        Largest response: 818850
```

## What is it?

This exercise is a follow-on to the [General Assignment](https://github.com/cloudflare-hiring/cloudflare-2020-general-engineering-assignment), you'll need to complete that first.  In this assignment you'll write a program that makes a request to the endpoints you created in the General Assignment.  This is a systems assignment so we want to see that you're able to use sockets directly rather than using a library that handles the HTTP request.

## Useful Links

- [A Tour of Go](https://tour.golang.org/welcome/1)
- [The Rust Programming Language](https://doc.rust-lang.org/book/index.html)
- [Cloudflare General Assignment](https://github.com/cloudflare-hiring/cloudflare-2020-general-engineering-assignment)

## Requirements

### 1. Use one of the specified languages

Choose from among C/C++/Go/Rust. If you aren't familiar with these languages, you're not alone! Many engineers join Cloudflare without
specific language experience. See the Useful Links section for some quickstart guides.

### 2. Use an off the shelf build tool

Choose something to build your assignment that works with the language you chose (Cargo, Make, CMake etc.).  Include instructions in your readme on how to build and run your program.  Don't check-in binaries, we won't run a pre-compiled binary.

### 3. Do **NOT** use a library to handle the HTTP request

We want to see how familiar you are with systems work.  Although we would normally recommend using a library to handle HTTP requests, for this assignment we want to see how you handle it yourself.

### 4. Create a CLI tool that makes a request to your links page

Your CLI tool should take an argument that is a full URL (--url).  The tool will make an HTTP request to the URL and print the response directly to the console.  Test the CLI tool by specifying the /links URL in your General Assignment and make sure it prints the entire json document with all your links.

Your CLI tool should also allow a --help parameter that describes how to use it.

Feel free to use a library to handle command line argument parsing (getopt etc.).

### 5. Measure how fast it is

Next, add logic to your tool to profile your page.  Add a new argument --profile that takes a positive integer.  Your tool should make that number of requests to your site.  Time the requests and print:

* The number of requests
* The fastest time
* The slowest time
* The mean & median times
* The percentage of requests that succeeded
* Any error codes returned that weren't a success
* The size in bytes of the smallest response
* The size in bytes of the largest response

Include a screenshot of your tool run against your site and another webpage.

Test your tool against your site and some other websites.  Let us know what you find in your readme.  Include outputs for popular sites and your own.  How do we compare?

## Submitting your project

When submitting your project, you should prepare your code for upload to Greenhouse. The preferred method for doing this is to create a "ZIP archive" of your project folder: for more instructions on how to do this on Windows and Mac, see [this guide](https://www.sweetwater.com/sweetcare/articles/how-to-zip-and-unzip-files/).

Please provide the source code only, a compiled binary is not necessary.
