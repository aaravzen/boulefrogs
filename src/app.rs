use leptos::*;

mod club;
mod game;

enum Subject {
    Boulefrogs,
    Petanque
}

#[component]
pub fn App() -> impl IntoView {
    // let subject = create_signal

    view! { 
        <p>"Hello, world!"</p>
        <p>"Hello, world!"</p>
        <p>"Hello, world!"</p>
        <p>"Hello, world!"</p>
        <p>"Hello, world!"</p>
     }
}