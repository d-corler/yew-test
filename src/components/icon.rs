use yew::{html, Component, Context, Html, Properties};

/// Icon library, set true to use sharp version.
#[derive(Clone, PartialEq, Debug)]
pub enum IconLibrary {
    Solid(bool),
    Regular(bool),
    Light(bool),
    Thin,
    Duotone,
    Brands,
}

impl Default for IconLibrary {
    fn default() -> Self {
        IconLibrary::Regular(false)
    }
}

/// Properties for the `Icon` component.
#[derive(Properties, PartialEq, Clone, Default)]
pub struct IconProps {
    /// Library of the icon. Defaults to `IconLibrary::default()`.
    #[prop_or(IconLibrary::default())]
    pub library: IconLibrary,
    /// Type of icon.
    pub icon: String,
}

pub struct Icon;

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let library = ctx.props().library.to_owned();

        let library_class = match library {
            IconLibrary::Solid(s) => format!("fa-solid{}", is_sharp_class(s)),
            IconLibrary::Regular(s) => format!("fa-regular{}", is_sharp_class(s)),
            IconLibrary::Light(s) => format!("fa-light{}", is_sharp_class(s)),
            IconLibrary::Thin => "fa-thin".to_string(),
            IconLibrary::Duotone => "fa-duotone".to_string(),
            IconLibrary::Brands => "fa-brands".to_string(),
        };

        html! {
            <i class={format!("{} fa-{}", library_class, ctx.props().icon)}></i>
        }
    }
}

fn is_sharp_class(sharp: bool) -> &'static str {
    match sharp {
        true => " fa-sharp",
        false => "",
    }
}
