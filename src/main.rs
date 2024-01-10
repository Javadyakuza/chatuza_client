mod app;
use app::App;
use modules::solana_wallet;

fn main() {
    yew::Renderer::<App>::new().render();
}
