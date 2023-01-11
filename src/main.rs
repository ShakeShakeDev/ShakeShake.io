#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastManager};

mod components;
mod hooks;
mod pages;

use hooks::mode::init_mode_info;

use crate::pages::{discovery::Discovery, user::UserPublicPage, post::TermsOfService};

static TOAST_MANAGER: dioxus::fermi::AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Dioxus Starter: https://github.com/mrxiaozhuox/dioxus-starter");
    dioxus::web::launch(App)
}

fn App(cx: Scope) -> Element {

    // init mode information
    init_mode_info(&cx);
    
    cx.render(rsx! {
        // dioxus toast manager init
        ToastFrame {
            manager: use_atom_ref(&cx, TOAST_MANAGER),
            maximum: 6,
        }

        // dioxus router info
        Router {
            components::nav::Navbar {}
            Route {
                to: "/",
                Discovery {}
            }
            Route {
                to: "/user/tempPage/:token",
                UserPublicPage {}
            }
            Route {
                to: "/terms-of-service",
                TermsOfService {}
            }
            Route {
                to: "/socialMedia/YuKun",
                div {
                    br {}
                    div {
                        class: "flex justify-center font-bold text-2xl dark:text-white",
                        "假装这是我的个人主页 -- YuKun Liu"
                    }
                }
            }
            // 404 page
            Route { to: "", pages::_404::NotFound {} }
        }
    })
}
