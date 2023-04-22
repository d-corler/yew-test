use yew::{html, Component, Context, Html};

use crate::components::avatar::Avatar;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <header class="bg-white h-24 border-b border-outline">
                <nav class="mx-auto flex items-center justify-between p-5 h-full" aria-label="Global">
                    <div class="flex items-center lg:flex-1 h-full">
                        <a href="#">
                            <span class="sr-only">{"Your Company"}</span>
                            <img class="max-h-12 w-auto" src="images/logo.png" alt="Your company logo" />
                        </a>

                        <div class="w-px h-full bg-outline ml-6" />

                        <div class="text-2xl font-semibold flex items-center ml-6">{"Dashboard"}</div>
                    </div>

                    <div class="hidden lg:flex lg:flex-1 lg:justify-end">
                        <Avatar
                            src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                            alt="avatar"
                        />
                        /*<Button label="Sign In" color={ButtonColor::Secondary} />
                        <Button
                            icon_l={IconProps {icon: "home".into(), ..Default::default()}}
                        />
                        // <Button />
                        <a href="#" class="text-sm font-semibold text-gray-900">{"Sign in"}</a>*/
                    </div>
                </nav>
            </header>
        }
    }
}
