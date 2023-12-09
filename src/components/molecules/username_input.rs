use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<String>,
}

#[function_component(UsernameInput)]
pub fn username_input(props: &Props) -> Html {
    let username_state = use_state(|| String::from(""));
    let username_setter = username_state.setter();

    let username_changed = Callback::from(move |username| {
        username_setter.set(username);
    });

    let cloned_username_state = username_state.clone();

    let form_onsubmit = props.onsubmit.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        form_onsubmit.emit(cloned_username_state.to_string());
    });

    html! {
     <form onsubmit={onsubmit}>
         <TextInput name="username" placeholder="Stormlightnoway" handle_onchange={username_changed}/>
     </form>
    }
}

