use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::{html, InputData};
use yew::events::KeyboardEvent;
use serde::{Serialize, Deserialize};
use yew::services::storage::{Area, StorageService};
use yew::format::Json;

#[derive(Debug,Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ListStatus {
  Todo,
  Done,
}
#[derive(Debug,Clone, PartialEq, Eq, Serialize, Deserialize)]
struct List {
  content: String,
  status: ListStatus,
}
#[derive(Debug,Clone)]
struct AppState {
  list: Vec<List>,
  cur_input: String,
}
#[derive(Debug)]
struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: AppState,
}

enum Msg {
    AddOne,
    ReduceOne(usize),
    Done(usize),
    InputText(String),
    Null
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
      let storage = StorageService::new(Area::Local).expect("");
        let init_list = List {
          content: String::from("This is a todo"),
          status: ListStatus::Todo,
        };
        let mut state = AppState {
          list: vec![init_list],
          cur_input: "".into()
        };
        if let Json(Ok(store_data)) = storage.restore("state") {
          state.list = store_data;
        }
        Self {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
              if self.state.cur_input.len() == 0 {
                
              } else {
                let new_todo = List {
                  content: self.state.cur_input.to_string(),
                  status: ListStatus::Todo,
                };
                self.state.list.push(new_todo);
                self.state.cur_input = "".to_string();
              }
            },
            Msg::ReduceOne(index) => {
              self.state.list.remove(index);
            }
            Msg::Done(index) => {
              if self.state.list[index].status == ListStatus::Done {
                self.state.list[index].status = ListStatus::Todo;
              } else {
                self.state.list[index].status = ListStatus::Done;
              }
            }
            Msg::InputText(text ) => {
              self.state.cur_input = text;
            }
            Msg::Null => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
              <div class="header">
                {"A Todolist APP With Rust ~"}
              </div>
              <div class="body">
                <div class="title">
                  {"Today"}
                </div>
                <div class="content">
                  <ul>
                    {self.state.list.iter().enumerate().map(|(i, item)| self.todo_item(i, item)).collect::<Html>()}
                    <li class="list-item list-item-input">
                      <input
                        class="input"
                        type="text"
                        placeholder="input to add a todo"
                        oninput=self.link.callback(|e: InputData| Msg::InputText(e.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                          if e.key() == "Enter" {
                            Msg::AddOne
                          } else {
                            Msg::Null
                          }
                        })
                        value={self.state.cur_input.to_string()}
                      />
                      <button class="button-add" onclick=self.link.callback(|_| Msg::AddOne)>{ "Add" }</button>
                    </li>
                  </ul>
                </div>
              </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        self.store();
    }

    fn destroy(&mut self) {}
}

impl Model {
  fn todo_item(&self, index: usize, list: &List) -> Html {
    let cur_status_class: &str;
    let cur_text_class: &str;
    match list.status {
        ListStatus::Todo => {
          cur_status_class = "status todo";
          cur_text_class = "text-todo";
        }
        ListStatus::Done => {
          cur_status_class = "status done";
          cur_text_class = "text-done";
        }
    }
    html! {
      <li class="list-item" >
        <div class={cur_status_class} onclick=self.link.callback(move |_| Msg::Done(index))></div>
        <div class="list-item-content">
          <p class={cur_text_class}>{list.content.to_string()}</p>
          <button class="button-delete" onclick=self.link.callback(move |_| Msg::ReduceOne(index))>{"Delete"}</button>
        </div>
      </li>
    }
  }

  fn store(&mut self) {
      self.storage.store("state", Json(&self.state.list));
  }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}