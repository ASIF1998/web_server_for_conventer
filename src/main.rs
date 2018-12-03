extern crate iron;

#[macro_use]
extern crate mime;

extern crate router;
extern crate urlencoded;

#[macro_use]
extern crate lazy_static;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

use std::fs::File;
use std::io::prelude::*;

use std::format;

use std::collections::HashMap;

const LOCAL: &str = "localhost:3016";

lazy_static! {
    static ref exchangeRates: Box<[Box<[f64]>]> = Box::new([
        Box::new([1.0, 0.37907, 0.30366, 249.821, 1.36576, 6.58758, 67.0193, 49.7280, 0.88300]),
        Box::new([2.63583, 1.0, 0.80039, 658.486, 183.302, 3.60207, 17.3803, 176.651, 131.074, 2.32937]),
        Box::new([3.28085, 1.24368, 1.0, 819.625, 228.158, 4.48084, 21.6128, 219.880, 163.150, 2.89700]),
        Box::new([0.00399, 0.00151, 0.00121, 1.0, 0.27767, 0.00545, 0.02630, 0.26760, 0.19856, 0.00353]),
        Box::new([0.01433, 0.00543, 0.00435, 3.57949, 1.0, 0.01957, 0.09439, 0.96027, 0.71251, 0.01265]),
        Box::new([0.73080, 0.27728, 0.22192, 182.571, 50.8220, 1.0, 4.81423, 48.9780, 36.3414, 0.64589]),
        Box::new([0.15157, 0.05751, 0.04602, 37.8643, 10.5402, 0.20700, 1.0, 10.1578, 7.53704, 0.13397]),
        Box::new([0.01491, 0.00565, 0.00453, 3.72429, 1.03673, 0.02036, 0.09821, 1.0, 0.74134, 0.01316]),
        Box::new([0.01977, 0.00749, 0.00749, 0.00600, 4.93784, 1.37454, 0.02699, 0.13021, 1.32467, 1.0, 0.01745]),
        Box::new([1.13156, 0.42930, 0.34361, 282.688, 78.6916, 1.54637, 7.46138, 75.8364, 56.2702, 1.0])
    ]);
}

fn main() {
    let mut router = Router::new();
    router.get("/", get_index_form, "root");
    router.post("/currency_converter", currency_converter_handler, "currency_converter");
    println!("Serving on {}", LOCAL);
    Iron::new(router).http(LOCAL).expect("Noooo");
}

fn currency_converter_handler(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(mime!(Text/Html; Charset=Utf8));

    let form_data = request.get::<UrlEncodedBody>().expect("Noo data");

    let sel1 = match  get_number::<isize>("sel1", &form_data) {
        Ok(t) => {
            t
        }

        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error: {:?}", e));
            return Ok(response);
        }
    };


    let sel2 = match get_number::<isize>("sel2", &form_data) {
        Ok(t) => {
            t
        }

        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error: {:?}", e));
            return Ok(response);
        }
    };

    let value = match get_number::<f32>("input", &form_data) {
        Ok(t) => {
            t
        }

        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error: {:?}", e));
            return Ok(response);
        }
    };

    response.set_mut(status::Ok);
    response.set_mut(format!("Data: {} {} {}", sel1, sel2, value));
    Ok(response)
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

fn get_number<T> (name: &str, hash: &HashMap<String, Vec<String>, std::collections::hash_map::RandomState>) -> Result<T, T::Err>
    where T: FromStr
{
    let num = match hash.get(name) {
        Some(e) => {
            e
        }

        None => {
            panic!("Nooo");
        }
    };

    num[0].parse::<T>()
}
