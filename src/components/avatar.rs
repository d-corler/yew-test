use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Default)]
pub enum AvatarSize {
    #[default]
    Normal,
}

/// Properties for the `Avatar` component.
#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    /// Source of avatar.
    pub src: String,
    /// Alt attribute.
    #[prop_or(None)]
    pub alt: Option<String>,
    /// Size of the avatar. Defaults to `AvatarSize::default()`.
    #[prop_or(AvatarSize::default())]
    pub size: AvatarSize,
}

pub struct Avatar;

impl Component for Avatar {
    type Message = ();
    type Properties = AvatarProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let src = ctx.props().src.to_owned();
        let alt = ctx.props().alt.to_owned();
        let size = ctx.props().size.to_owned();

        let size_class = match size {
            AvatarSize::Normal => "w-14 h-14",
        };

        html! {
            <img class={format!("avatar {size_class}")} src={src} alt={alt} />
        }
    }
}
