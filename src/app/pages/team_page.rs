use leptos::*;

use crate::app::components::Header;

#[component]
pub fn TeamPage() -> impl IntoView {
    view! {
        
      <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
        <Header/>
        <div class="mt-20 ml-20 text-white">"This is Team Page"</div></div>
       
    }
}