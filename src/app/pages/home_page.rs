use leptos::*;
use crate::app::components::{Header, DashboardHeader};
use crate::app::server_functions::get_persons;

#[component]
pub fn HomePage() -> impl IntoView {

    let get_persons_info = create_resource(|| (), |_|{
      async move { get_persons().await }
    });

    view! {
      <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
          <Header/>
          <DashboardHeader />
      </div>
        
    }
}