use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Expr, FieldValue, Member, Path, Token};

#[derive(Debug)]
pub struct DefaultHandlersConfiguration {
    adv_scenario: Option<Path>,
    advertise_period: Option<Path>,
    security_request_scenario: Option<Path>,
}

impl Parse for DefaultHandlersConfiguration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> =
            input.parse_terminated(FieldValue::parse)?;

        let mut callbacks = Self {
            adv_scenario: None,
            advertise_period: None,
            security_request_scenario: None,
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
                "adv_scenario" => match value {
                    Expr::Path(path) => {
                        callbacks.adv_scenario = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for adv_scenario: {:?}", value);
                    }
                },
                "advertise_period" => match value {
                    Expr::Path(path) => {
                        callbacks.advertise_period = Some(path.path);
                    }
                    _ => {
                        panic!("Unextpected expression for advertise_period: {:?}", value);
                    }
                },
                "security_request_scenario" => match value {
                    Expr::Path(path) => {
                        callbacks.security_request_scenario = Some(path.path);
                    }
                    _ => {
                        panic!(
                            "Unextpected expression for security_request_scenario: {:?}",
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

impl DefaultHandlersConfiguration {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        if let Some(adv_scenario) = &self.adv_scenario {
            if let Some(advertise_period) = &self.advertise_period {
                if let Some(security_request_scenario) = &self.security_request_scenario {
                    quote!(
                        #[export_name = "user_default_hnd_conf"]
                        pub static USER_DEFAULT_HND_CONF: da14531_sdk::app_modules::DefaultHandlersConfiguration =
                            da14531_sdk::app_modules::DefaultHandlersConfiguration {
                            adv_scenario: #adv_scenario,
                            advertise_period: #advertise_period,
                            security_request_scenario: #security_request_scenario
                        };
                    )
                } else {
                    panic!("Missing field: security_request_scenario");
                }
            } else {
                panic!("Missing field: advertise_period");
            }
        } else {
            panic!("Missing field: adv_scenario");
        }
    }
}
