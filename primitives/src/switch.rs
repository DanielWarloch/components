//! A two-state button.
//! 
//! #### Features
//! - Supports ARIA keyboard interactions.
//! - Can be controlled or uncontrolled.
//! 
//! ## Example
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_primitives::switch::{Switch, SwitchThumb};
//! 
//! #[component]
//! fn MyComponent() -> Element {
//!     rsx! {
//!         Switch {
//!             class: "switch",
//!             default_checked: false,
//!             on_checked_change: move |new_checked| println!("checked: {new_checked}"),
//!             
//!             SwitchThumb { class: "switch-thumb" }
//!         }
//!     }
//! }
//! ```
//! 
//! ## Props
//! 
//! | Prop                  | Description                                               | Default   |
//! | --------------------- | --------------------------------------------------------- | --------- |
//! | `checked`             | The controlled checked value.                             | `None`    |
//! | `default_checked`     | The default checked state.                                | `false`   |
//! | `disabled`            | Whether the switch is disabled.                           | `false`   |
//! | `required`            | Whether the switch is required in a form.                 | `false`   |
//! | `name`                | The form name of the switch.                              | `None`    |
//! | `on_checked_change`   | Callback for state changes. Required with `checked` prop. | `None`    |
//! 
//! ### Attributes
//! 
//! | Attribute         | States                    | 
//! | ----------------- | ------------------------- |
//! | `data-state`      | `checked` or `unchecked`  |
//! | `data-disabled`   | `true` or `false`         |
//! 
//! ## Accessibility
//! 
//! Follows the ARIA `switch` [role requirements](https://www.w3.org/WAI/ARIA/apg/patterns/switch/).
//! 
//! **Keyboard Interactions**
//! | Key   | Description           |
//! | ----- | --------------------- |
//! | Space | Toggle the switch.    |
use crate::use_controlled;
use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    checked: Option<Signal<bool>>,

    #[props(default = false)]
    default_checked: bool,

    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    required: ReadOnlySignal<bool>,

    #[props(default)]
    name: ReadOnlySignal<String>,

    #[props(default = ReadOnlySignal::new(Signal::new(String::from("on"))))]
    value: ReadOnlySignal<String>,

    #[props(default)]
    on_checked_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            value: props.value,
            aria_checked: checked,
            aria_required: props.required,
            disabled: props.disabled,
            "data-state": if checked() { "checked" } else { "unchecked" },
            // Only add data-disabled when actually disabled
            "data-disabled": if (props.disabled)() { "true" } else { "false" },

            onclick: move |_| {
                let new_checked = !checked();
                set_checked.call(new_checked);
            },

            // Switches should only toggle on Space, not Enter
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }

        // Hidden input for form submission
        input {
            r#type: "checkbox",
            "aria-hidden": true,
            tabindex: -1,
            name: props.name,
            value: props.value,
            checked,
            disabled: props.disabled,
            style: "transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0; width: 0; height: 0;",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SwitchThumbProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    rsx! {
        span { ..props.attributes }
    }
}
