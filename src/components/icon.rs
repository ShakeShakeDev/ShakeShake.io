use dioxus::prelude::*;

use crate::hooks::mode::is_dark;

#[inline_props]
pub fn PhoneIcon(cx: Scope) -> Element {

    log::debug!("{}", is_dark(&cx));

    let color = if !is_dark(&cx) {
        "#7B45E7"
    } else {
        "#C1AEE8"
    };

    cx.render(rsx! {
        span {
            svg {
                width: "37",
                height: "37",
                "viewBox": "0 0 37 37",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M16.5704 36.3792C13.5438 34.5791 0.450203 25.6995 0.00151479 7.7224C-0.0133463 7.22391 0.0812883 6.72817 0.278886 6.26938C1.04574 4.42082 3.47681 0.279708 9.19555 2.78213C9.83943 3.07888 10.3812 3.55601 10.7537 4.15443L13.2745 8.1906C13.7089 8.87001 13.8933 9.67683 13.7966 10.4751C13.6875 11.3073 13.4058 12.1083 12.9692 12.8279C12.5326 13.5474 11.9506 14.17 11.2595 14.6565C11.2595 14.6565 8.81213 15.9481 9.17108 17.9258C9.42397 19.3627 10.9658 21.623 11.7898 22.7692C12.2256 23.3855 12.8511 23.8458 13.573 24.0815C14.2949 24.3172 15.0745 24.3156 15.7954 24.077H15.8606C18.2101 23.2213 19.9396 22.7047 21.767 24.3353C23.0233 25.4573 24.2715 27.9113 24.9323 29.3401C25.2944 30.1169 25.361 30.9969 25.1199 31.8183C24.5162 33.8929 22.7623 37.5255 17.9898 36.9362C17.4811 36.8559 16.9964 36.6658 16.5704 36.3792V36.3792Z",
                    fill: "{color}",
                }
                path {
                    d: "M22.1758 8.02344C22.1758 8.02344 28.5961 10.1707 29.1264 18.2592",
                    stroke: "{color}",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    "stroke-width": "3",
                }
                path {
                    d: "M23.3594 2C26.7911 3.49691 29.7141 5.94062 31.7787 9.03865C33.8433 12.1367 34.962 15.7578 35.0008 19.4685",
                    stroke: "{color}",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    "stroke-width": "3",
                }
            }
        }
    })
}

