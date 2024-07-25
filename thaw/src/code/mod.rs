use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Code(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] inner_html: Option<String>,
) -> impl IntoView {
    mount_style("code", include_str!("./code.css"));
    view! {
        <code class=class_list!["thaw-code", class]>

            {if let Some(inner_html) = inner_html {
                view! { <pre inner_html=inner_html></pre> }.into_any().into()
            } else if let Some(text) = text {
                view! { <pre>{text}</pre> }.into_any().into()
            } else {
                None
            }}

        </code>
    }
}
