use yew::prelude::*;

pub struct Client;

pub struct ClientMessage;

#[derive(Debug, Properties, Default, PartialEq)]
pub struct ClientProperties;

pub type ClientRenderer = yew::Renderer<Client>;

impl Component for Client {
    type Message = ClientMessage;
    type Properties = ClientProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Client
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        _msg: ClientMessage,
    ) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{ "Hello, world!" }</p>
        }
    }
}
