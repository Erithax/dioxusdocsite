use dioxus::prelude::*;

/// # A Simple Component
///
/// Dioxus components are declaratively defined using either the `rsx!` macro or the `html!` macro. Both these macros are
/// just helpful wrappers around the `NodeFactory` API - which can be used directly to create new elements, listeners,
/// attributes, and components.
///
/// The `rsx!` macro is designed to feel like writing nested structs with optional values, taking advantage of deep
/// integration with Rust-Analyzer. The `html!` macro is designed to feel like writing HTML - it's possible to drop in
/// HTML templates without any additional work.
pub static Simple: FC<()> = |cx| {
    cx.render(rsx! {
        div { "Hello world!"}
    })
};

/// # A Stateful Component
///
/// Dioxus components use hooks to store state between renders. The `use_state` hooks make it easy to update state from
/// event listeners attached to elements in the component. Whenever the state is modified, the component will be re-rendered.
///
/// Thanks to Rust's ownership rules, it's impossible to misuse the `use_state` hook.
pub static Stateful: FC<()> = |cx| {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        button { "Upvote counter: {count}", onclick: move |_| count += 1 }
    })
};

/// # Advanced Rendering
///
/// Dioxus accepts fragments, iterators, conditionals, listeners, matching, f_string iterpolation and more. Anything that can
/// be coerced into an iterator of VNodes can be used in the macro bodies. By default, `rsx!` is Lazy, meaning it won't allocate
/// until "rendered" with a `render` call.
///
/// Elements are created with a dedicated memory allocator that intelligently reuses memory between renders. A component
/// at "steady-state" performs zero global allocations, making rendering extremely fast and memory efficient.
pub static AdvancedRendering: FC<()> = |cx| {
    let should_show = use_state(cx, || true);

    let button_text = match *should_show {
        true => "Click to show",
        false => "Click to hide",
    };

    let fizzes = (0..10).map(|i| match (i % 3, i % 5) {
        (0, 0) => rsx!(in cx, li {"FizzBuzz"} ),
        (0, _) => rsx!(in cx, li {"Fizz"} ),
        (_, 0) => rsx!(in cx, li {"Buzz"} ),
        (_, _) => rsx!(in cx, li {"{i}"} ),
    });

    cx.render(rsx! {
        "Advanced rendering"
        button { "{button_text}", onclick: move |_| should_show.set(!should_show)}
        {should_show.then(|| rsx!( ul { {fizzes} } ))}
    })
};

/// # Built-in error handling
///
/// Because components return an `Option<VNode>`, errors can be handled gracefully through the use of the question mark
/// syntax. Components that fail to render will return will be frozen until the next successful render.
///
/// This is exceptionally useful for components that select optional values that will never be `None` while the component
/// is being viewed - IE a settings panel that can only be shown if a user is logged in.
pub static ErrorHandling: FC<()> = |cx| {
    let items = vec!["a", "b", "c", "d", "e"];
    let first_item = items.first()?;
    
    rsx!(in cx, h1 { "First item: {first_item}" })
};

/// # Global state
///
/// With Dioxus, it's possible to directly expose shared state to child components with the `use_provide_context` hook.
/// Components lower in the tree can then directly read and write to the shared state with runtime safety.
///
/// Dioxus also has 1st-class support for Diplex: a global state management toolkit modeled after RecoilJS.
pub static GlobalState: FC<()> = |cx| {
    struct SharedState(&'static str);

    cx.use_provide_context(|| SharedState("world!"));

    static Child: FC<()> = |cx| {
        let name = cx.use_context::<SharedState>().0;
        rsx!(in cx, "{name}")
    };

    cx.render(rsx! {
        div { "Hello, ", Child {} }
    })
};

/// # Coroutines and tasks
///
/// Components may spawn a coroutine or task to perform asynchronous operations. These tasks may be started, stopped, or
/// reset by other logc in the component. Coroutines are extremely handy for asynchronous tasks like network requests,
/// websockets, and multi-threading.
pub static Tasks: FC<()> = |cx| {
    let count = use_state(cx, || 0);
    let count_async = count.for_async();

    cx.use_task(|| async move {
        loop {
            *count_async.get_mut() += 1;
        }
    });

    cx.render(rsx! {
        pre {"Count: {count}"}
    })
};

/// # Suspense
///
/// Dioxus supports Suspense - a way of deferring rendering until a condition is met. Simply pass in a future and a callback,
/// and Dioxus will wait for the future to resolve before rendering the result. Suspense makes it possible to prevent
/// cascaded re-rendering and allows Dioxus to render the rest of the component while waiting for the future to complete.
pub static Suspense: FC<()> = |cx| {
    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }
    const ENDPOINT: &str = "https://dog.ceo/api/breeds/image/random";

    let doggo = cx.use_suspense(
        || surf::get(ENDPOINT).recv_json::<DogApi>(),
        |cx, res| match res {
            Ok(res) => rsx!(in cx, img { src: "{res.message}" }),
            Err(_) => rsx!(in cx, div { "No doggos for you :(" }),
        },
    );

    cx.render(rsx! {
        h1 {"Waiting for doggos:"}
        {doggo}
    })
};
