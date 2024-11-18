use std::io;
use std::ops::Add;
use std::process::exit;
use error_chain::error_chain;
use clap::{Arg, ArgGroup, ArgMatches, Command};
use clap::ArgAction::{SetTrue};
use console::{style, Style};
use reqwest::header::HeaderMap;
use http::{Method};

error_chain! {
    foreign_links {
        Io(io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() {
    let normal_style = Style::new().bold().blue();
    let highlight_style = Style::new().bold().cyan();

    let matches = Command::new("headers")
        .about(format!(
                "{}\n{} {}{}",
                normal_style.apply_to("A simple CLI tool to retrieve HTTP headers."),
                normal_style.apply_to("Made in Rust by"),
                highlight_style.apply_to("Antoine de Barbarin"),
                normal_style.apply_to(".")
                ))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            // Only required argument: the URL to send the request to
            Arg::new("url")
            .value_name("URL")
            .required(true)
            .help("The URL to get the headers from"))
        .arg(Arg::new("get")
                .short('G')
                .long("get")
                .help("Use method GET")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("post")
                .short('S')
                .long("post")
                .help("Use method POST")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("put")
                .short('U')
                .long("put")
                .help("Use method PUT")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("delete")
                .short('D')
                .long("delete")
                .help("Use method DELETE")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("patch")
                .short('A')
                .long("patch")
                .help("Use method PATCH")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("options")
                .short('O')
                .long("options")
                .help("Use method OPTIONS")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("trace")
                .short('T')
                .long("trace")
                .help("Use method TRACE")
                .action(SetTrue)
                .required(false))
        .arg(Arg::new("head")
                .short('H')
                .long("head")
                .help("Use method HEAD")
                .action(SetTrue)
                .required(false))
        .group(ArgGroup::new("method")
                .args(&["get", "post", "put", "delete", "patch", "options", "trace", "head"])
                .required(false))
        .get_matches();

    // Get the url argument
    let url = check_proto(matches.get_one::<String>("url").unwrap());

    // Get the method argument
    let method = get_method(&matches).unwrap();

    let res = make_request(&url, &method).await;
    match res {
        Ok(headers) => {
            println!();
            println!("{} {} {}\n", style(String::from("-> Request")).bold().blue(), style(method.to_string()).bold().cyan(), style(url).bold().yellow());
            let result = print_headers(&headers);
            if let Err(_) = result { exit_error() }
        }
        _err => { exit_error() }
    };
}

fn exit_error() {
    println!("Something went wrong..."); exit(1)
}

fn check_proto(url: &String) -> String {
    if !url.starts_with("http://") & !url.starts_with("https://") {
        String::from("https://").add(url)
    } else {
        String::from(url)
    }
}

fn get_method(matches: &ArgMatches) -> Option<Method> {
    Some(matches.get_flag("get").then_some(Method::GET)
        .or_else(|| matches.get_flag("post").then_some(Method::POST))
        .or_else(|| matches.get_flag("put").then_some(Method::PUT))
        .or_else(|| matches.get_flag("delete").then_some(Method::DELETE))
        .or_else(|| matches.get_flag("patch").then_some(Method::PATCH))
        .or_else(|| matches.get_flag("options").then_some(Method::OPTIONS))
        .or_else(|| matches.get_flag("trace").then_some(Method::TRACE))
        .unwrap_or(Method::HEAD))
}

fn print_headers(headers: &HeaderMap) -> io::Result<()> {
    let mut max_header_name_len = 0;
    for (k, _v) in headers.iter() {
        if k.as_str().len() > max_header_name_len {
            max_header_name_len = k.as_str().len();
        }
    }
    max_header_name_len += 3;
    for (&ref header_name, &ref header_value) in headers.iter() {
        let header: String = String::from(header_name.as_str()).to_uppercase();
        let mut value: String = String::new();
        let result = header_value.to_str();
        match result {
            Ok(val) => { value = String::from(val) }
            Err(_) => { exit_error() }
        }
        print!("{:1$}", style(header).bold().green(), max_header_name_len);
        println!("{}", style(value).blue());
    }

    println!();
    Ok(())
}

async fn make_request(url: &str, method: &Method) -> Result<HeaderMap> {
    let client = reqwest::Client::new();
    let res = client.request(method.clone(), url).send().await?;
    let headers = res.headers();
    Ok(headers.clone())
}