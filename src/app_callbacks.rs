use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Expr, FieldValue, Member, Path, Token};

#[derive(Debug)]
pub struct AppCallbacks {
    app_on_connection: Option<Path>,
    app_on_disconnect: Option<Path>,
    app_on_connect_failed: Option<Path>,
    app_on_update_params_rejected: Option<Path>,
    app_on_update_params_complete: Option<Path>,
    app_on_set_dev_config_complete: Option<Path>,
    app_on_adv_nonconn_complete: Option<Path>,
    app_on_adv_undirect_complete: Option<Path>,
    app_on_adv_direct_complete: Option<Path>,
    app_on_db_init_complete: Option<Path>,
    app_on_scanning_completed: Option<Path>,
    app_on_adv_report_ind: Option<Path>,
    app_on_get_dev_name: Option<Path>,
    app_on_get_dev_appearance: Option<Path>,
    app_on_get_dev_slv_pref_params: Option<Path>,
    app_on_set_dev_info: Option<Path>,
    app_on_data_length_change: Option<Path>,
    app_on_update_params_request: Option<Path>,
    app_on_generate_static_random_addr: Option<Path>,
    app_on_svc_changed_cfg_ind: Option<Path>,
    app_on_get_peer_features: Option<Path>,
}

impl Parse for AppCallbacks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> =
            input.parse_terminated(FieldValue::parse)?;

        let mut callbacks = Self {
            app_on_connection: None,
            app_on_disconnect: None,
            app_on_connect_failed: None,
            app_on_update_params_rejected: None,
            app_on_update_params_complete: None,
            app_on_set_dev_config_complete: None,
            app_on_adv_nonconn_complete: None,
            app_on_adv_undirect_complete: None,
            app_on_adv_direct_complete: None,
            app_on_db_init_complete: None,
            app_on_scanning_completed: None,
            app_on_adv_report_ind: None,
            app_on_get_dev_name: None,
            app_on_get_dev_appearance: None,
            app_on_get_dev_slv_pref_params: None,
            app_on_set_dev_info: None,
            app_on_data_length_change: None,
            app_on_update_params_request: None,
            app_on_generate_static_random_addr: None,
            app_on_svc_changed_cfg_ind: None,
            app_on_get_peer_features: None,
        };

        for field in fields {
            let key = match field.member {
                Member::Named(name) => name,
                Member::Unnamed(unnamed) => {
                    panic!("Unexpected unnamed field: {:?}", unnamed);
                }
            };
            let value = field.expr;
            match key.to_string().as_str() {
                "app_on_connection" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_connection = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_on_connection: {:?}", value);
                    }
                },
                "app_on_disconnect" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_disconnect = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_on_disconnect: {:?}", value);
                    }
                },
                "app_on_connect_failed" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_connect_failed = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_connect_failed: {:?}",
                            value
                        );
                    }
                },
                "app_on_update_params_rejected" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_update_params_rejected = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_update_params_rejected: {:?}",
                            value
                        );
                    }
                },
                "app_on_update_params_complete" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_update_params_complete = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_update_params_complete: {:?}",
                            value
                        );
                    }
                },
                "app_on_set_dev_config_complete" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_set_dev_config_complete = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_set_dev_config_complete: {:?}",
                            value
                        );
                    }
                },
                "app_on_adv_nonconn_complete" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_adv_nonconn_complete = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_adv_nonconn_complete: {:?}",
                            value
                        );
                    }
                },
                "app_on_adv_undirect_complete" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_adv_undirect_complete = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_adv_undirect_complete: {:?}",
                            value
                        );
                    }
                },
                "app_on_adv_direct_complete" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_adv_direct_complete = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_adv_direct_complete: {:?}",
                            value
                        );
                    }
                },
                "app_on_db_init_complete" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_db_init_complete = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_db_init_complete: {:?}",
                            value
                        );
                    }
                },
                "app_on_scanning_completed" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_scanning_completed = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_scanning_completed: {:?}",
                            value
                        );
                    }
                },
                "app_on_adv_report_ind" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_adv_report_ind = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_adv_report_ind: {:?}",
                            value
                        );
                    }
                },
                "app_on_get_dev_name" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_get_dev_name = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_get_dev_name: {:?}",
                            value
                        );
                    }
                },
                "app_on_get_dev_appearance" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_get_dev_appearance = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_get_dev_appearance: {:?}",
                            value
                        );
                    }
                },
                "app_on_get_dev_slv_pref_params" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_get_dev_slv_pref_params = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_get_dev_slv_pref_params: {:?}",
                            value
                        );
                    }
                },
                "app_on_set_dev_info" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_set_dev_info = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_set_dev_info: {:?}",
                            value
                        );
                    }
                },
                "app_on_data_length_change" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_data_length_change = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_data_length_change: {:?}",
                            value
                        );
                    }
                },
                "app_on_update_params_request" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_update_params_request = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_update_params_request: {:?}",
                            value
                        );
                    }
                },
                "app_on_generate_static_random_addr" => {
                    match value {
                        Expr::Path(path) => {
                            callbacks.app_on_generate_static_random_addr = Some(path.path);
                        }
                        _ => {
                            panic!("Unextpected expression for app_on_generate_static_random_addr: {:?}", value);
                        }
                    }
                }
                "app_on_svc_changed_cfg_ind" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_svc_changed_cfg_ind = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_svc_changed_cfg_ind: {:?}",
                            value
                        );
                    }
                },
                "app_on_get_peer_features" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_get_peer_features = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_get_peer_features: {:?}",
                            value
                        );
                    }
                },
                _ => {
                    panic!("Unexpected field: {} = {:?}", key.to_string(), value);
                }
            }
        }

        Ok(callbacks)
    }
}

