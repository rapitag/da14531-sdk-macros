#![feature(iterator_try_collect)]

mod app_callbacks;
mod app_custs;
mod app_diss;
mod default_handlers_configuration;
mod main_loop_callbacks;

use app_custs::CustomServer1ServiceConfiguration;
use app_diss::DeviceInformationServiceConfiguration;
use proc_macro::TokenStream;
use syn::parse_macro_input;

use app_callbacks::AppCallbacks;
use default_handlers_configuration::DefaultHandlersConfiguration;
use main_loop_callbacks::ArchMainLoopCallbacks;

#[proc_macro]
pub fn register_main_loop_callbacks(input: TokenStream) -> TokenStream {
    let callbacks = parse_macro_input!(input as ArchMainLoopCallbacks);

    callbacks.generate().into()
}

#[proc_macro]
pub fn register_app_callbacks(input: TokenStream) -> TokenStream {
    let callbacks = parse_macro_input!(input as AppCallbacks);
    let x = callbacks.generate();

    x.into()
}

#[proc_macro]
pub fn default_handlers_configuration(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as DefaultHandlersConfiguration);

    config.generate().into()
}

#[proc_macro]
pub fn configure_device_information_service(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as DeviceInformationServiceConfiguration);

    config.generate().into()
}

#[proc_macro]
pub fn configure_custom_server1_service(input: TokenStream) -> TokenStream {
    let mut config = parse_macro_input!(input as CustomServer1ServiceConfiguration);

    match config.generate() {
        Ok(code) => code.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
