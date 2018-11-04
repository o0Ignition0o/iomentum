#![recursion_limit="128"]

extern crate strum;
extern crate strum_macros;
extern crate serde_derive;
extern crate stdweb;

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate yew;
pub mod banner;
pub mod whoiam;
pub mod skills;

use yew::prelude::App;

use banner::Banner;
use skills::Skills;
use whoiam::WhoIAm;
use stdweb::web::{IParentNode, document};
use stdweb::private::TODO;

#[derive(Debug, Fail)]
pub enum MountError {
    #[fail(display = "App mount: An error occured within stdweb query_selector")]
    QuerySelectorError,
    #[fail(display = "App mount: Element not found: {}", _0)]
    ElementNotFound(String)
}

impl From<TODO> for MountError {
    fn from(_: TODO) -> Self {
        MountError::QuerySelectorError
    }
}

macro_rules! mount {
    ($element:expr, $model:ty) => {{
        if let Some(element) = document().query_selector($element)? {
            App::<$model>::new().mount(element);
                Ok(())
        } else {
            Err(MountError::ElementNotFound(format!("Could not find the element {} in the DOM.", $element)))
        }
    }};
}

pub fn mount_app() -> Result<(), MountError> {
    mount!("#banner", Banner)?;
    mount!("#one", WhoIAm)?;
    mount!("#two", Skills)?;
    Ok(())
}
