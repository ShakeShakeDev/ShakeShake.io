use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_brands_icons, Icon};

use crate::components::nav::Navbar;

pub fn Discovery(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {}
        div {
            div {
                class: "relative px-6 lg:px-8",
                div {
                    class: "mx-auto max-w-3xl pt-20 pb-32 sm:pt-30 sm:pb-40",
                    div {
                        div {
                            class: "hidden sm:mb-8 sm:flex sm:justify-center",
                            div {
                                class: "relative overflow-hidden rounded-full py-1.5 px-4 text-sm leading-6 ring-1 ring-gray-900/10 dark:ring-gray-100",
                                span {
                                    class: "text-gray-600 dark:text-gray-200",
                                    "Announcing our next round of funding."
                                }
                            }
                        }
                        div {
                            h1 {
                                class: "text-4xl font-bold tracking-tight sm:text-center sm:text-6xl dark:text-white",
                                "Shake to link the world"
                            }
                            p {
                                class: "mt-6 text-lg leading-8 text-gray-600 dark:text-gray-300 sm:text-center",
                                "SHAKESHAKE interconnects all social media platforms via ONE shake. By simply shaking the phone, SHAKESHAKE's smart detector recognises synchronous gestures which allow individuals and collectives to connect on their default social links."
                            }
                            div {
                                class: "mt-8 flex gap-x-4 sm:justify-center",
                                a {
                                    class: "inline-block rounded-lg bg-indigo-600 px-4 py-1.5 text-base font-semibold leading-7 text-white shadow-sm ring-1 ring-indigo-600 hover:bg-indigo-700 hover:ring-indigo-700",
                                    href: "#",
                                    div {
                                        class: "flex items-center",
                                        Icon {
                                            icon: fa_brands_icons::FaApple
                                        }
                                        span {
                                            class: "ml-2",
                                            "Apple Store"
                                        }
                                    }
                                }
                                a {
                                    class: "inline-block rounded-lg px-4 py-1.5 text-base font-semibold leading-7 text-gray-900 dark:text-white ring-1 ring-gray-900/10 hover:ring-gray-900/20 dark:ring-gray-400",
                                    href: "#",
                                    div {
                                        class: "flex items-center",
                                        Icon {
                                            icon: fa_brands_icons::FaGoogle
                                        }
                                        span {
                                            class: "ml-2",
                                            "Google Store"
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            class: "absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]",
                            "icons::icon_1"
                        }
                    }
                }
            }
        }
    })
}
