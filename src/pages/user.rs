use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::{hi_solid_icons}};
use serde::{Serialize, Deserialize};

use crate::components::icon::{PhoneIcon, SocialMediaIcon};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct UserData {
    #[serde(rename(deserialize = "Username"))]
    username: String,

    #[serde(rename(deserialize = "Profile"))]
    profile: UserDataProfile,

    #[serde(rename(deserialize = "Links"))]
    links: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct UserDataProfile {
    #[serde(rename(deserialize = "Avatar"))]
    avatar: String,
    #[serde(rename(deserialize = "CoverImage"))]
    cover_image: String,
    #[serde(rename(deserialize = "Introduction"))]
    introduction: String,
}

pub fn UserPublicPage(cx: Scope) -> Element {
    let route = use_route(&cx);
    let token = route.segment("token").unwrap();

    let _ = js_sys::eval("document.body.classList.add('bg-gray-100')");

    let url = format!("https://api.shakeshake.io/user/tempHomePage/{}", token);
    let res = use_future(&cx, (), |_| {
        async move {
            let resp = reqwasm::http::Request::get(&url).send().await;
            if let Ok(res) = resp {
                let data = res.json::<UserData>().await;
                log::info!("{:?}", data);
                return data.ok();
            } else {
                return None;
            }
        }
    });

    match res.value() {
        Some(user) => {

            if user.is_none() {
                return cx.render(rsx! {
                        crate::pages::_404::TokenNotFound {}
                    });
                }

                let user = user.clone().unwrap();

                cx.render(rsx! {
                    div {
                        div {
                            img {
                                class: "absolute w-full h-[35vh] bg-center bg-cover",
                                src: "{user.profile.cover_image}",
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
                                        src: "{user.profile.avatar}"
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
                                            "{user.profile.introduction}"
                                        }
                                        p {
                                            hr {
                                                class: "w-1/2 h-0.5 mx-auto my-4 bg-gray-100 border-0 rounded md:my-10 bg-[#7B45E7] dark:bg-gray-700"
                                            }
                                        }
                                    }
            
                                    div {
                                        class: "grid gap-y-4",
                                        user.links.iter().map(|(key, value)| {

                                            if key == "phoneNumber" {

                                                let region_number = value.split("!").collect::<Vec<&str>>();
                                                let phone_number = region_number[1];
                                                let region_number = region_number[0];

                                                return rsx! {
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
                                                                    "+{region_number} {phone_number}"
                                                                }
                                                                a {
                                                                    class: "flex-1 inline-flex items-center text-gray-600 dark:text-white",
                                                                    href: "javascript:navigator.clipboard.writeText('+{region_number} {phone_number}');",
                                                                    Icon {
                                                                        icon: hi_solid_icons::HiClipboardCopy
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            let (name, link) = crate::utils::generate_link(&key.to_lowercase(), value);

                                            rsx! {
                                                a {
                                                    class: "flex justify-center",
                                                    href: "{link}",
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
                                                                "{name}"                                    
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

            },
            None => {
                return cx.render(rsx! { crate::pages::_404::TokenNotFound {} });
            },
        }
}
