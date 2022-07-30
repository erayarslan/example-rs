use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    name: String,
    email: String,
    text: String,
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:8080/comments");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("X-Blyad-Value", "hehe")?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let comments: Vec<Comment> = json.into_serde().unwrap();

    let table = document.create_element("table")?;
    let thead = document.create_element("thead")?;
    let tr = document.create_element("tr")?;
    let name_th = document.create_element("th")?;
    name_th.set_text_content(Some("Name"));
    let email_th = document.create_element("th")?;
    email_th.set_text_content(Some("Email"));
    let text_th = document.create_element("th")?;
    text_th.set_text_content(Some("Text"));

    tr.append_child(&name_th)?;
    tr.append_child(&email_th)?;
    tr.append_child(&text_th)?;

    thead.append_child(&tr)?;
    table.append_child(&thead)?;

    let tbody = document.create_element("tbody")?;

    for val in comments.iter() {
        let tr = document.create_element("tr")?;

        let name_td = document.create_element("td")?;
        name_td.set_text_content(Some(val.name.as_str()));
        let email_td = document.create_element("td")?;
        email_td.set_text_content(Some(val.email.as_str()));
        let text_td = document.create_element("td")?;
        text_td.set_text_content(Some(val.text.as_str()));

        tr.append_child(&name_td)?;
        tr.append_child(&email_td)?;
        tr.append_child(&text_td)?;

        tbody.append_child(&tr)?;
    }

    table.append_child(&tbody)?;
    body.append_child(&table)?;

    log("Render done!");

    Ok(())
}