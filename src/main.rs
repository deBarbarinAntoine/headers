use std::io;
use std::process::exit;
use error_chain::error_chain;
use console::style;
use reqwest::header::HeaderMap;
use reqwest::Response;

error_chain! {
    foreign_links {
        Io(io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: headers <url>");
        exit(1);
    }
    let url = add_proto(&args[1]);

    let res = make_request(&*url);
    match res {
        Ok(headers) => {
            let result = print_headers(&*url, &headers.headers());
            if let Err(_) = result { exit_error() }
        }
        _err => { exit_error() }
    };
}

fn exit_error() {
    println!("Something went wrong..."); exit(1)
}

fn add_proto(url: &String) -> String {
    if !url.starts_with("http://") & !url.starts_with("https://") {
        "https://".to_string() + url
    } else {
        url.to_string()
    }
}

fn print_headers(url: &str, headers: &HeaderMap) -> io::Result<()> {
    let title = String::from("Request to ") + url;
    println!();
    println!("{}\n", style(title).bold().yellow());

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
    Ok(())
}

#[tokio::main]
async fn make_request(url: &str) -> Result<Response> {
    let res = reqwest::get(url).await?;
    Ok(res)
}
