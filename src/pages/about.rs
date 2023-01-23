use dioxus::prelude::*;

pub fn AboutUS(cx: Scope) -> Element {
    cx.render(rsx! {
        br {}
        div {
            class: "container mx-auto px-3 sm:px-40 dark:text-gray-100 sm:font-semibold text-base sm:text-xl",
            div {
                class: "flex flex-col space-y-6",
                h1 {
                    class: "underline font-bold text-4xl",
                    "About Us"
                }
                p {
                    "We are a passionate and diverse team made of students at Stanford, SJSU, Cornell, LSE, etc; and engineers from Meta (Facebook), Google, and Amazon. We have experience in consulting, financial advisory, software development, artificial intelligence, and interaction design at Fortune 500 firms."
                }
                p {
                    "We strive to establish links and smart lives across 1.5 billion users over the globe with our product ShakeShake, which allows the linkage of all social media platforms via ONE SINGLE Shake on phone, simplifying the exchange of contacts to the greatest extent in both social and professional settings. "
                }
            }
        }
    })
}