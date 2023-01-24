use leptos_dom::{Errors, Fragment, IntoView};
use leptos_macro::component;
use leptos_reactive::{create_rw_signal, provide_context, RwSignal, Scope};

#[component(transparent)]
pub fn ErrorBoundary<F, IV>(
    cx: Scope,
    /// The components inside the tag which will get rendered
    children: Box<dyn FnOnce(Scope) -> Fragment>,
    /// A fallback that will be shown if an error occurs.
    fallback: F,
) -> impl IntoView
where
    F: Fn(Scope, Option<RwSignal<Errors>>) -> IV + 'static,
    IV: IntoView,
{
    let errors: RwSignal<Errors> = create_rw_signal(cx, Errors::default());

    provide_context(cx, errors);

    // Run children so that they render and execute resources
    let children = children(cx);

    move || match errors.get().0.is_empty() {
        true => children.clone().into_view(cx),
        false => fallback(cx, Some(errors)).into_view(cx),
    }
}