#[inline_props]
pub fn SocialMediaIcon(cx: Scope, name: String) -> Element {

    let color = if !is_dark(&cx) {
        "#7B45E7"
    } else {
        "#C1AEE8"
    };

    cx.render(
        match name.to_lowercase().as_str() {
            "instagram" => {
                rsx! {
                    svg {
                        width: "38",
                        height: "38",
                        "viewBox": "0 0 38 38",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M25.08 19C25.08 20.2025 24.7234 21.378 24.0553 22.3779C23.3873 23.3777 22.4377 24.157 21.3267 24.6172C20.2157 25.0774 18.9933 25.1978 17.8139 24.9632C16.6344 24.7286 15.5511 24.1495 14.7008 23.2992C13.8505 22.4489 13.2714 21.3656 13.0368 20.1861C12.8022 19.0067 12.9226 17.7843 13.3828 16.6733C13.843 15.5623 14.6223 14.6127 15.6221 13.9447C16.622 13.2766 17.7975 12.92 19 12.92C20.611 12.925 22.1545 13.5672 23.2937 14.7063C24.4328 15.8455 25.075 17.389 25.08 19ZM38 10.64V27.36C38 30.1819 36.879 32.8882 34.8836 34.8836C32.8882 36.879 30.1819 38 27.36 38H10.64C7.8181 38 5.11177 36.879 3.11638 34.8836C1.121 32.8882 0 30.1819 0 27.36V10.64C0 7.8181 1.121 5.11177 3.11638 3.11638C5.11177 1.121 7.8181 0 10.64 0H27.36C30.1819 0 32.8882 1.121 34.8836 3.11638C36.879 5.11177 38 7.8181 38 10.64ZM28.12 19C28.12 17.1962 27.5851 15.433 26.583 13.9332C25.5809 12.4334 24.1565 11.2645 22.4901 10.5742C20.8236 9.88395 18.9899 9.70334 17.2208 10.0552C15.4517 10.4071 13.8266 11.2757 12.5512 12.5512C11.2757 13.8266 10.4071 15.4517 10.0552 17.2208C9.70334 18.9899 9.88395 20.8236 10.5742 22.4901C11.2645 24.1565 12.4334 25.5809 13.9332 26.583C15.433 27.5851 17.1962 28.12 19 28.12C21.4188 28.12 23.7385 27.1591 25.4488 25.4488C27.1591 23.7385 28.12 21.4188 28.12 19ZM31.16 9.12C31.16 8.66906 31.0263 8.22824 30.7757 7.8533C30.5252 7.47836 30.1691 7.18612 29.7525 7.01355C29.3359 6.84099 28.8775 6.79584 28.4352 6.88381C27.9929 6.97178 27.5867 7.18893 27.2678 7.5078C26.9489 7.82666 26.7318 8.23292 26.6438 8.67519C26.5558 9.11747 26.601 9.5759 26.7736 9.99252C26.9461 10.4091 27.2384 10.7652 27.6133 11.0158C27.9882 11.2663 28.4291 11.4 28.88 11.4C29.4847 11.4 30.0646 11.1598 30.4922 10.7322C30.9198 10.3046 31.16 9.72469 31.16 9.12Z",
                            fill: "{color}",
                        }
                    }
                }       
            },
            "snapchat" => {
                rsx! {
                    svg {
                        width: "44",
                        height: "44",
                        "viewBox": "0 0 44 44",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M40.2823 12.2441C40.2803 11.5843 40.2202 10.9261 40.1026 10.2769C39.9707 9.48238 39.7198 8.71229 39.3583 7.99256C38.9627 7.23656 38.4476 6.54939 37.833 5.95756C36.9047 5.04905 35.7581 4.39493 34.5036 4.05823C33.3519 3.77891 32.1684 3.65255 30.9836 3.6824L30.9781 3.66406H13.0133V3.6824C12.3393 3.67755 11.6655 3.70815 10.9948 3.77406C10.2599 3.85198 9.53646 4.01439 8.83879 4.25806C7.68916 4.69135 6.6579 5.38963 5.82873 6.2962C4.99956 7.20277 4.39586 8.29209 4.06662 9.47573C3.78924 10.6185 3.66289 11.7926 3.69079 12.9682L3.67979 31.0046C3.67703 31.822 3.73217 32.6387 3.84479 33.4484C3.96795 34.3039 4.22144 35.1355 4.59645 35.9142C5.12436 36.9546 5.87635 37.865 6.79829 38.5799C7.45747 39.1045 8.2041 39.5085 9.00379 39.7734C10.1768 40.1357 11.3988 40.3151 12.6265 40.3051C13.3965 40.3106 14.1646 40.3307 14.9328 40.3271C20.519 40.3032 26.1051 40.3674 31.6895 40.2922C32.428 40.2764 33.1639 40.1985 33.8895 40.0594C35.2592 39.8178 36.5339 39.1973 37.569 38.2682C38.7882 37.2079 39.6394 35.788 40 34.2129C40.2223 33.1582 40.3287 32.0824 40.3171 31.0046V30.7901C40.3171 30.7057 40.286 12.4861 40.2823 12.2459V12.2441ZM35.3763 29.2024C35.1416 29.7524 33.9573 30.2071 31.9516 30.5169C31.7646 30.5462 31.684 30.8506 31.5758 31.3492C31.5318 31.5546 31.4841 31.7562 31.4236 31.9671C31.4051 32.0563 31.3547 32.1358 31.2818 32.1904C31.2089 32.2451 31.1185 32.2713 31.0276 32.2641H30.9946C30.8034 32.2567 30.6132 32.2321 30.4263 32.1907C29.9312 32.0876 29.4269 32.0348 28.9211 32.0331C28.5574 32.034 28.1945 32.0646 27.8358 32.1247C27.0792 32.3165 26.3755 32.6762 25.777 33.1771C24.7651 34.0403 23.494 34.541 22.1653 34.5997C22.0968 34.5995 22.0283 34.5977 21.96 34.5942C21.9154 34.598 21.8708 34.5998 21.8261 34.5997C20.4973 34.5415 19.226 34.0408 18.2145 33.1771C17.6157 32.6764 16.9121 32.3168 16.1556 32.1247C15.797 32.0646 15.434 32.0339 15.0703 32.0331C14.5647 32.0385 14.0609 32.0956 13.567 32.2036C13.3796 32.2468 13.1888 32.2738 12.9968 32.2842C12.9 32.2956 12.8025 32.2705 12.7232 32.2139C12.6439 32.1573 12.5885 32.0732 12.5678 31.9781C12.5082 31.7726 12.4574 31.5646 12.4156 31.3547C12.3056 30.8542 12.2268 30.5481 12.0398 30.5187C10.0341 30.2107 8.84979 29.7542 8.61329 29.2024C8.58905 29.1473 8.57478 29.0883 8.57112 29.0282C8.5669 28.9489 8.59205 28.8709 8.64177 28.809C8.69149 28.7471 8.76229 28.7057 8.84062 28.6927C10.4993 28.3926 11.9939 27.504 13.05 26.1902C13.6299 25.5176 14.1119 24.7665 14.4818 23.9591L14.4891 23.9444C14.5986 23.7655 14.6676 23.5648 14.6913 23.3564C14.7151 23.1481 14.693 22.937 14.6266 22.7381C14.37 22.1294 13.5138 21.8581 12.9473 21.6784C12.8194 21.6399 12.6928 21.5971 12.5678 21.5501C12.0655 21.3521 11.2405 20.9322 11.3505 20.3547C11.4344 20.137 11.5844 19.951 11.7793 19.8228C11.9743 19.6946 12.2045 19.6306 12.4376 19.6397C12.5511 19.6371 12.6638 19.6597 12.7676 19.7057C13.1899 19.9221 13.6539 20.0447 14.128 20.0651C14.4103 20.0853 14.69 19.9997 14.9126 19.8249C14.898 19.5572 14.8815 19.2896 14.8631 19.0219C14.6057 17.2151 14.7161 15.3748 15.1876 13.6117C15.7419 12.3456 16.6568 11.2708 17.8182 10.5216C18.9796 9.77232 20.336 9.38178 21.718 9.39873L22.2588 9.39506C23.6423 9.37738 25.0005 9.76748 26.1638 10.5167C27.327 11.2659 28.244 12.3411 28.8001 13.6081C29.2716 15.3737 29.3813 17.2164 29.1228 19.0256L29.1173 19.1117L29.0751 19.8249C29.2771 19.985 29.5288 20.0693 29.7865 20.0632C30.2336 20.0296 30.6697 19.9081 31.0698 19.7057C31.2015 19.6482 31.344 19.6195 31.4878 19.6214C31.6517 19.6208 31.8142 19.652 31.9663 19.7131L31.9736 19.7167C32.1456 19.7577 32.3015 19.849 32.4213 19.9789C32.5412 20.1089 32.6196 20.2717 32.6465 20.4464C32.652 20.7269 32.4448 21.1449 31.42 21.5501C31.3155 21.5904 31.1816 21.6344 31.0405 21.6784C30.4721 21.8581 29.6178 22.1294 29.3611 22.7381C29.2946 22.9366 29.2723 23.1473 29.2958 23.3553C29.3192 23.5634 29.3878 23.7638 29.4968 23.9426L29.5041 23.9591C30.0165 25.1298 30.7926 26.1662 31.7719 26.9873C32.7511 27.8085 33.9069 28.3922 35.149 28.6927C35.2269 28.7061 35.2973 28.7477 35.3467 28.8095C35.396 28.8714 35.4209 28.9492 35.4166 29.0282C35.4131 29.0889 35.3988 29.1485 35.3745 29.2042L35.3763 29.2024Z",
                            fill: "{color}",
                        }
                    }
                }
            }
            "tiktok" => {
                rsx! {
                    svg {
                        width: "43",
                        height: "41",
                        "viewBox": "00 43 41",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M29.6307 9.79643C29.6307 9.79643 30.5193 10.638 29.6307 9.79643C28.4398 8.4829 27.7834 6.79601 27.7839 5.0498H22.4004V25.9215C22.3588 27.0509 21.8653 28.1208 21.0236 28.9057C20.1819 29.6906 19.0577 30.1293 17.8879 30.1295C15.4139 30.1295 13.358 28.177 13.358 25.7532C13.358 22.8581 16.2502 20.6867 19.2294 21.5788V16.2599C13.2187 15.4856 7.95703 19.9966 7.95703 25.7532C7.95703 31.3582 12.7657 35.3474 17.8705 35.3474C23.3412 35.3474 27.7839 31.0552 27.7839 25.7532V15.1658C29.967 16.6805 32.588 17.4931 35.2757 17.4886V12.2876C35.2757 12.2876 32.0002 12.4391 29.6307 9.79643Z",
                            fill: "{color}",
                        }
                    }
                }
            }
            "twitter" => {
                rsx! {
                    svg {
                        fill: "none",
                        height: "40",
                        "viewBox": "0 0 39 40",
                        width: "39",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M33.7668 13.0763C33.7823 13.3922 33.7882 13.712 33.7882 14.0279C33.7882 23.7584 26.384 34.9748 12.8452 34.9748C8.68585 34.9748 4.819 33.756 1.56055 31.6656C3.48788 31.8987 5.44225 31.7497 7.31195 31.227C9.18165 30.7044 10.93 29.8184 12.4571 28.6197C10.9214 28.5907 9.43304 28.0825 8.20025 27.1661C6.96747 26.2497 6.05187 24.971 5.58145 23.5088C6.68527 23.7152 7.8213 23.6719 8.9062 23.382C7.24014 23.0442 5.74213 22.1407 4.66601 20.8247C3.5899 19.5087 3.00188 17.8611 3.0016 16.1612V16.0695C4.02378 16.6381 5.167 16.9544 6.3361 16.9919C4.77585 15.9517 3.67138 14.3553 3.24811 12.5285C2.82484 10.7017 3.11469 8.78227 4.0585 7.16192C5.90669 9.43652 8.21268 11.2969 10.8267 12.6222C13.4407 13.9475 16.3043 14.7082 19.2314 14.8547C19.1048 14.304 19.0413 13.7407 19.0423 13.1757C19.0423 12.2094 19.2327 11.2525 19.6025 10.3597C19.9724 9.46697 20.5145 8.65582 21.1979 7.9726C21.8813 7.28938 22.6926 6.74748 23.5855 6.37785C24.4783 6.00823 25.4353 5.81811 26.4016 5.81837C27.4087 5.81673 28.4054 6.02203 29.3299 6.42153C30.2543 6.82104 31.0869 7.40624 31.7758 8.14082C33.4238 7.81458 35.0044 7.21058 36.4499 6.35462C35.9001 8.05932 34.7498 9.50625 33.2129 10.4262C34.6721 10.2532 36.097 9.86282 37.4406 9.26792C36.4536 10.7469 35.2093 12.0368 33.7668 13.0763Z",
                            fill: "{color}",
                        }
                    }
                }
            }
            "facebook" => {
                rsx! {                                      
                    svg {
                        fill: "none",
                        height: "45",
                        "viewBox": "0 0 44 45",
                        width: "44",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M39.6299 22.396C39.6299 12.4527 31.56 4.38281 21.6167 4.38281C11.6734 4.38281 3.60352 12.4527 3.60352 22.396C3.60352 31.1144 9.80005 38.3737 18.0141 40.0489V27.8H14.4114V22.396H18.0141V17.8927C18.0141 14.4162 20.8421 11.5881 24.3187 11.5881H28.822V16.992H25.2193C24.2286 16.992 23.418 17.8026 23.418 18.7934V22.396H28.822V27.8H23.418V40.3191C32.5147 39.4185 39.6299 31.7448 39.6299 22.396Z",
                            fill: "#7B45E7",
                        }
                    }
                }
            }
            "discord" => {                
                rsx! {
                    svg {
                        fill: "none",
                        height: "24",
                        "viewBox": "0 0 32 24",
                        width: "32",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M27.0581 2.74397C25.1052 1.85773 22.9909 1.21449 20.7884 0.842839C20.7691 0.842238 20.7499 0.845766 20.7321 0.853175C20.7143 0.860584 20.6985 0.871694 20.6856 0.885722C20.4213 1.35743 20.113 1.97208 19.9074 2.44379C17.5713 2.10073 15.1956 2.10073 12.8595 2.44379C12.6539 1.95779 12.3455 1.35743 12.0666 0.885722C12.0519 0.857134 12.0078 0.842839 11.9638 0.842839C9.7613 1.21449 7.6616 1.85773 5.69405 2.74397C5.67936 2.74397 5.66468 2.75827 5.65 2.77256C1.65617 8.59031 0.554924 14.2508 1.0982 19.8542C1.0982 19.8828 1.11289 19.9113 1.14225 19.9256C3.78523 21.8125 6.32543 22.956 8.83625 23.7136C8.8803 23.7279 8.92435 23.7136 8.93904 23.685C9.52636 22.8988 10.055 22.0698 10.5101 21.1978C10.5395 21.1406 10.5101 21.0835 10.4514 21.0692C9.61446 20.7547 8.82157 20.3831 8.04336 19.9542C7.98463 19.9256 7.98463 19.8399 8.02868 19.797C8.19019 19.6826 8.35171 19.554 8.51322 19.4396C8.54259 19.411 8.58664 19.411 8.61601 19.4253C13.667 21.6695 19.1145 21.6695 24.1068 19.4253C24.1362 19.411 24.1802 19.411 24.2096 19.4396C24.3711 19.5683 24.5326 19.6826 24.6941 19.8113C24.7528 19.8542 24.7528 19.9399 24.6794 19.9685C23.9159 20.4116 23.1083 20.769 22.2714 21.0835C22.2127 21.0978 22.198 21.1692 22.2127 21.2121C22.6825 22.0841 23.2111 22.9131 23.7838 23.6993C23.8278 23.7136 23.8719 23.7279 23.9159 23.7136C26.4414 22.956 28.9816 21.8125 31.6246 19.9256C31.654 19.9113 31.6686 19.8828 31.6686 19.8542C32.3147 13.3789 30.5968 7.76125 27.1168 2.77256C27.1022 2.75827 27.0875 2.74397 27.0581 2.74397ZM11.2737 16.4378C9.7613 16.4378 8.49854 15.0799 8.49854 13.4075C8.49854 11.735 9.73193 10.3771 11.2737 10.3771C12.8301 10.3771 14.0635 11.7493 14.0488 13.4075C14.0488 15.0799 12.8154 16.4378 11.2737 16.4378ZM21.5079 16.4378C19.9955 16.4378 18.7327 15.0799 18.7327 13.4075C18.7327 11.735 19.9661 10.3771 21.5079 10.3771C23.0643 10.3771 24.2977 11.7493 24.283 13.4075C24.283 15.0799 23.0643 16.4378 21.5079 16.4378Z",
                            fill: "#7B45E7",
                        }
                    }
                }
            }
            "linkedin" => {
                rsx! {
                    svg {
                        fill: "none",
                        height: "39",
                        "viewBox": "0 0 39 39",
                        width: "39",
                        xmlns: "http://www.w3.org/2000/svg",path {
                            d: "M33.8807 0.0883789C35.0009 0.0883789 36.0753 0.533391 36.8674 1.32552C37.6595 2.11764 38.1045 3.192 38.1045 4.31223V33.8792C38.1045 34.9995 37.6595 36.0738 36.8674 36.8659C36.0753 37.6581 35.0009 38.1031 33.8807 38.1031H4.3137C3.19346 38.1031 2.11911 37.6581 1.32698 36.8659C0.534856 36.0738 0.0898438 34.9995 0.0898438 33.8792V4.31223C0.0898438 3.192 0.534856 2.11764 1.32698 1.32552C2.11911 0.533391 3.19346 0.0883789 4.3137 0.0883789H33.8807ZM32.8247 32.8233V21.63C32.8247 19.8041 32.0994 18.0529 30.8082 16.7617C29.517 15.4705 27.7658 14.7452 25.9398 14.7452C24.1447 14.7452 22.0539 15.8434 21.0402 17.4907V15.1464H15.1479V32.8233H21.0402V22.4115C21.0402 20.7853 22.3496 19.4548 23.9757 19.4548C24.7599 19.4548 25.512 19.7663 26.0665 20.3208C26.6209 20.8752 26.9324 21.6273 26.9324 22.4115V32.8233H32.8247ZM8.28412 11.8307C9.22512 11.8307 10.1276 11.4569 10.793 10.7915C11.4584 10.1261 11.8322 9.22366 11.8322 8.28266C11.8322 6.31857 10.2482 4.7135 8.28412 4.7135C7.33752 4.7135 6.4297 5.08954 5.76035 5.75888C5.091 6.42823 4.71497 7.33606 4.71497 8.28266C4.71497 10.2468 6.32003 11.8307 8.28412 11.8307ZM11.2197 32.8233V15.1464H5.36966V32.8233H11.2197Z",
                            fill: "#7B45E7",
                        }
                    }
                }
            }
            "email" => {
                rsx! {
                    svg {
                        fill: "none",
                        height: "55",
                        "viewBox": "0 0 51 51",
                        width: "55",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M43.9488 12H12.8832C10.7474 12 9.01942 13.7942 9.01942 15.9872L9 39.9105C9 42.1035 10.7474 43.8977 12.8832 43.8977H43.9488C46.0846 43.8977 47.832 42.1035 47.832 39.9105V15.9872C47.832 13.7942 46.0846 12 43.9488 12ZM43.1722 20.4728L29.4451 29.2846C28.8238 29.6833 28.0083 29.6833 27.387 29.2846L13.6598 20.4728C13.4652 20.3606 13.2947 20.209 13.1587 20.0272C13.0227 19.8453 12.9241 19.6371 12.8688 19.415C12.8135 19.1929 12.8026 18.9616 12.8369 18.735C12.8712 18.5085 12.9499 18.2915 13.0682 18.0971C13.1865 17.9027 13.3421 17.7349 13.5253 17.604C13.7086 17.4731 13.9159 17.3818 14.1346 17.3355C14.3532 17.2892 14.5788 17.289 14.7975 17.3348C15.0163 17.3806 15.2237 17.4715 15.4073 17.602L28.416 25.9553L41.4247 17.602C41.6083 17.4715 41.8157 17.3806 42.0345 17.3348C42.2533 17.289 42.4788 17.2892 42.6975 17.3355C42.9161 17.3818 43.1234 17.4731 43.3067 17.604C43.49 17.7349 43.6455 17.9027 43.7638 18.0971C43.8822 18.2915 43.9608 18.5085 43.9951 18.735C44.0294 18.9616 44.0186 19.1929 43.9632 19.415C43.9079 19.6371 43.8093 19.8453 43.6733 20.0272C43.5374 20.209 43.3669 20.3606 43.1722 20.4728Z",
                            fill: "#7B45E7",
                        }
                    }
                }
            }
            _ => {
                rsx! {
                    div {}
                }
            }
        }
    )
}