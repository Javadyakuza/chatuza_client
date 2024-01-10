use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    let _greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
                // <main class="container">
                //     <div class="row">
                //         <a href="https://tauri.app" target="_blank">
                //             <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                //         </a>
                //         <a href="https://yew.rs" target="_blank">
                //             <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                //         </a>
                //     </div>

                //     <p>{"Click on the Tauri and Yew logos to learn more."}</p>

                //     <p>
                //         {"Recommended IDE setup: "}
                //         <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                //         {" + "}
                //         <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                //         {" + "}
                //         <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
                //     </p>

                //     <form class="row" onsubmit={greet}>
                //         <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                //         <button type="submit">{"Greet"}</button>
                //     </form>

                //     <p><b>{ &*greet_msg }</b></p>
                // </main>
                <div id="container">
            <aside>
                <header>
                    <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/ico_search.png"/>
                    <input type="text" placeholder="search"/>
                </header>
                <ul>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_01.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status orange"></span>
                                {"offline"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_02.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status green"></span>
                                {"online"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_03.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status orange"></span>
                                {"offline"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_04.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status green"></span>
                                {"online"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_05.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status orange"></span>
                                {"offline"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_06.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status green"></span>
                                {"online"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_07.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status green"></span>
                                {"online"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_08.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status green"></span>
                                {"online"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_09.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status green"></span>
                                {"online"}
                            </h3>
                        </div>
                    </li>
                    <li>
                        <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_10.jpg" alt=""/>
                        <div>
                            <h2>{"Prénom Nom"}</h2>
                            <h3>
                                <span class="status orange"></span>
                                {"offline"}
                            </h3>
                        </div>
                    </li>
                </ul>
            </aside>
            <main>
                <header>
                    <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/chat_avatar_01.jpg" alt=""/>
                    <div>
                        <h2>{"Chat with Vincent Porter"}</h2>
                        <h3>{"already 1902 messages"}</h3>
                    </div>
                    <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/ico_star.png" alt=""/>
                </header>
                <ul id="chat">
                    <li class="you">
                        <div class="entete">
                            <span class="status green"></span>
                            <h2>{"Vincent"}</h2>
                            <h3>{"10:12AM, Today"}</h3>
                        </div>
                        <div class="triangle"></div>
                        <div class="message">
    {"                        Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor.
    "}                    </div>
                    </li>
                    <li class="me">
                        <div class="entete">
                            <h3>{"10:12AM, Today"}</h3>
                            <h2>{"Vincent"}</h2>
                            <span class="status blue"></span>
                        </div>
                        <div class="triangle"></div>
                        <div class="message">
    {"                        Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor.
    "}                    </div>
                    </li>
                    <li class="me">
                        <div class="entete">
                            <h3>{"10:12AM, Today"}</h3>
                            <h2>{"Vincent"}</h2>
                            <span class="status blue"></span>
                        </div>
                        <div class="triangle"></div>
                        <div class="message">
                            {"OK"}
                        </div>
                    </li>
                    <li class="you">
                        <div class="entete">
                            <span class="status green"></span>
                            <h2>{"Vincent"}</h2>
                            <h3>{"10:12AM, Today"}</h3>
                        </div>
                        <div class="triangle"></div>
                        <div class="message">
    {"                        Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor.
    "}                    </div>
                    </li>
                    <li class="me">
                        <div class="entete">
                            <h3>{"10:12AM, Today"}</h3>
                            <h2>{"Vincent"}</h2>
                            <span class="status blue"></span>
                        </div>
                        <div class="triangle"></div>
                        <div class="message">
    {"                        Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor.
    "}                    </div>
                    </li>
                    <li class="me">
                        <div class="entete">
                            <h3>{"10:12AM, Today"}</h3>
                            <h2>{"Vincent"}</h2>
                            <span class="status blue"></span>
                        </div>
                        <div class="triangle"></div>
                        <div class="message">
                            {"OK"}
                        </div>
                    </li>
                </ul>
                <footer>
                    <textarea placeholder="Type your message"></textarea>
                    <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/ico_picture.png" alt=""/>
                    <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/1940306/ico_file.png" alt=""/>
                    <a href="#">{"Send"}</a>
                </footer>
            </main>
        </div>
            }
}
