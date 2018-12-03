extern crate iron;

#[macro_use] extern crate mime;

extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

use std::fs::File;
use std::io::prelude::*;

use std::format;

const LOCAL: &str = "localhost:3015";

fn main() {
    let mut router = Router::new();
    router.get("/", get_index_form, "root");
    router.post("/currency_converter", currency_converter_handler, "currency_converter");
    println!("Serving on {}", LOCAL);
    Iron::new(router).http(LOCAL).expect("Noooo");
}

fn get_index_form(_: &mut Request) -> IronResult<Response> {
    let mut responce = Response::new();

    responce.set_mut(mime!(Text/Html; Charset=Utf8));

    let mut contents = String::new();

    let mut file = match File::open("html_code/index.html") {
        Ok(f) => {
            f
        }

        Err(e) => {
            responce.set_mut(status::BadRequest);
            responce.set_mut(format!("Error open file: {}", e));
            return Ok(responce);
        }
    };

    match file.read_to_string(&mut contents) {
        Err(e) => {
            responce.set_mut(status::BadRequest);
            responce.set_mut(format!("Error read file: {}", e));
            return Ok(responce);
        }

        _=> {

        }
    }

    responce.set_mut(status::Ok);
    responce.set_mut(contents);

    Ok(responce)
}

fn currency_converter_handler(request: &mut Request) -> IronResult<Response> {
    println!("Yes");
    let mut response = Response::new();

    let form_data = request.get::<UrlEncodedBody>().expect("Noo data");

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("Data: {:?}", form_data));
    Ok(response)
}
