# Ex-rs-server(experimental rust webserver)

This is a single-threaded webserver written in rust that can serve static files and dynamic content via python
or any interpreted language that can write to `stdout` :).

## System overview

![Conceptual diagram](/screenshots/conceptual_diagram.png)

## Limitations/Improvements

- Single threaded
- Only supports http and content-type: text/html
- Does not support request parameters

## How to get and build

1. Clone the repo
```
git clone https://github.com/jnjenga/exp_rs_server
cd exp_rs_server
```

2. Build

This project uses [cargo](https://doc.rust-lang.org/cargo/), rust's package manager,

To build the project 

```
cargo build
```

## How to run

This server uses a simple text file format to manage the configurations. 
Below is a summary of the format


```
PROPERTY=VALUE
# This is a comment
```

1. Set confgurations

Create a configuration file, e.g `config.txt` and set the following properties

```
# Server address and port
BIND_ADDRESS=127.0.0.1:8080

# Location of the program
BASE_URL=D:/server_demo

# Path to interpretor program, in this case it's python
INTERPRETER_PATH=D:/Python39/python.exe

# Prefix of interpreted language source file
INTERPRETER_SOURCE_PREFIX=.py

# Rewrites and App properties

/=index.html

# Error page
404_PAGE=404.html
```

2. Launch application

```
cargo run config.txt
```

# Authors

- [@jnjenga](https://www.github.com/jnjenga)

# License

- [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)


