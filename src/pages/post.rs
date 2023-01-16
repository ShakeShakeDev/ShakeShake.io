use dioxus::prelude::*;

pub fn TermsOfService (cx: Scope) -> Element {
    cx.render(rsx! {
        br {}
        div {
            class: "container mx-auto px-3 sm:px-40 dark:text-gray-100 sm:font-semibold text-base sm:text-xl",
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

pub fn PrivacyPolicy (cx: Scope) -> Element {
    cx.render(rsx! {
        br {}
        div {
            class: "container mx-auto px-3 sm:px-40 dark:text-gray-100 sm:font-semibold text-base sm:text-xl",
            div {
                class: "flex flex-col space-y-6",
                h1 {
                    class: "underline font-bold text-5xl",
                    "Privacy Notice"
                }
                span {
                    class: "text-gray-600 dark:text-gray-300",
                    "Last updated January 6th, 2023"
                }
                p {
                    "When you use our services, such as when you: Download and use our mobile application (ShakeShake) or any other application of ours that links to this privacy notice, we may collect, store, utilize, and/or share your information.
                    Participate in any additional related activities with us, such as any sales, marketing, or events.
                    Any queries or worries? You can better understand your privacy rights and options by carefully reading this privacy notice. Please refrain from using our Services if you disagree with our policies and procedures. Please email us at zhaokai459@gmail.com if you still have any queries or issues."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "KEY POINTS SYNOPSIS"
                }
                p {
                    "What kinds of personal data do we handle? Depending on how you engage with ShakeShake and the Services, the decisions you make, and the products and features you use, we may process personal information when you visit, utilize, or explore our Services."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Do we handle any sensitive personal data?"
                }
                p {
                    "When necessary, with your permission or in another manner allowed by relevant law, we may treat sensitive personal information."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Do we obtain any data from outside sources?"
                }
                p {
                    "We might get data from outside sources like social media networks, public databases, and marketing partners."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "What steps do we take to process your data?"
                }
                p {
                    "We process your information for the purposes of delivering, enhancing, and managing our Services, keeping in touch with you, security and fraud prevention, and legal compliance. With your permission, we may use use your information for other purposes. We only process your information when we have a good reason to do so under the law."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "What circumstances and people do we share personal information with?"
                }
                p {
                    "In some circumstances and with particular third parties, we might disclose information."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "How do we safeguard your personal information?"
                }
                p {
                    "To protect your personal information, we have organizational and technical systems and procedures in place. We cannot promise or guarantee that hackers, cybercriminals, or other unauthorized third parties won't be able to circumvent our security and improperly collect, access, steal, or modify your information because no electronic transmission over the internet or information storage technology can be guaranteed to be 100% secure."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "What rights do you have?"
                }
                p {
                    "Depending on where you are based legally, the privacy law that is in effect may indicate that you have specific rights with regard to your personal data."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "What rights do you exercise?"
                }
                p {
                    "The simplest approach to exercise your rights is to contact us or complete our data subject request form, which is available here. Any request will be taken into account, and we'll follow any applicable data protection rules in doing so."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "CONTENTS"
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "1. WHAT DATA ARE WE COLLECTING?"
                }
                p {
                    "Information about yourself that you give us
                    Briefly stated: We gather the personal data you give us.
                    When you register on the Services, express an interest in learning more about us or our products and Services, take part in activities on the Services, or contact us in any other way, you voluntarily give us personal information.
                    Personal Data You Have Provided. Your interactions with us and the Services, your decisions, and the products and features you use all have an impact on the personal information we collect.
                    The following is a list of the types of personal data we might gather:
                    Names, contact information, such as phone numbers and emails, work titles, usernames and passwords, and contact preferences, as well as contact or authentication data.
                    "
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Sensitive Information"
                }
                p {
                    "We may process any of the following categories of sensitive information as required, with your permission, or in another manner permitted by applicable law:
                    Student data"
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Application Data"
                }
                p {
                    "If you choose to grant us access or permission while using our application(s), we may also gather the following details:"
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Mobile device accessibility"
                }
                p {
                    "We might ask for authorization or access to
                    key mobile device features, such as your mobile
                    Bluetooth, contacts, SMS, social network accounts, and other data on the device features including a camera, storage, and others. If you would want to modify our
                    You can modify access or permissions in your device's settings."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Data from mobile devices"
                }
                p {
                    "We gather device data automatically.
                    (such as the manufacturer, model, and ID of your mobile device), working information about the system's version and setup, identifying codes for devices and applications, browser type, and Internet service provider, hardware model, and/or mobile Internet Protocol address, carrier, and/or proxy server. If you use one of our applications, we may additionally gather information about the mobile device type you are using, the operating system or platform it is running, the phone network it is connected to, the features of our application(s) that you have used, and other details."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Geolocation Details."
                }
                p {
                    "In order to provide certain location-based services, we might ask for access to or authorization to track location-based data from your mobile device continuously or while you use one of our mobile applications. If you wish to adjust our access or permissions, you may do so in your device's settings."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Push Notifications."
                }
                p {
                    "We could ask to send you push alerts about your account or certain application features (s). You can disable these communications in your device's settings if you'd prefer not to receive them."
                }
                p {
                    "In order to maintain the security and functionality of our application(s), as well as for troubleshooting, internal analytics, and reporting needs, this information is largely required."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "information gleaned automatically"
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Briefly stated: "
                }
                p {
                    "When you visit our Services, some information, including your Internet Protocol (IP) address and/or browser and device details, is automatically gathered."
                }
                p {
                    "When you access, utilize, or navigate the Services, we automatically gather some information. Your IP address, browser and device characteristics, operating system, preferred language, referring URLs, device name, country, location, information about how and when you use our Services, and other technical information may be included in this information even though it does not specifically identify you (like your name or contact information). This data is largely required for internal analytics and reporting, as well as to ensure the security and functionality of our Services. Like many companies, we also use cookies and other similar technologies to gather information."
                }
                p {
                    "Service-related, diagnostic, usage, and performance data are contained in log and usage data. Device data consists of specifics about the phone, tablet, computer, or other device you use to access the Services. We also get location information about the whereabouts of your device."
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Data gathered from additional sources"
                }
                h3 {
                    class: "underline font-semibold text-2xl",
                    "Briefly stated:"
                }
                p {
                    "We may obtain a limited amount of information from marketing partners, public databases, and other outside sources. We may receive information about you from additional sources, such as public databases, joint marketing partners, affiliate programs, data suppliers, and from other third parties, in order to improve our capacity to provide you with pertinent marketing, offers, and services and to update our records. For the purpose of targeted advertising and event promotion, this data may comprise mailing addresses, job titles, email addresses, phone numbers, intent data (or user activity data), Internet Protocol (IP) addresses, social media profiles, social media URLs, and custom profiles."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "HOW ARE YOUR DATA PROCESSED BY US?"
                }
                p {
                    "In a nutshell, we use your information to interact with you, to deliver, enhance, and manage our Services, to avoid fraud and security threats, and to comply with the law. With your permission, we may use use your information for other purposes. Depending on how you use our Services, we process your personal information for a number of purposes, including: To make account creation and authentication easier and manage user accounts in other ways. We may process your information to enable you to create and access your account and maintain the functionality of your account. To safeguard our services. Your information may be processed as part of our efforts to maintain the security and safety of our Services, including fraud detection and prevention."
                }
                p {
                    "We handle your personal data for a number of functions, including account setup, authentication, and general account management. In the instances mentioned in this section and/or with the following third parties, we might disclose information. Any information you share with third parties with whom we may link or who advertise on our Services is not subject to our control. We might access or retain information through cookies and other tracking technologies. Your personal information will only be kept by us for as long as it is required to fulfill the objectives outlined in this privacy notice. We will only keep your information as long as you have an account for the purposes described in this notice.
                    We never intentionally request information from minors under the age of 18 or market to them. If we become aware that a user under the age of 18 has provided us with personal information, we will immediately deactivate the account and take
                    reasonable steps to ensure that the information is deleted. You warrant that you are at least 18 years old or that you are the parent or legal guardian of a child by using the Services."
                }
                p {
                    "You have the right to file a complaint with your local data protection supervisory authority if you reside in the EEA or the UK and feel we are processing your personal information unlawfully. You can use the contact information to revoke your consent at any time."
                }
                p {
                    "You can turn on the Do-Not-Track ("DNT") function or setting in the majority of web browsers, some mobile operating systems, and mobile applications to indicate that you prefer not to have information about your online browsing activity tracked and collected. There is currently no established common technical standard for identifying and utilizing DNT signals. As a result, we currently do not react to DNT browser signals or any other system that automatically communicates your preference not to be tracked online. If a standard for online tracking is adopted that we must follow in the future, we will inform you about that practice in a revised version of this privacy notice."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "ARE THERE SPECIFIC PRIVACY RIGHTS FOR CALIFORNIA RESIDENTS?"
                }
                p {
                    "In a nutshell: If you live in California, you do have certain access rights to your personal information.
                    The \"Shine The Light\" law, also known as California Civil Code Section 1798.83, entitles our users who reside in California to request and obtain from us, once a year and without cost, details about the categories of personal information (if any) we disclosed to third parties for direct marketing purposes and the names and addresses of all third parties with whom we shared personal information in the previous calendar year. If you live in California and would like to make such a request, kindly send your request to us in writing at the address listed below. You have the right to ask for the removal of undesired information that you publicly post on the Services if you are under 18 years old, live in California, and have a registered account with the Services. Please contact us using the details listed below, specify the email address linked to your account, and state that you are a resident of California in order to seek the removal of such data. Although we'll take steps to prevent the data from being publicly exposed on the Services, please be advised that it might not be entirely or completely erased from all our services."
                }
                p {
                    "The California Code of Regulations defines a \"resident\" as: (1) every individual who is in the State of California for other than a temporary or transitory purpose and (2) every individual who is domiciled in the State of California who is outside the State of California for a temporary or transitory purpose All other individuals are defined as \"non-residents.\" If this definition of \"resident\" applies to you, we must adhere to certain rights and obligations regarding your personal information."
                }
                p {
                    "In the last 12 months, we have gathered the following categories of personal data: names, addresses, geolocation information. When you interact with us in person, online, or over the phone, we may also gather more personal data about you. Inferences drawn from these categories can be used to create a profile or summary about you. ShakeShake will honor your request and erase your personal information if you ask us to do so. In response to a consumer's request, we are not required to release or remove de-identified consumer information.
                    We won't treat you differently if you exercise your right to privacy. For security and fraud-prevention measures, we could ask you for more information if we can't confirm your identity using the data we already have on file. You have the right to object to the processing of your personal data. You can ask that your personal information not be sold to third parties in the future. We will respond to an opt-out request as quickly as we can, but no later than 15 days after the request was submitted."
                }
                h2 {
                    class: "underline font-bold text-3xl",
                    "HOW CAN YOU CONTACT US ABOUT THIS NOTICE?"
                }
                p {
                    "If you have questions or comments about this notice, you may email us at zhaokai459@gmail.com or by post to: ShakeShake 1 Washington Sq, San Jose, CA 95192, USA San Jose , CA 95192 United States"
                }
            }
        }
    })
}