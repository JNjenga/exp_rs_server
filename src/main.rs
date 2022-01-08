mod config;
use crate::config::Config;
use std::net::TcpListener;
use std::net::TcpStream;
use exp_rs_server::Request;
use exp_rs_server::Response;
use std::str::FromStr;
use std::fs;
use std::io::prelude::*;
use std::str;
use std::process::Command;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::env;

fn usage()
{
    println!("Usage:
             exp_server <config file>");
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    match args.len() 
    {
        1 =>
        {
            usage();
            return;
        },
        _ => { }
    }

    let config_path = &args[1];
    let c: Config = Config::new_from_file(config_path);

    let bind_address: String;
    if let Some(address) = c.get("BIND_ADDRESS")
    {
        bind_address = address.to_string();
    }
    else
    {
        bind_address = "localhost:8080".to_string();
    }

    let listener = TcpListener::bind(bind_address).unwrap();
    for stream in listener.incoming()
    {
        let stream = stream.unwrap();

        handle_connection(stream, &c);
    }
}

fn handle_connection(mut stream: TcpStream, config: &config::Config)
{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let s: &str = std::str::from_utf8(&buffer).unwrap();
    let mut request = Request::from_str(&s).unwrap();

    match config.get(&request.route)
    {
        Some(route) => request.route = route.to_string(),
        None => {},
    }

    let mut response = Response{
        version: "HTTP/1.1".to_string(),
        status:  "200 OK".to_string(),
        body:    "".to_string(),
    };

    // Non source code files
    if !request.route.ends_with(config.get("INTERPRETER_SOURCE_PREFIX").unwrap())
    {
        let mut url: String = request.route;
        if let Some(url_property) = config.get("BASE_URL")
        {
            url = url_property.clone() + &url;
        }
        println!("Retriving file {}", url);

        match fs::read_to_string(url)
        {
            Ok(contents) => {
                response.body   = String::from(contents);
                response.status = "200 OK".to_string();
            },
            Err(e) => {
                response.status = "404 Not found".to_string();
                // Read 404 page or use default
                if let Some(http_404_page) = config.get("404_PAGE")
                {
                    // Check if the path is valid
                    match fs::read_to_string(config.get("BASE_URL").unwrap().to_owned() + http_404_page)
                    {
                        Ok(http_404_page_contents) =>
                        {
                            response.body = String::from(http_404_page_contents);
                        }
                        Err(_) =>
                        {
                            response.body = String::from("Resource not found");
                        }
                    }
                }
                else
                {
                    response.body = String::from("Resource not found");
                }
            }
        }
    }
    else
    {
        let mut url: String = request.route;
        if let Some(url_property) = config.get("BASE_URL")
        {
            url = url_property.clone() + &url;
        }

        // Check if file exists
        if std::path::Path::new(&url).is_file()
        {
            let interpreter = config.get("INTERPRETER_PATH").unwrap();

            // Execute file
            let output = Command::new(interpreter)
                .arg(url)
                .output()
                .expect("failed to execute process");

            response.body = String::from_utf8(output.stdout).unwrap();
            response.status = "200 OK".to_string();
        }
        else
        {
            response.status = "404 Not found".to_string();
            // Read 404 page or use default
            if let Some(http_404_page) = config.get("404_PAGE")
            {
                // Check if the path is valid
                match fs::read_to_string(config.get("BASE_URL").unwrap().to_owned() + http_404_page)
                {
                    Ok(http_404_page_contents) =>
                    {
                        response.body = String::from(http_404_page_contents);
                    }
                    Err(_) =>
                    {
                        response.body = String::from("Resource not found");
                    }
                }
            }
            else
            {
                response.body = String::from("Resource not found");
            }
        }
    }

    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
