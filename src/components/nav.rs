use dioxus::prelude::*;
use dioxus_free_icons::{icons::hi_solid_icons, Icon};

use crate::hooks::mode::{is_dark, mode};

pub fn Navbar(cx: Scope) -> Element {

    // condtional display modeIcon (for different dark-mode)
    let mode_icon_node = if is_dark(&cx) {
        rsx! {
            Icon {
                icon: hi_solid_icons::HiSun
            }
        }
    } else {
        rsx! {
            Icon {
                icon: hi_solid_icons::HiMoon
            }
        }
    };


    cx.render(rsx! {
        nav {
            class: "dark:bg-gray-800",
            div {
                class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex h-16 items-center justify-between",
                    div {
                        class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button {
                            class: "inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white",
                            "aria-controls": "mobile-menu",
                            "aria-expanded": "false",
                            r#type: "button",span {
                                class: "sr-only",
                                "Open main menu"
                            }
                            Icon {
                                icon: hi_solid_icons::HiMinus
                            }
                        }
                    }
                    div {
                        class: "flex flex-1 items-center justify-center sm:items-stretch sm:justify-start",
                        div {
                            class: "flex flex-shrink-0 items-center",
                            img {
                                class: "block h-8 w-auto lg:hidden",
                                alt: "ShakeShake.io",
                                src: "/assets/logo.png",
                            }
                            img {
                                class: "hidden h-8 w-auto lg:block",
                                alt: "ShakeShake.io",
                                src: "/assets/logo.png",
                            }
                        }
                        div {
                            class: "hidden sm:ml-6 sm:block",
                            div {
                                class: "flex space-x-4",
                                a {
                                    class: "dark:text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                                    "aria-current": "page",
                                    href: "#",
                                    "Discovery"
                                }
                                a {
                                    class: "dark:text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                                    href: "#",
                                    "Contact"
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                        button {
                            class: "rounded-full p-1 text-gray-500 hover:text-black dark:text-gray-300 dark:bg-gray-800 dark:hover:text-white",
                            r#type: "button",
                            onclick: move |_| {
                                mode(&cx, !is_dark(&cx));
                                cx.needs_update();
                            },
                            mode_icon_node
                        }
                        button {
                            class: "rounded-full p-1 text-gray-500 hover:text-black dark:text-gray-300 dark:bg-gray-800 dark:hover:text-white",
                            r#type: "button",
                            Icon {
                                icon: hi_solid_icons::HiBell
                            }
                        }
                        div {
                            class: "relative ml-3",
                            div {
                                button {
                                    class: "flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800",
                                    id: "user-menu-button",
                                    "aria-expanded": "false",
                                    "aria-haspopup": "true",
                                    r#type: "button",
                                    img {
                                        class: "h-8 w-8 rounded-full",
                                        alt: "",
                                        src: "https://avatars.githubusercontent.com/u/41265098?v=4",
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "sm:hidden",
                id: "mobile-menu",
                div {
                    class: "space-y-1 px-2 pt-2 pb-3",
                    a {
                        class: "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium",
                        "aria-current": "page",
                        href: "#",
                        "Discovery"
                    }
                    a {
                        class: "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium",
                        href: "#",
                        "Contact"
                    }
                }
            }
        }
    })
}
