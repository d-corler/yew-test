use yew::{html, Component, Context, Html, Properties};

use crate::components::icon::{Icon, IconProps};

#[derive(Clone, PartialEq, Default)]
pub enum ButtonColor {
    #[default]
    Primary,
    Secondary,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// Color of the button. Defaults to `ButtonColor::default()`.
    #[prop_or(ButtonColor::default())]
    pub color: ButtonColor,
    #[prop_or(None)]
    pub label: Option<String>,
    /// Icon left
    #[prop_or(None)]
    pub icon_l: Option<IconProps>,
    /// Icon right
    #[prop_or(None)]
    pub icon_r: Option<IconProps>,
}

pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let color = ctx.props().color.to_owned();
        let label = ctx.props().label.to_owned();
        let icon_l = ctx.props().icon_l.to_owned();
        let icon_r = ctx.props().icon_r.to_owned();

        let button_class = match color {
            ButtonColor::Primary => "button--primary",
            ButtonColor::Secondary => "button--secondary",
        };

        html! {
            <button class={button_class}>
                { icon_l.map(|icon| html! { <Icon ..icon /> }).unwrap_or_default() }
                { label.map(|l| html! { <span>{ l }</span> }).unwrap_or_default() }
                { icon_r.map(|icon| html! { <Icon ..icon /> }).unwrap_or_default() }
            </button>
        }
    }
}
