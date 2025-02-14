use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[function_component(DirectoryRow)]
fn directory_row(item: &ItemProps) -> Html {

use std::path::PathBuf;
    let mut path = PathBuf::from(item.path.clone());
    path.push(item.name.clone());
    let path_str = path.to_string_lossy().into_owned();
    html! {
      <tr>
            <td>{item.mimetype_icon.clone()}</td>
            <td><a href={path_str}>{item.name.clone()}</a></td>
      </tr>
    }
}

#[function_component(DirectoryListing)]
fn directory_listing(props: &DirectoryListingProps) -> Html {
    let items = use_state(|| vec![]);
    let path = props.path.clone();

    use_effect_with((), {
        let items = items.clone();
        move |_| {
            spawn_local(async move {
                let fetched = fetch_directory(path.clone()).await;
                items.set(fetched);
            });
            || ()
        }
    });

    html! {
      <table border="1">
        <tr>
          <th>{""}</th>
          <th>{"Name"}</th>
        </tr>
        { if props.path != "/" {
            html! {
                <tr>
                    <td></td>
                    <td><a href="..">{".."}</a></td>
                    </tr>
            }
        } else {
            html! {}
        }}

        { (*items).iter().map(|item| html! {
            <DirectoryRow
                path={props.path.clone()}
                name={item.name.clone()}
                is_dir={false}
                mimetype={item.mimetype.clone()}
                mimetype_icon={item.mimetype_icon.clone()}
                request="" />
        }).collect::<Html>()}
      </table>
    }
}

#[function_component(Directory)]
pub fn directory(props: &DirectoryProps) -> Html {
    html! {
      <main>
        <heading>
          <h2>{ format!("Directory: {}", props.path) }</h2>
         </heading>
         <div class="content">
           <DirectoryListing path={props.path.clone()} />
         </div>
       </main>
    }
}


#[derive(Properties, PartialEq)]
pub struct DirectoryProps {
    pub path: String,
}

#[derive(Properties, PartialEq)]
struct DirectoryListingProps {
    pub path: String,
}

// Define the structure of the JSON response
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Item {
    pub mimetype: String,
    pub mimetype_icon: String,
    pub name: String,
    pub request: String,
    pub is_dir: bool,
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    pub mimetype: String,
    pub mimetype_icon: String,
    pub name: String,
    pub path: String,
    pub request: String,
    pub is_dir: bool,
}

async fn fetch_directory(path: String) -> Vec<Item> {
    let url = format!("/api{}", path);
    log(&format!("Fetching {:?}", url));

    let response = Request::get(&url).send().await;
    let Ok(response) = response else {
        log("Failed to fetch response");
        return vec![];
    };

    let body = response.text().await.unwrap_or_default();
    log(&format!("Response Body: {:?}", body));

    match serde_json::from_str::<Vec<Item>>(&body) {
        Ok(fetched_items) => fetched_items,
        Err(err) => {
            log(&format!("Failed to deserialize JSON: {:?}", err));
            vec![]
        }
    }
}

fn log(msg: &str) {
    console::log_1(&msg.into());
}
