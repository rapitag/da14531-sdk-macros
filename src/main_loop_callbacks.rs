use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Expr, FieldValue, Member, Path, Token};

#[derive(Debug)]
pub struct ArchMainLoopCallbacks {
    app_on_init: Option<Path>,
    app_on_ble_powered: Option<Path>,
    app_on_system_powered: Option<Path>,
    app_before_sleep: Option<Path>,
    app_validate_sleep: Option<Path>,
    app_going_to_sleep: Option<Path>,
    app_resume_from_sleep: Option<Path>,
}

impl Parse for ArchMainLoopCallbacks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> =
            input.parse_terminated(FieldValue::parse)?;

        let mut callbacks = Self {
            app_on_init: None,
            app_on_ble_powered: None,
            app_on_system_powered: None,
            app_before_sleep: None,
            app_validate_sleep: None,
            app_going_to_sleep: None,
            app_resume_from_sleep: None,
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
                "app_on_init" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_init = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_on_init: {:?}", value);
                    }
                },
                "app_on_ble_powered" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_ble_powered = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_on_ble_powered: {:?}", value);
                    }
                },
                "app_on_system_powered" => match value {
                    Expr::Path(path) => {
                        callbacks.app_on_system_powered = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_on_system_powered: {:?}",
                            value
                        );
                    }
                },
                "app_before_sleep" => match value {
                    Expr::Path(path) => {
                        callbacks.app_before_sleep = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_before_sleep: {:?}", value);
                    }
                },
                "app_validate_sleep" => match value {
                    Expr::Path(path) => {
                        callbacks.app_validate_sleep = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_validate_sleep: {:?}", value);
                    }
                },
                "app_going_to_sleep" => match value {
                    Expr::Path(path) => {
                        callbacks.app_going_to_sleep = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for app_going_to_sleep: {:?}", value);
                    }
                },
                "app_resume_from_sleep" => match value {
                    Expr::Path(path) => {
                        callbacks.app_resume_from_sleep = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for app_resume_from_sleep: {:?}",
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

impl ArchMainLoopCallbacks {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let mut callback_wrappers = Vec::new();
        let mut struct_fields = Vec::new();

        if let Some(app_on_init) = &self.app_on_init {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_init() {
                    #app_on_init();
                }
            ));
            struct_fields.push(quote!(app_on_init: Some(__app_on_init)));
        } else {
            struct_fields.push(quote!(app_on_init: None));
        }

        if let Some(app_on_ble_powered) = &self.app_on_ble_powered {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_ble_powered() -> da14531_sdk::platform::arch::ArchMainLoopCallbackRet {
                    #app_on_ble_powered()
                }
            ));
            struct_fields.push(quote!(app_on_ble_powered: Some(__app_on_ble_powered)));
        } else {
            struct_fields.push(quote!(app_on_ble_powered: None));
        }

        if let Some(app_on_system_powered) = &self.app_on_system_powered {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_on_system_powered() -> da14531_sdk::platform::arch::ArchMainLoopCallbackRet {
                    #app_on_system_powered()
                }
            ));
            struct_fields.push(quote!(app_on_system_powered: Some(__app_on_system_powered)));
        } else {
            struct_fields.push(quote!(app_on_system_powered: None));
        }

        if let Some(app_before_sleep) = &self.app_before_sleep {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_before_sleep() {
                    #app_before_sleep();
                }
            ));
            struct_fields.push(quote!(app_before_sleep: Some(__app_before_sleep)));
        } else {
            struct_fields.push(quote!(app_before_sleep: None));
        }

        if let Some(app_validate_sleep) = &self.app_validate_sleep {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_validate_sleep(mode: da14531_sdk::platform::arch::SleepMode) -> da14531_sdk::platform::arch::SleepMode {
                    #app_validate_sleep(mode)
                }
            ));
            struct_fields.push(quote!(app_validate_sleep: Some(__app_validate_sleep)));
        } else {
            struct_fields.push(quote!(app_validate_sleep: None));
        }

        if let Some(app_going_to_sleep) = &self.app_going_to_sleep {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_going_to_sleep(mode: da14531_sdk::platform::arch::SleepMode) {
                    #app_going_to_sleep(mode)
                }
            ));
            struct_fields.push(quote!(app_going_to_sleep: Some(__app_going_to_sleep)));
        } else {
            struct_fields.push(quote!(app_going_to_sleep: None));
        }

        if let Some(app_resume_from_sleep) = &self.app_resume_from_sleep {
            callback_wrappers.push(quote!(
                #[no_mangle]
                pub extern "C" fn __app_resume_from_sleep() {
                    #app_resume_from_sleep();
                }
            ));
            struct_fields.push(quote!(app_resume_from_sleep: Some(__app_resume_from_sleep)));
        } else {
            struct_fields.push(quote!(app_resume_from_sleep: None));
        }

        quote!(
            #(#callback_wrappers)*

            #[export_name = "user_app_main_loop_callbacks"]
            pub static USER_APP_MAIN_LOOP_CALLBACKS: da14531_sdk::platform::arch::ArchMainLoopCallbacks =
                da14531_sdk::platform::arch::ArchMainLoopCallbacks {
                #(#struct_fields),*
            };
        )
    }
}
