#![recursion_limit = "128"]
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod common;
use crate::login::LoginModel;
use crate::one_person::OnePersonModel;
use crate::persons_list::PersonsListModel;
use common::{DbPrivilege, Person, User};

mod login;
mod one_person;
mod persons_list;

enum Page {
    Login,
    PersonsList,
    OnePerson(Option<Person>),
}

pub struct MainModel {
    page: Page,
    username: String,
    password: String,
    can_write: bool,
}

#[derive(Debug)]
pub enum MainMsg {
    LoggedIn(User),
    ChangeUserPressed,
    GoToOnePersonPage(Option<Person>),
    GoToPersonsListPage,
}

impl Component for MainModel {
    type Message = MainMsg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MainModel {
            page: Page::Login,
            username: "".to_string(),
            password: "".to_string(),
            can_write: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MainMsg::LoggedIn(user) => {
                self.page = Page::PersonsList;
                self.username = user.username;
                self.password = user.password;
                self.can_write = user.privileges.contains(&DbPrivilege::CanWrite);
            }
            MainMsg::ChangeUserPressed => self.page = Page::Login,
            MainMsg::GoToOnePersonPage(person) => self.page = Page::OnePerson(person),
            MainMsg::GoToPersonsListPage => self.page = Page::PersonsList,
        }
        true
    }
}

impl Renderable<MainModel> for MainModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <style>
                { "
                    .current-user { color: #0000C0}
                " }
                </style>
                <header>
                    <h2>{ "Persons management" }</h2>
                    <p>
                        { "Current user: " }
                        <span class="current-user", >
                        {
                            if self.username.is_empty() {
                                "---"
                            }
                            else {
                                &self.username
                            }
                        }
                        </span>
                        {
                            match self.page {
                                Page::Login => html! { <div/> },
                                _ => html! {
                                    <span>
                                        { " " }
                                        <button
                                            onclick=|_| MainMsg::ChangeUserPressed,>
                                            { "Change User" }
                                        </button>
                                    </span>
                                },
                            }
                        }
                    </p>
                    <hr/>
                </header>
                {
                    match &self.page {
                        Page::Login => html! {
                            <LoginModel:
                                when_logged_in=MainMsg::LoggedIn,
                                username=self.username.clone(),
                                password=self.password.clone(),
                            />
                        },
                        Page::PersonsList => html! {
                            <PersonsListModel:
                                can_write=self.can_write,
                                go_to_one_person_page=MainMsg::GoToOnePersonPage,
                                username=self.username.clone(),
                                password=self.password.clone(),
                            />
                        },
                        Page::OnePerson(person) => html! {
                            <OnePersonModel:
                                id=match person { Some(p) => Some(p.id), None => None },
                                name=match person { Some(p) => p.name.clone(), None => "".to_string() },
                                can_write=self.can_write,
                                go_to_persons_list_page=|_| MainMsg::GoToPersonsListPage,
                                username=self.username.clone(),
                                password=self.password.clone(),
                            />
                        },
                    }
                }
                <footer>
                    <hr/>
                    { "\u{A9} Carlo Milanesi - Developed using Yew and Actix-web" }
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<MainModel>();
}
