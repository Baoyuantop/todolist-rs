use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::{html, InputData};
use yew::events::KeyboardEvent;
#[derive(Debug,Clone)]
enum ListStatus {
  Todo,
  Done,
}
#[derive(Debug,Clone)]
struct List {
  content: String,
  status: ListStatus,
}
#[derive(Debug,Clone)]
struct AppState {
  list: Vec<List>,
  cur_input: String,
}
#[derive(Debug,Clone)]
struct Model {
    link: ComponentLink<Self>,
    state: AppState,
}

enum Msg {
    AddOne,
    ReduceOne(String),
    InputText(String),
    Null
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let init_list = List {
          content: String::from("This is a todo"),
          status: ListStatus::Done,
        };
        let state = AppState {
          list: vec![init_list],
          cur_input: "".into()
        };
        Self {
            link,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
              let new_todo = List {
                content: self.state.cur_input.to_string(),
                status: ListStatus::Todo,
              };
              self.state.list.push(new_todo);
              self.state.cur_input = "".to_string();
            },
            Msg::ReduceOne(text) => {
              let index = self.state.list.iter().position(|x| x.content == text).unwrap();
              self.state.list.remove(0);
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
                    {self.state.list.iter().map(|item| self.todo_item(item)).collect::<Html>()}
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

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

impl Model {
  fn todo_item(&self, list: &List) -> Html {
    html! {
      <li class="list-item">
        {list.content.to_string()}
        <button class="button-delete" onclick=self.link.callback(|_| Msg::ReduceOne(list.content))>{"Delete"}</button>
      </li>
    }
  }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}