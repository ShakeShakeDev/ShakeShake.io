use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::{hi_solid_icons}};

use crate::components::icon::{PhoneIcon, SocialMediaIcon};

struct UserData {
    username: String,
    phone_number: String,
    avatar: String,
    cover_image: String,
    links: HashMap<String, String>,
}

pub fn UserPublicPage(cx: Scope) -> Element {
    let route = use_route(&cx);
    let token = route.segment("token").unwrap();

    let _ = js_sys::eval("document.body.classList.add('bg-gray-100')");

    let user = if token.to_uppercase() == "YUKUNNB" {
        
        let mut links = HashMap::new();
        links.insert("snapchat".to_string(), "YuKun_Liu".to_string());
        links.insert("instagram".to_string(), "mrxiaozhuox".to_string());

        UserData {
            username: "YuKun Liu".into(),
            phone_number: "+1 6692939678".into(),
            avatar: "https://avatars.githubusercontent.com/u/41265098?v=4".into(),
            cover_image: "https://images.unsplash.com/photo-1499336315816-097655dcfbda?ixlib=rb-1.2.1&amp;ixid=eyJhcHBfaWQiOjEyMDd9&amp;auto=format&amp;fit=crop&amp;w=2710&amp;q=80".into(),
            links,
        }
    } else {
        return cx.render(rsx! { crate::pages::_404::NotFound {} });
    };

    cx.render(rsx! {
        div {
            div {
                img {
                    class: "absolute w-full h-[35vh] bg-center bg-cover",
                    src: "{user.cover_image}",
                }
            }
            div {
                class: "absolute py-64 w-full",
                div {
                    class: "relative bg-white dark:bg-gray-600 container mx-auto rounded-md h-[40rem] shadow-sm shadow-purple-400 dark:shadow-purple-700",
                    div {
                        class: "absolute left-1/2 transform -translate-x-1/2 -translate-y-1/2",
                        img {
                            class: "w-32 h-32 rounded-full",
                            src: "{user.avatar}"
                        }
                    }
                    div {
                        class: "absolute py-20 w-full",
                        div {
                            p {
                                class: "flex justify-center font-semibold text-2xl dark:text-white",
                                "{user.username}"
                            }
                            p {
                                class: "flex justify-center font-extralight text-gray-400",
                                "@SJSU 23 Computer Science"
                            }
                            p {
                                hr {
                                    class: "w-1/2 h-0.5 mx-auto my-4 bg-gray-100 border-0 rounded md:my-10 bg-[#7B45E7] dark:bg-gray-700"
                                }
                            }
                        }

                        div {
                            class: "grid gap-y-4",
                            div {
                                class: "flex justify-center",
                                div {
                                    class: "rounded-lg w-5/6 sm:w-1/2 border-solid border border-[#7B45E7]",
                                    div {
                                        class: "flex p-4 space-x-4",
                                        div {
                                            class: "flex-shrink-0",
                                            PhoneIcon {}
                                        }
                                        span {
                                            class: "inline-flex items-center font-semibold dark:text-white w-5/6",
                                            "{user.phone_number}"                                    
                                        }
                                        a {
                                            class: "flex-1 inline-flex items-center text-gray-600 dark:text-white",
                                            href: "javascript:navigator.clipboard.writeText('{user.phone_number}');",
                                            Icon {
                                                icon: hi_solid_icons::HiClipboardCopy
                                            }
                                        }
                                    }
                                }
                            }
    
                            user.links.iter().map(|(key, value)| {
                                rsx! {
                                    a {
                                        class: "flex justify-center",
                                        href: "/socialMedia/YuKun",
                                        key: "{key}",
                                        div {
                                            class: "rounded-lg w-5/6 sm:w-1/2 border-solid border border-[#7B45E7]",
                                            div {
                                                class: "flex p-4 space-x-4",
                                                div {
                                                    class: "flex-shrink-0",
                                                    SocialMediaIcon {
                                                        name: key.to_string()
                                                    }
                                                }
                                                span {
                                                    class: "inline-flex items-center font-bold text-lg dark:text-white w-5/6",
                                                    "@{value}"                                    
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }

                    }
                }
            }

        }
    })
}
