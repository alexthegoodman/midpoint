use yew::prelude::*;

use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
    contexts::local::{LocalAction, LocalContextType},
};

struct NavOption {
    icon: &'static str,
    route: &'static str,
}

#[function_component]
pub fn ViewNav() -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");

    let nav_options = vec![
        NavOption {
            icon: "panorama",
            route: "/concepts",
        },
        NavOption {
            icon: "map-trifold",
            route: "/map",
        },
        NavOption {
            icon: "book-open",
            route: "/story",
        },
        NavOption {
            icon: "cube-focus",
            route: "/scene",
        },
        NavOption {
            icon: "faders",
            route: "/audio",
        },
        NavOption {
            icon: "speedometer",
            route: "/performance",
        },
        NavOption {
            icon: "gear",
            route: "/settings",
        },
    ];

    if local_context.current_project_id.is_none() {
        return html! {
            <nav class="view-nav">
                <MdButton
                    label=""
                    icon="sparkle"
                    on_click={Callback::from({
                        let local_context = local_context.clone();
                        move |_| {
                            local_context.dispatch(LocalAction::SetRoute("/".to_string()));
                        }
                    })}
                    disabled={false}
                    loading={false}
                    kind={MdButtonKind::MediumShadow}
                    variant={MdButtonVariant::Light}
                />
            </nav>
        };
    }

    html! {
        <nav class="view-nav">
            {nav_options.into_iter().map(|option| {
                let local_context = local_context.clone();
                html! {
                    <MdButton
                        label=""
                        icon={option.icon}
                        on_click={Callback::from(move |_| {
                            local_context.dispatch(LocalAction::SetRoute(option.route.to_string()));
                        })}
                        disabled={false}
                        loading={false}
                        kind={MdButtonKind::MediumShadow}
                        variant={MdButtonVariant::Light}
                    />
                }
            }).collect::<Html>()}
        </nav>
    }
}
