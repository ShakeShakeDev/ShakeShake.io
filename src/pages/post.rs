use dioxus::prelude::*;

pub fn TermsOfService (cx: Scope) -> Element {
    cx.render(rsx! {
        br {}
        div {
            class: "container mx-auto px-3 sm:px-40 dark:text-gray-100 font-semibold text-base sm:text-xl",
            div {
                class: "flex flex-col space-y-6",
                h1 {
                    class: "underline font-bold text-5xl",
                    "Terms of Service"
                }
                span {
                    class: "text-gray-600 dark:text-gray-300",
                    "Last updated January 9th, 2023"
                }
                p {
                    "Shakeshake is licensed to You by Shakeshake, located and registered at 1 Washington Sq, San Jose, CA 95192, USA (\"Licensor\"), for use only in accordance with the terms of this License Agreement.
                    You acknowledge that you have read, understand, and agree to be bound by the terms and conditions of this License Agreement by downloading the Licensed Application from the Apple App Store or the Google Play Store, as applicable, and any updates thereto (as permitted by this License Agreement).
                    This License Agreement refers to the App Store and Play Store as \"Services.\""
                }
                p {
                    "The parties to this license agreement acknowledge that the Services are not a party to it and are not subject to any of its terms and conditions, including those relating to warranty, liability, maintenance, and support of the Licensed Application. The Licensed Application and its contents are exclusively the responsibility of Shakeshake and not of the Services. This License Agreement may not establish usage guidelines for the Licensed Application that are inconsistent with the most recent versions of the Apple Media Services Terms (https://www.apple.com/legal/ internet-services/itunes/us/terms.html) and Conditions and Google Play Terms of Service (https://play.google.com/intl/en_US/about/play-terms/) Usage Rules. Shakeshake admits that it had a chance to review the Usage Rules and that this License Agreement does not infringe on them."
                }
                p {
                    "Shakeshake is licensed to You for use solely in accordance with the terms of this License Agreement when purchased or downloaded through the Services. All rights not specifically granted to you are reserved by the licensor. Shakeshake must be used on devices running either the Google or Apple operating systems (iOS and Mac OS) (\"Android\")."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "THE APPLICATION"
                }
                p {
                    "first Shakeshake (\"Licensed Application\") is a piece of software designed as a Web3 social link data platform that seeks out and uses cutting-edge solutions to enable the ease and security across international social linkages. It has been specially adapted for iOS and Android mobile devices ("Devices"). Our program promises to easily integrate a variety of application scenarios, starting with social and professional lives, with a simple click or shake of the phone's screen.
                    You cannot use the Licensed Application if your interactions would be subject to rules like the Federal Information Security Management Act (FISMA) or the
                       
                    Health Insurance Portability and Accountability Act (HIPAA), which are modified to comply with industry-specific regulations. Do not violate GLBA .
                    "
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "SCOPE OF LICENSE (SECTION 2)"
                }
                p {
                    "Licence granted by Shakeshake is non-transferable, non-exclusive and non- sublicensable. You may only install and use the Licensed Application on devices that You (End-User) own or control. If you sell your Devices to a third party, you must remove the Licensed Application from the Devices before doing so. Violations of the obligations mentioned above, as well as the attempt of infringement, may be subject to prosecution and damages. You may not reverse engineer, translate, disassemble, integrate, decompile, remove, modify, combine, create derivative works or updates of, adapt, or attempt to derive the source code of the Licensed Application, or any part thereof (except with Shakeshake 's prior written consent)."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "UPDATES AND SUPPORT"
                }
                p {
                    "All maintenance and support services for this Licensed Application are the sole responsibility of the Licensor. The email address for the Licensor is provided in the overview for this Licensed Application on the App Store or Play Store. Shakeshake and the End-User agree that the Services are under no duty to provide any upkeep or support services for the Licensed Application."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "USE OF DATA"
                }
                p {
                    "You agree that the licensor will have access to and be able to change the data you downloaded for the Licensed Application and your personal information. Your legal agreements with the licensor and the licensor's privacy policy, which may be viewed by clicking on settings and scrolling to privacy policy, govern how you may use such materials and information. The data and associated information from your device, system, and peripherals may be regularly collected and used by the licensor."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "USER-GENERATED CONTRIBUTION"
                }
                p {
                    "Your participation in chat rooms, message boards, online forums, and other features may be requested by Licensed Application. Any contributions you send could therefore be considered non-confidential and non-proprietary. Your Contributions do not violate any other party's proprietary rights throughout their creation, distribution, transmission, public display, or performance. The Licensed Application and this License Agreement may include and exploit your Contributions in any manner that is contemplated by your name, likeness, and/or other identifiable individual person. Your contributions aren't unacceptable in any
                    way, including being obscene, vulgar, lascivious, filthy, libelous, or slanderous (as determined by us). You do not mock, degrade, intimidate, or abuse anybody in Your Contributions."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "CONTRIBUTION LISCENSE"
                }
                p {
                    "You give us an unfettered, unlimited, irrevocable, non-exclusive, transferable, royalty-free, fully paid, worldwide right by submitting your Contributions to the Licensed Application. Any currently existing or future form, material, or technology will be covered under this license."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "LIABILITY"
                }
                p {
                    "The Licensor disclaims all liability and obligation for any harm brought on by a failure to uphold the terms of Section 2 of this License Agreement. To the extent permitted by any third-party terms and conditions of use, You must use the backup features of the Licensed Application to prevent data loss. You are aware that if the Licensed Application is changed or interfered with, You will not have access to the Licensed Application."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "WARRANTY"
                }
                p {
                    "No warranty is provided for the Licensed Application that is not executable on the device, that has been unauthorizedly modified, handled improperly or combined with other software or hardware. You are required to inspect the Licensed Application immediately after installing it and notify Shakeshake about issues discovered without delay.",
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "PRODUCT CLAIMS"
                }
                p {
                    "The End-User and Shakeshake agree that Shakeshake, not the Services, is in charge of resolving any claims the End-User or any third party may have regarding the Licensed Application or the End-possession User's and/or use of that Licensed Application, including but not limited to: I product liability claims; (ii) any claim that the Licensed Application does not meet any applicable legal or regulatory requirement; and (iii) claims arising under consumer protection."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "COMPLIANCE WITH LAW"
                }
                p {
                    "You affirm and guarantee that neither Your location nor Your status on any US government list of prohibited or restricted parties places You under an embargo imposed by the US Government or on its list of nations that \"sponsor terrorism.\""
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "CONTACT INFORMATION"
                }
                p {
                    "For general inquiries, complaints, questions or claims concerning the Licensed Application, please contact: Shakeshake 1 Washington Sq, San Jose, CA 95192, USA San Jose , CA 95192 United States zhaokai459@gmail.com"
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "TERMINATION"
                }
                p {
                    "The license is in effect until it is cancelled by either Shakeshake or you. If You violate any term(s) of this license, Your rights under this license will instantly and without notice from Shakeshake end. When your license expires, you must stop using the licensed application and delete all copies—complete or partial—that you have made."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "TERMS OF AGREEMENTS WITH THIRD PARTIES AND BENEFICIARY"
                }
                p {
                    "When using the Licensed Application, Shakeshake represents and warrants that Shakeshake will adhere to all applicable third-party terms of agreement. Apple and Google and their subsidiaries shall be third-party beneficiaries of this End User License Agreement in accordance with Section 9 of the \"Instructions for Minimum Terms of Developer's End-User License Agreement.\" Upon Your acceptance of the terms and conditions of this License Agreement, Apple and Google will have the right (and will be deemed to have accepted the right) to enforce this End User License Agreement against You."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "RIGHTS TO INTELLECTUAL PROPERTY"
                }
                p {
                    "In the event that a third party asserts that the Licensed Application or the End- possession User's and use of that Licensed Application violates the third party's intellectual property rights, Shakeshake and the End-User agree that Shakeshake will be solely responsible for the investigation, defense, settlement, and discharge of any such intellectual property infringement claims, independent of the Services."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "MISCELLANEOUS"
                }
                p {
                    "The validity of the remaining provisions of this agreement will not be impacted if any of its terms are found to be or become invalid. Invalid phrases will be swapped out for ones that are constructed to accomplish the main goal and are valid. Collateral agreements, revisions, and adjustments must be put in writing to be enforceable. It is only possible to waive the previous clause in writing. After entering their phone number for the verification code, the user would grant us access to their contacts on their phone."
                }
                p {
                    class: "font-bold text-xl sm:text-2xl",
                    "This License Agreement is governed by the laws of the State of California excluding its conflicts of law rules."
                }
            }
        }
    })
}