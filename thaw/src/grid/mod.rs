mod grid_item;

pub use grid_item::*;
use leptos::{context::Provider, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn Grid(
    #[prop(default = MaybeSignal::Static(1u16), into)] cols: MaybeSignal<u16>,
    #[prop(optional, into)] x_gap: MaybeSignal<u16>,
    #[prop(optional, into)] y_gap: MaybeSignal<u16>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let style = Memo::new(move |_| {
        let mut style = String::from("display: grid;");
        style.push_str(&format!(
            "grid-template-columns: repeat({}, minmax(0px, 1fr));",
            cols.get()
        ));
        style.push_str(&format!("grid-gap: {}px {}px;", y_gap.get(), x_gap.get()));
        style
    });

    view! {
        <Provider value=GridInjection::new(x_gap)>
            <div
                class=class_list!["thaw-grid", class]
                style=move || style.get()
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct GridInjection {
    x_gap: MaybeSignal<u16>,
}

impl GridInjection {
    pub fn new(x_gap: MaybeSignal<u16>) -> Self {
        Self { x_gap }
    }
}

pub(crate) fn use_grid() -> GridInjection {
    expect_context()
}
