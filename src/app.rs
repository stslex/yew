use yew::prelude::*;

pub fn my_avatar() -> Html {
    html! {
        <img src="https://github.com/stslex.png"
                alt="Avatar"
                style="position: absolute;
                top: 0; 
                left: 0; 
                padding: 2%;
                border-radius: 50%;"
            />
    }
}

pub fn title() -> Html {
    html! {
        <main>
            <h1 style="color: white;
                    position: absolute;
                    top: 30%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    text-align: center;">
                    { "Hello, Yew!" }
            </h1>
            <h2 style="color: white;
                    position: absolute;
                    top: 40%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    text-align: center;">
                    { "This is a simple example of Yew." }
            </h2>
        </main>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            { title()}
            { my_avatar() }
            <a href="https://github.com/stslex"
                style="color: white;
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                text-align: center;" // Add this line
            >
                { "gitHub stslex" }
            </a>
            <p style="position: absolute;
                top: 60%;
                left: 50%;
                transform: translate(-50%, -50%);
                color: white;">
                { "Additional text" }
            </p>
        </main>
    }
}