impl AppCallbacks {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let mut callback_wrappers = Vec::new();
        let mut struct_fields = Vec::new();

        if let Some(app_on_connection) = &self.app_on_connection {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_connection(conidx: u8, param: *const da14531_sdk::ble_stack::host::gap::gapc::task::GapcConnectionReqInd) {
                    #app_on_connection(conidx, unsafe{&*param});
                }
            ));
            struct_fields.push(quote!(app_on_connection: Some(__app_on_connection)));
        } else {
            struct_fields.push(quote!(
                app_on_connection: Some(da14531_sdk::bindings::default_app_on_connection)
            ));
        }
        if let Some(app_on_disconnect) = &self.app_on_disconnect {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_disconnect(param: *const da14531_sdk::ble_stack::host::gap::gapc::task::GapcDisconnectInd) {
                    #app_on_disconnect(unsafe{&*param});
                }
            ));
            struct_fields.push(quote!(app_on_disconnect: Some(__app_on_disconnect)));
        } else {
            struct_fields.push(quote!(app_on_disconnect: None));
        }
        if let Some(app_on_connect_failed) = &self.app_on_connect_failed {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_connect_failed() {
                    #app_on_connect_failed();
                }
            ));
            struct_fields.push(quote!(app_on_connect_failed: Some(__app_on_connect_failed)));
        } else {
            struct_fields.push(quote!(app_on_connect_failed: None));
        }
        if let Some(app_on_update_params_rejected) = &self.app_on_update_params_rejected {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_update_params_rejected() {
                    #app_on_update_params_rejected();
                }
            ));
            struct_fields.push(quote!(
                app_on_update_params_rejected: Some(__app_on_update_params_rejected)
            ));
        } else {
            struct_fields.push(quote!(app_on_update_params_rejected: None));
        }
        if let Some(app_on_update_params_complete) = &self.app_on_update_params_complete {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_update_params_complete() {
                    #app_on_update_params_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_update_params_complete: Some(__app_on_update_params_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_update_params_complete: None));
        }
        if let Some(app_on_set_dev_config_complete) = &self.app_on_set_dev_config_complete {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_set_dev_config_complete() {
                    #app_on_set_dev_config_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_set_dev_config_complete: Some(__app_on_set_dev_config_complete)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_set_dev_config_complete:
                    Some(da14531_sdk::bindings::default_app_on_set_dev_config_complete)
            ));
        }
        if let Some(app_on_adv_nonconn_complete) = &self.app_on_adv_nonconn_complete {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_adv_nonconn_complete() {
                    #app_on_adv_nonconn_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_adv_nonconn_complete: Some(__app_on_adv_nonconn_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_adv_nonconn_complete: None));
        }
        if let Some(app_on_adv_undirect_complete) = &self.app_on_adv_undirect_complete {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_adv_undirect_complete(status: u8) {
                    #app_on_adv_undirect_complete(status);
                }
            ));
            struct_fields.push(quote!(
                app_on_adv_undirect_complete: Some(__app_on_adv_undirect_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_adv_undirect_complete: None));
        }
        if let Some(app_on_adv_direct_complete) = &self.app_on_adv_direct_complete {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_adv_direct_complete() {
                    #app_on_adv_direct_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_adv_direct_complete: Some(__app_on_adv_direct_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_adv_direct_complete: None));
        }
        if let Some(app_on_db_init_complete) = &self.app_on_db_init_complete {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_db_init_complete() {
                    #app_on_db_init_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_db_init_complete: Some(__app_on_db_init_complete)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_db_init_complete:
                    Some(da14531_sdk::bindings::default_app_on_db_init_complete)
            ));
        }
        if let Some(app_on_scanning_completed) = &self.app_on_scanning_completed {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_scanning_completed() {
                    #app_on_scanning_completed();
                }
            ));
            struct_fields.push(quote!(
                app_on_scanning_completed: Some(__app_on_scanning_completed)
            ));
        } else {
            struct_fields.push(quote!(app_on_scanning_completed: None));
        }
        if let Some(app_on_adv_report_ind) = &self.app_on_adv_report_ind {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_adv_report_ind() {
                    #app_on_adv_report_ind();
                }
            ));
            struct_fields.push(quote!(app_on_adv_report_ind: Some(__app_on_adv_report_ind)));
        } else {
            struct_fields.push(quote!(app_on_adv_report_ind: None));
        }
        if let Some(app_on_get_dev_name) = &self.app_on_get_dev_name {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_get_dev_name() {
                    #app_on_get_dev_name();
                }
            ));
            struct_fields.push(quote!(app_on_get_dev_name: Some(__app_on_get_dev_name)));
        } else {
            struct_fields.push(quote!(
                app_on_get_dev_name: Some(da14531_sdk::bindings::default_app_on_get_dev_name)
            ));
        }
        if let Some(app_on_get_dev_appearance) = &self.app_on_get_dev_appearance {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_get_dev_appearance() {
                    #app_on_get_dev_appearance();
                }
            ));
            struct_fields.push(quote!(
                app_on_get_dev_appearance: Some(__app_on_get_dev_appearance)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_get_dev_appearance:
                    Some(da14531_sdk::bindings::default_app_on_get_dev_appearance)
            ));
        }
        if let Some(app_on_get_dev_slv_pref_params) = &self.app_on_get_dev_slv_pref_params {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_get_dev_slv_pref_params() {
                    #app_on_get_dev_slv_pref_params();
                }
            ));
            struct_fields.push(quote!(
                app_on_get_dev_slv_pref_params: Some(__app_on_get_dev_slv_pref_params)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_get_dev_slv_pref_params:
                    Some(da14531_sdk::bindings::default_app_on_get_dev_slv_pref_params)
            ));
        }
        if let Some(app_on_set_dev_info) = &self.app_on_set_dev_info {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_set_dev_info() {
                    #app_on_set_dev_info();
                }
            ));
            struct_fields.push(quote!(app_on_set_dev_info: Some(__app_on_set_dev_info)));
        } else {
            struct_fields.push(quote!(
                app_on_set_dev_info: Some(da14531_sdk::bindings::default_app_on_set_dev_info)
            ));
        }
        if let Some(app_on_data_length_change) = &self.app_on_data_length_change {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_data_length_change() {
                    #app_on_data_length_change();
                }
            ));
            struct_fields.push(quote!(
                app_on_data_length_change: Some(__app_on_data_length_change)
            ));
        } else {
            struct_fields.push(quote!(app_on_data_length_change: None));
        }
        if let Some(app_on_update_params_request) = &self.app_on_update_params_request {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_update_params_request() {
                    #app_on_update_params_request();
                }
            ));
            struct_fields.push(quote!(
                app_on_update_params_request: Some(__app_on_update_params_request)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_update_params_request:
                    Some(da14531_sdk::bindings::default_app_update_params_request)
            ));
        }
        if let Some(app_on_generate_static_random_addr) = &self.app_on_generate_static_random_addr {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_generate_static_random_addr() {
                    #app_on_generate_static_random_addr();
                }
            ));
            struct_fields.push(quote!(
                app_on_generate_static_random_addr: Some(__app_on_generate_static_random_addr)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_generate_static_random_addr:
                    Some(da14531_sdk::bindings::default_app_generate_static_random_addr)
            ));
        }
        if let Some(app_on_svc_changed_cfg_ind) = &self.app_on_svc_changed_cfg_ind {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_svc_changed_cfg_ind() {
                    #app_on_svc_changed_cfg_ind();
                }
            ));
            struct_fields.push(quote!(
                app_on_svc_changed_cfg_ind: Some(__app_on_svc_changed_cfg_ind)
            ));
        } else {
            struct_fields.push(quote!(app_on_svc_changed_cfg_ind: None));
        }
        if let Some(app_on_get_peer_features) = &self.app_on_get_peer_features {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_get_peer_features() {
                    #app_on_get_peer_features();
                }
            ));
            struct_fields.push(quote!(
                app_on_get_peer_features: Some(__app_on_get_peer_features)
            ));
        } else {
            struct_fields.push(quote!(app_on_get_peer_features: None));
        }

        quote!(
            #(#callback_wrappers)*

            #[export_name = "user_app_callbacks"]
            pub static USER_APP_CALLBACKS: da14531_sdk::app_modules::AppCallbacks =
                da14531_sdk::app_modules::AppCallbacks {
                #(#struct_fields),*
            };
        )
    }
}
