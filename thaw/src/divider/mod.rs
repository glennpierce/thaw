use crate::utils::{class_list::class_list, mount_style, OptionalProp};
use leptos::*;

#[component]
pub fn Divider(#[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>) -> impl IntoView {
    mount_style("divider", include_str!("./divider.css"));

    view! {
        <div class=class_list!["thaw-divider", class.map(| c | move || c.get())]>
            <div class="thaw-divider__line"></div>
        </div>
    }
}
