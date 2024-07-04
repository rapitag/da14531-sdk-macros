use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, FieldValue, Lit, Member, Token,
};

#[derive(Debug)]
pub struct DeviceInformationServiceConfiguration {
    manufacturer_name: Option<Expr>,
    model_nb_str: Option<Expr>,
    system_id: Option<Expr>,
    pnp_id: Option<Expr>,
    hard_rev_str: Option<Expr>,
    serial_nb_str: Option<Expr>,
    firm_rev_str: Option<Expr>,
    sw_rev_str: Option<Expr>,
    ieee: Option<Expr>,
}

impl Parse for DeviceInformationServiceConfiguration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> =
            input.parse_terminated(FieldValue::parse)?;

        let mut app_diss_values = Self {
            manufacturer_name: None,
            model_nb_str: None,
            system_id: None,
            pnp_id: None,
            hard_rev_str: None,
            serial_nb_str: None,
            firm_rev_str: None,
            sw_rev_str: None,
            ieee: None,
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
                "manufacturer_name" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.manufacturer_name = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for manufacturer_name: {:?}", value);
                    }
                },
                "model_nb_str" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.model_nb_str = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for model_nb_str: {:?}", value);
                    }
                },
                "system_id" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.system_id = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for system_id: {:?}", value);
                    }
                },
                "pnp_id" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.pnp_id = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for pnp_id: {:?}", value);
                    }
                },
                "hard_rev_str" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.hard_rev_str = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for hard_rev_str: {:?}", value);
                    }
                },
                "serial_nb_str" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.serial_nb_str = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for serial_nb_str: {:?}", value);
                    }
                },
                "firm_rev_str" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.firm_rev_str = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for firm_rev_str: {:?}", value);
                    }
                },
                "sw_rev_str" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.sw_rev_str = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for sw_rev_str: {:?}", value);
                    }
                },
                "ieee" => match &value {
                    Expr::Lit(_) => {
                        app_diss_values.ieee = Some(value);
                    }
                    _ => {
                        panic!("Unextpected expression for ieee: {:?}", value);
                    }
                },
                _ => {
                    panic!("Unexpected field: {} = {:?}", key.to_string(), value);
                }
            }
        }

        Ok(app_diss_values)
    }
}

