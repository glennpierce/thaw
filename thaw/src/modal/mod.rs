use crate::{
    components::{CSSTransition, OptionComp, Teleport},
    utils::{mount_style, use_click_position, Model},
    Card, CardFooter, CardHeader, CardHeaderExtra, Icon,
};
use leptos::*;

#[slot]
pub struct ModalFooter {
    children: ChildrenFn,
}

#[component]
pub fn Modal(
    #[prop(into)] show: Model<bool>,
    #[prop(default = true.into(), into)] mask_closeable: MaybeSignal<bool>,
    #[prop(default = 2000.into(), into)] z_index: MaybeSignal<i16>,
    #[prop(default = MaybeSignal::Static("600px".to_string()), into)] width: MaybeSignal<String>,
    #[prop(optional, into)] title: MaybeSignal<String>,
    children: Children,
    #[prop(optional)] modal_footer: Option<ModalFooter>,
) -> impl IntoView {
    mount_style("modal", include_str!("./modal.css"));

    let on_mask_click = move |_| {
        if mask_closeable.get_untracked() {
            show.set(false);
        }
    };

    let mask_ref = NodeRef::<html::Div>::new();
    let scroll_ref = NodeRef::<html::Div>::new();
    let modal_ref = NodeRef::<html::Div>::new();

    let click_position = use_click_position();
    let on_enter = move |_| {
        let Some(position) = click_position.get_untracked() else {
            return;
        };

        let Some(scroll_el) = scroll_ref.get_untracked() else {
            return;
        };
        let scroll_top = scroll_el.scroll_top();

        let Some(modal_el) = modal_ref.get_untracked() else {
            return;
        };

        let x = -(modal_el.offset_left() - position.0);
        let y = -(modal_el.offset_top() - position.1 - scroll_top);

        let _ = modal_el.attr("style", format!("transform-origin: {}px {}px", x, y));
    };

    view! {
        <Teleport>
            <div
                class="thaw-modal-container"
                style:z-index=move || z_index.get()
                style=("--thaw-width", move || width.get())
            >
                <CSSTransition
                    node_ref=mask_ref
                    show=show.signal()
                    name="fade-in-transition"
                    let:display
                >
                    <div
                        class="thaw-modal-mask"
                        style=move || display.get()
                        on:click=on_mask_click
                        ref=mask_ref
                    ></div>
                </CSSTransition>
                <CSSTransition
                    node_ref=scroll_ref
                    show=show.signal()
                    name="fade-in-scale-up-transition"
                    on_enter
                    let:display
                >
                    <div
                        class="thaw-modal-scroll"
                        style=move || display.get()
                        ref=scroll_ref
                    >
                        <div
                            class="thaw-modal-body"
                            ref=modal_ref
                            role="dialog"
                            aria-modal="true"
                        >
                            <Card>
                                <CardHeader slot>
                                    <span class="thaw-model-title">{move || title.get()}</span>
                                </CardHeader>
                                <CardHeaderExtra slot>
                                    <span style="cursor: pointer;" on:click=move |_| show.set(false)>
                                        <Icon icon=icondata_ai::AiCloseOutlined/>
                                    </span>
                                </CardHeaderExtra>
                                {children()}
                                <CardFooter slot if_=modal_footer.is_some()>
                                    <OptionComp value=modal_footer.as_ref() let:footer>
                                        {(footer.children)()}
                                    </OptionComp>
                                </CardFooter>
                            </Card>
                        </div>
                    </div>
                </CSSTransition>
            </div>
        </Teleport>
    }
}
