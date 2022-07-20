use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

mod app_callbacks;
mod default_handlers_configuration;
mod main_loop_callbacks;
mod service_db;

use app_callbacks::AppCallbacks;
use default_handlers_configuration::DefaultHandlersConfiguration;
use main_loop_callbacks::ArchMainLoopCallbacks;
use service_db::{DatabaseDefinition, DatabaseEntry};

#[proc_macro]
#[proc_macro_error]
pub fn service_database(input: TokenStream) -> TokenStream {
    let DatabaseDefinition { entries } = parse_macro_input!(input as DatabaseDefinition);

    let entry_count: usize = entries.iter().map(|entry| entry.entry_count()).sum();
    let entry_count_u8: u8 = entry_count as u8;

    let services: Vec<proc_macro2::TokenStream> = entries
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| {
            if let DatabaseEntry::Service { uuid: _ } = entry {
                let idx = idx as u8;
                Some(quote!(
                    #idx
                ))
            } else {
                None
            }
        })
        .collect();
    let services_size = services.len();
    let entries = entries.iter().map(|entry| entry.generate());

    let out = quote!(
        #[export_name = "custs1_att_db"]
        pub(crate) static CUSTS1_ATT_DB: [da14531_sdk::ble_stack::host::att::attm::AttmDesc128; #entry_count] = [
            #(#entries),*
        ];

        pub(crate) const CUSTS1_ATT_DB_LEN: u8 = #entry_count_u8;

        #[export_name = "custs1_services"]
        static CUSTS1_SERVICES: [u8; #services_size + 1] = [#(#services),* , CUSTS1_ATT_DB_LEN];

        #[export_name = "custs1_services_size"]
        static CUSTS1_SERVICES_SIZE: u32 = #services_size as u32;


        #[export_name = "rom_cust_prf_cfg"]
        static ROM_CUST_PRF_CFG: da14531_sdk::ble_stack::profiles::custom::custs::RomCustPrfCfg = da14531_sdk::ble_stack::profiles::custom::custs::RomCustPrfCfg {
            custs1_services: CUSTS1_SERVICES.as_ptr(),
            custs1_services_size: &(#services_size as u8),
            custs1_att_db: CUSTS1_ATT_DB.as_ptr() as *mut _,
            custs_get_func_callbacks: Some(da14531_sdk::app_modules::app_common::app::custs_get_func_callbacks),
        };
    );

    out.into()
}

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