macro_rules! generate_handler {
    ($handlers: ident, $supported_chars: ident, $field: expr, $char: ident, $char_sup: ident) => {
        if let Some(field) = &$field {
            let (field_data, field_len) = match field {
                Expr::Lit(literal) => match &literal.lit {
                    Lit::Str(str_lit) => (
                        str_lit.value().as_bytes().to_vec(),
                        str_lit.value().len() as u16,
                    ),
                    _ => panic!(
                        "Invalid token for {}, expected string literal, got: {:?}",
                        stringify!($field).replace("self.", ""),
                        literal
                    ),
                },
                _ => panic!(
                    "Invalid token for {}, expected string literal, got: {:?}",
                    stringify!($field).replace("self.", ""),
                    field
                ),
            };

            $handlers.push(quote!(
                $char => {
                    let mut msg = KeMsgDynDissValueCfm::<#field_len>::new(dest_id, src_id);
                    unsafe {
                        msg.fields()
                            .data
                            .as_mut_slice(#field_len as usize)
                            .copy_from_slice(&[#(#field_data),*])
                    };
                    msg.fields().length = #field_len as u8;

                    msg.fields().value = param.value;

                    msg.send();
                }
            ));

            $supported_chars.push(quote!(
                db_cfg.features |= $char_sup as u16;
            ));
        }
    };
}

impl DeviceInformationServiceConfiguration {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let mut handlers = Vec::new();
        let mut supported_chars = Vec::new();

        generate_handler!(
            handlers,
            supported_chars,
            self.manufacturer_name,
            DIS_MANUFACTURER_NAME_CHAR,
            DIS_MANUFACTURER_NAME_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.firm_rev_str,
            DIS_FIRM_REV_STR_CHAR,
            DIS_FIRM_REV_STR_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.hard_rev_str,
            DIS_HARD_REV_STR_CHAR,
            DIS_HARD_REV_STR_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.ieee,
            DIS_IEEE_CHAR,
            DIS_IEEE_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.model_nb_str,
            DIS_MODEL_NB_STR_CHAR,
            DIS_MODEL_NB_STR_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.pnp_id,
            DIS_PNP_ID_CHAR,
            DIS_PNP_ID_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.serial_nb_str,
            DIS_SERIAL_NB_STR_CHAR,
            DIS_SERIAL_NB_STR_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.sw_rev_str,
            DIS_SW_REV_STR_CHAR,
            DIS_SW_REV_STR_CHAR_SUP
        );
        generate_handler!(
            handlers,
            supported_chars,
            self.system_id,
            DIS_SYSTEM_ID_CHAR,
            DIS_SYSTEM_ID_CHAR_SUP
        );

        quote!(
            mod app_diss_task {
                use da14531_sdk::{
                    bindings,
                    app_modules::ProcessEventResponse,
                    ble_stack::profiles::dis::diss::{
                        task::{DissValueReqInd, KeMsgDynDissValueCfm, DISS_VALUE_REQ_IND},
                        DIS_MANUFACTURER_NAME_CHAR, DIS_MODEL_NB_STR_CHAR, DIS_PNP_ID_CHAR,
                        DIS_SW_REV_STR_CHAR, DIS_SYSTEM_ID_CHAR, DIS_FIRM_REV_STR_CHAR,
                        DIS_HARD_REV_STR_CHAR, DIS_IEEE_CHAR, DIS_SERIAL_NB_STR_CHAR

                    },
                    platform::core_modules::ke::{
                        msg::{KeMsgHandler, KeMsgId, KeMsgStatusTag, KE_MSG_CONSUMED},
                        task::KeTaskId,
                    },
                };


                #[no_mangle]
                pub extern "C" fn diss_value_req_ind_handler(
                    _msg_id: KeMsgId,
                    param: *const cty::c_void,
                    dest_id: KeTaskId,
                    src_id: KeTaskId,
                ) -> i32 {
                    let param = param as *const DissValueReqInd;
                    let param = unsafe { &*param };
                    match param.value as u32 {
                        #(#handlers)*
                        _ => {
                            let mut msg = KeMsgDynDissValueCfm::<0>::new(dest_id, src_id);
                            msg.fields().length = 0;

                            msg.fields().value = param.value;

                            msg.send();
                        }
                    };

                    KE_MSG_CONSUMED as i32
                }

                static APP_DISS_PROCESS_HANDLERS: [KeMsgHandler; 1] = [KeMsgHandler {
                    id: DISS_VALUE_REQ_IND as u16,
                    func: Some(diss_value_req_ind_handler),
                }];

                #[no_mangle]
                pub extern "C" fn app_diss_process_handler(
                    msg_id: KeMsgId,
                    param: *const cty::c_void,
                    dest_id: KeTaskId,
                    src_id: KeTaskId,
                    msg_ret: *mut KeMsgStatusTag,
                ) -> ProcessEventResponse {
                    return unsafe {
                        bindings::app_std_process_event(
                            msg_id,
                            param,
                            src_id,
                            dest_id,
                            msg_ret,
                            APP_DISS_PROCESS_HANDLERS.as_ptr() as *mut _,
                            APP_DISS_PROCESS_HANDLERS.len() as i32,
                        )
                    };
                }
            }

            mod app_diss {
                use da14531_sdk::{
                    app_modules::get_user_prf_srv_perm,
                    ble_stack::{
                        host::gap::gapm::task::{KeMsgDynGapmProfileTaskAdd, GAPM_PROFILE_TASK_ADD},
                        profiles::dis::diss::{
                            DissDbCfg, DIS_FIRM_REV_STR_CHAR_SUP, DIS_HARD_REV_STR_CHAR_CHAR_SUP,
                            DIS_IEEE_CHAR_CHAR_SUP, DIS_MANUFACTURER_NAME_CHAR_SUP,
                            DIS_MODEL_NB_STR_CHAR_SUP, DIS_PNP_ID_CHAR_SUP,
                            DIS_SERIAL_NB_STR_CHAR_SUP, DIS_SW_REV_STR_CHAR_SUP,
                            DIS_SYSTEM_ID_CHAR_SUP,
                        },
                    },
                    platform::core_modules::rwip::{TASK_APP, TASK_GAPM, TASK_ID_DISS},
                };

                #[no_mangle]
                pub extern "C" fn app_dis_init() {
                    // Nothing to do
                }

                #[no_mangle]
                pub extern "C" fn app_diss_create_db() {
                    const SIZE: u16 = core::mem::size_of::<DissDbCfg>() as u16;
                    let mut msg = KeMsgDynGapmProfileTaskAdd::<SIZE>::new(TASK_APP as u16, TASK_GAPM as u16);

                    msg.fields().operation = GAPM_PROFILE_TASK_ADD as u8;
                    msg.fields().sec_lvl = get_user_prf_srv_perm(TASK_ID_DISS) as u8;
                    msg.fields().prf_task_id = TASK_ID_DISS as u16;
                    msg.fields().app_task = TASK_APP as u16;
                    msg.fields().start_hdl = 0;

                    let db_cfg_ptr = &mut msg.fields().param as *mut _ as *mut DissDbCfg;

                    let db_cfg = unsafe { db_cfg_ptr.as_mut().unwrap() };

                    db_cfg.features = 0;

                    #(#supported_chars)*

                    msg.send();
                }

            }
        )
    }
}
