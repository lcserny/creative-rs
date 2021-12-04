use failure::Error;
use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::common::{add_auth, User, BACKEND_SITE};

pub struct LoginModel {
    fetching: bool,
    fetch_service: FetchService,
    ft: Option<FetchTask>,
    link: ComponentLink<LoginModel>,
    dialog: DialogService,
    username: String,
    password: String,
    when_logged_in: Option<Callback<User>>,
    console: ConsoleService,
}

#[derive(Debug)]
pub enum LoginMsg {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
    ReadyLogin(User),
    Failure(String),
}

#[derive(PartialEq, Clone)]
pub struct LoginProps {
    pub username: String,
    pub password: String,
    pub when_logged_in: Option<Callback<User>>,
}

impl Default for LoginProps {
    fn default() -> Self {
        LoginProps {
            username: "".to_string(),
            password: "".to_string(),
            when_logged_in: None,
        }
    }
}

#[derive(Deserialize)]
enum AuthenticationResult {
    LoggedUser(User),
    ErrorMessage(String),
}

impl Component for LoginModel {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginModel {
            fetching: false,
            fetch_service: FetchService::new(),
            ft: None,
            link,
            dialog: DialogService::new(),
            username: props.username,
            password: props.password,
            when_logged_in: props.when_logged_in,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginMsg::UsernameChanged(username) => self.username = username,
            LoginMsg::PasswordChanged(password) => self.password = password,
            LoginMsg::LoginPressed => {
                if self.username.is_empty() {
                    self.dialog.alert("User not specified.");
                    return false;
                }
                self.fetching = true;
                let callback = self.link.send_back(
                    move |response: Response<Json<Result<AuthenticationResult, Error>>>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(auth_res) => match auth_res {
                                AuthenticationResult::LoggedUser(user) => {
                                    LoginMsg::ReadyLogin(user)
                                }
                                AuthenticationResult::ErrorMessage(msg) => LoginMsg::Failure(msg),
                            },
                            Err(err_msg) => {
                                LoginMsg::Failure(format!("Authentication failed: {}.", err_msg))
                            }
                        }
                    },
                );

                let mut request = Request::get(format!("{}authenticate", BACKEND_SITE))
                    .body(Nothing)
                    .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            LoginMsg::ReadyLogin(user) => {
                self.fetching = false;
                self.console.log(&format!("User: {:?}.", user));
                if let Some(ref go_to_page) = self.when_logged_in {
                    go_to_page.emit(user.clone());
                }
            }
            LoginMsg::Failure(msg) => {
                self.fetching = false;
                self.console.log(&format!("Failure: {:?}.", msg));
                self.dialog.alert(&msg);
                return false;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.username = props.username;
        self.password = props.password;
        self.when_logged_in = props.when_logged_in;
        true
    }
}

impl Renderable<LoginModel> for LoginModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <label>{ "User name: " }</label>
                    <input
                        type="text",
                        value=&self.username,
                        oninput=|e| LoginMsg::UsernameChanged(e.value),
                    />
                </div>
                <div>
                    <label>{ "Password: " }</label>
                    <input
                        type="password",
                        oninput=|e| LoginMsg::PasswordChanged(e.value),
                    />
                </div>
                <button
                    onclick=|_| LoginMsg::LoginPressed,>
                    { "Log in" }
                </button>
            </div>
        }
    }
}
