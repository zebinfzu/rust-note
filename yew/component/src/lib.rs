use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let name = "gary";
    let my_object = MyObject {
        username: name.clone().to_string(),
        favorite_language: "Rust".to_string(),
    };
    log!(name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let class = "my_title";
    let message = Some("I am a message");

    let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];

    let stylesheet = style!(
      r#"
        color: red;
      "#
    );
    html! {
    <>
      <h1 class={stylesheet.unwrap()}>{"Hello world"}</h1>
      <h1 class={class}>{"Good bey!"}</h1>
      if class == "my_title" {
        <p>{"True case"}</p>
      } else {
        <p>{"False case"}</p>
      }
      if let Some(message) = message {
        <p>{message}</p>
      } else {
        <p>{"no message here"}</p>
      }

      <ul>
        {list_to_html(tasks)}
      </ul>
    </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
