use leptos::*;
use std::rc::Rc;
use validator::Validate;
use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use crate::app::models::{ EditPersonRequest, Person };
use crate::app::server_functions::persons::edit_person;


const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";


const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";


const UPDATE_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 top-20 z-50";


const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

#[component]
pub fn EditPersonModal(person:Rc<Person>, set_if_show_modal: WriteSignal<bool>, set_if_show_toast: WriteSignal<bool>, set_toast_message: WriteSignal<ToastMessage>, person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>) -> impl IntoView {
    view! {
        <div></div>
    }
}


