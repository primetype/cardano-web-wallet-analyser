use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)]
pub fn initialize_theme() {
    use wasm_bindgen::JsCast;
    use web_sys::{window, Event, MediaQueryList};

    let window = window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let root_element = document
        .document_element()
        .expect("document should have a root element");

    if let Some(match_media) = window
        .match_media("(prefers-color-scheme:light)")
        .ok()
        .flatten()
    {
        // Set initial theme based on user preference
        if match_media.matches() {
            root_element
                .set_attribute("data-bs-theme", "light")
                .expect("could not set attribute");
        } else {
            root_element
                .set_attribute("data-bs-theme", "dark")
                .expect("could not set attribute");
        }

        // Set up event listener for theme changes
        let root_clone = root_element.clone();
        let callback = Closure::wrap(Box::new(move |event: Event| {
            let media_query_list = event
                .target()
                .expect("event should have a target")
                .dyn_into::<MediaQueryList>()
                .expect("event target should be a MediaQueryList");

            let theme = if media_query_list.matches() {
                "light"
            } else {
                "dark"
            };
            root_clone
                .set_attribute("data-bs-theme", theme)
                .expect("could not set attribute");
        }) as Box<dyn FnMut(_)>);

        match_media
            .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
            .expect("could not add event listener");

        // Leak the closure to keep it alive for the lifetime of the program
        callback.forget();
    }
}
