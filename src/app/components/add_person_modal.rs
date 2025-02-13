use crate::app::models::{AddPersonRequest, Person};
use crate::app::server_functions::persons::add_person;
use leptos::*;
use validator::Validate;


#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>
) -> impl IntoView {
    const INPUT_STYLE: &str = "w-full h-23 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";

    const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";
}