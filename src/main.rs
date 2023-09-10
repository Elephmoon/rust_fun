extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;
use std::{str::FromStr, collections::HashMap};
use urlencoded::UrlEncodedBody;
use iron::{
    headers::ContentType,
    mime,
    status,
};

fn main() {
    let mut router = Router::new();

    router.get("/", get_main_page, "main");
    router.post("/sum", post_calc_sum, "sum");

    println!("listen localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_main_page(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    set_http_status_ok(&mut resp);
    
    resp.set_mut(r#"
        <title>Sum calculator</title>
        <form action="/sum" method="post">
            <input type="number" name="a">
            <input type="number" name="b">
            <button type="submit">Calc sum</button>
        </form>
    "#);

    Ok(resp)
}

fn post_calc_sum(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let first_arg_name = String::from("a");
    let second_arg_name = String::from("b");

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(err) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("Error parsing form data: {:?}\n", err));
            return Ok(resp);
        }
        Ok(map) => map
    };

    let first_arg = match get_numb_arg_from_body(form_data, &first_arg_name) {
        Err(err) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("Error parsing form data: {:?}\n", err));
            return Ok(resp);
        }
        Ok(value) => value
    };

    let second_arg = match get_numb_arg_from_body(form_data, &second_arg_name) {
        Err(err) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("Error parsing form data: {:?}\n", err));
            return Ok(resp);
        }
        Ok(value) => value
    };

    set_http_status_ok(&mut resp);

    resp.set_mut(
        format!("The sum of the numbers {}, {} is <b>{}</b>\n", 
            first_arg, second_arg, first_arg+second_arg));

    Ok(resp)
}

fn get_numb_arg_from_body(form_data: &HashMap<String, Vec<String>>, param_name: &String) -> Result<i32, String> {
    let unparsed_arg = match form_data.get(param_name) {
        None => {
            return Err(format!("cant parse arg {}", param_name));
        }
        Some(arg) => arg
    };

    if unparsed_arg.len() == 0 {
        return Err(format!("cant parse arg {}", param_name));
    };

    return match i32::from_str(unparsed_arg[0].as_str()) {
        Err(err) => {
            Err(format!("parsing arg {} error {:?}", param_name, err))
        }
        Ok(value) => Ok(value)
    }
}

fn set_http_status_ok(resp: &mut Response) {
    resp.headers.set(
        ContentType(
            mime::Mime(
                mime::TopLevel::Text,
                mime::SubLevel::Html,
                vec![(mime::Attr::Charset, mime::Value::Utf8)]
            )
        )
    );

    resp.set_mut(status::Ok);
}
