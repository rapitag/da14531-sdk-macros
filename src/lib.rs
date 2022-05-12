use std::fmt::Debug;

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, ResultExt};
use quote::quote;
use syn::{
    braced, parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, FieldValue, Lit, Member,
    Token,
};

enum EntryType {
    Service,
    Characteristic,
}

struct DatabaseEntryBuilder {
    entry_type: Option<EntryType>,
    uuid: Option<Expr>,
    uuid_len: Option<u8>,
    length: Option<Expr>,
    user_description: Option<Expr>,
    perm: Option<Expr>,
}

impl DatabaseEntryBuilder {
    fn new() -> Self {
        Self {
            entry_type: None,
            uuid: None,
            uuid_len: None,
            length: None,
            user_description: None,
            perm: None,
        }
    }

    pub fn set_type(mut self, entry_type: &String) -> Self {
        if self.entry_type.is_none() {
            self.entry_type = Some(match entry_type.as_str() {
                "service" => EntryType::Service,
                "characteristic" => EntryType::Characteristic,
                _ => {
                    panic!("Unkown entry type: {:?}", entry_type)
                }
            });
        } else {
            panic!("Entry type is set already set!");
        }

        self
    }

    pub fn set_uuid16(mut self, uuid: Expr) -> Self {
        if self.uuid.is_none() {
            self.uuid = Some(uuid);
            self.uuid_len = Some(2);
        } else {
            panic!("Uuid is set already!");
        }
        self
    }

    pub fn set_uuid128(mut self, uuid: Expr) -> Self {
        if self.uuid.is_none() {
            self.uuid = Some(uuid);
            self.uuid_len = Some(16);
        } else {
            panic!("Uuid is set already!");
        }
        self
    }

    pub fn set_length(mut self, length: Expr) -> Self {
        if let Some(entry_type) = &mut self.entry_type {
            match entry_type {
                EntryType::Service => {
                    panic!("Cannot set length for service!");
                }
                EntryType::Characteristic => {
                    self.length = Some(length);
                }
            };
        } else {
            panic!("Entry type not set!");
        }

        self
    }

    pub fn set_perm(mut self, perm: Expr) -> Self {
        self.perm = Some(perm);

        self
    }

    pub fn set_user_description(mut self, user_description: Expr) -> Self {
        if let Some(entry_type) = &mut self.entry_type {
            match entry_type {
                EntryType::Service => {
                    panic!("Cannot set user desciption for service!");
                }
                EntryType::Characteristic => {
                    self.user_description = Some(user_description);
                }
            };
        } else {
            panic!("Entry type not set!");
        }

        self
    }

    pub fn build(self) -> Option<DatabaseEntry> {
        match self.entry_type {
            Some(entry_type) => match entry_type {
                EntryType::Service => {
                    if let Some(uuid) = self.uuid {
                        Some(DatabaseEntry::Service { uuid })
                    } else {
                        panic!("Uuid not set!");
                    }
                }
                EntryType::Characteristic => {
                    if let Some(uuid) = self.uuid {
                        if let Some(length) = self.length {
                            if let Some(perm) = self.perm {
                                Some(DatabaseEntry::Characteristic {
                                    uuid,
                                    length,
                                    perm,
                                    user_description: self.user_description,
                                })
                            } else {
                                panic!("Perm not set!");
                            }
                        } else {
                            panic!("Length not set!");
                        }
                    } else {
                        panic!("Uuid not set!");
                    }
                }
            },
            None => None,
        }
    }
}

#[derive(Debug)]
enum DatabaseEntry {
    Service {
        uuid: Expr,
    },
    Characteristic {
        uuid: Expr,
        length: Expr,
        perm: Expr,
        user_description: Option<Expr>,
    },
}

impl DatabaseEntry {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let out = match self {
            DatabaseEntry::Service { uuid } => quote!(
                da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                    uuid: &da14531_sdk::ble_stack::host::att::ATT_DECL_PRIMARY_SERVICE as *const _ as *const u8,
                    uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                    perm: perm!(RD, ENABLE),
                    max_length: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u16,
                    length: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u16,
                    value: &(#uuid) as *const _ as *const u8,
                }
            ),
            DatabaseEntry::Characteristic {
                uuid,
                length,
                perm,
                user_description,
            } => {
                let mut entries = Vec::new();

                entries.push(quote!(
                    da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                        uuid: &da14531_sdk::ble_stack::host::att::ATT_DECL_CHARACTERISTIC
                            as *const _ as *const u8,
                        uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                        perm: perm!(RD, ENABLE),
                        max_length: 0,
                        length: 0,
                        value: core::ptr::null(),
                    }
                ));

                entries.push(quote!(
                    da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                        uuid: &(#uuid) as *const _ as *const u8,
                        uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                        perm: #perm,
                        max_length: perm!(RI, ENABLE) as u16 | #length,
                        length: 0,
                        value: core::ptr::null(),
                    }
                ));

                if let Some(user_description) = user_description {
                    let (user_description, user_description_len) = match user_description {
                        Expr::Lit(literal) => match &literal.lit {
                            Lit::Str(str_lit) => {
                                (str_lit.value().as_bytes().to_vec(), str_lit.value().len() as u16)},
                            _ => panic!("Invalid token for user_description, expected string literal, got: {:?}", literal)
                        },
                        _ => panic!("Invalid token for user_description, expected string literal, got: {:?}", user_description),
                    };
                    entries.push(quote!(
                        da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                            uuid: &da14531_sdk::ble_stack::host::att::ATT_DESC_CHAR_USER_DESCRIPTION as *const _ as *const u8,
                            uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                            perm: perm!(RD, ENABLE),
                            max_length: #user_description_len,
                            length: #user_description_len,
                            value: &[#(#user_description),*] as *const _ as *const u8,
                        }
                    ))
                }

                quote!(
                    #(#entries),*
                )
            }
        };

        out
    }

    pub fn entry_count(&self) -> usize {
        match self {
            DatabaseEntry::Service { uuid: _ } => 1,
            DatabaseEntry::Characteristic {
                uuid: _,
                length: _,
                perm: _,
                user_description,
            } => 2 + if user_description.is_some() { 1 } else { 0 },
        }
    }
}

impl Parse for DatabaseEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let block_content;
        braced!(block_content in input);

        let fields: Punctuated<FieldValue, Token![,]> = block_content
            .parse_terminated(FieldValue::parse)
            .unwrap_or_abort();

        let mut builder = DatabaseEntryBuilder::new();

        for field in fields {
            let key = match field.member {
                Member::Named(name) => name,
                Member::Unnamed(unnamed) => {
                    panic!("Unexpected unnamed field: {:?}", unnamed);
                }
            };
            let value = field.expr;
            match key.to_string().as_str() {
                "etype" => match value {
                    Expr::Path(path) => {
                        builder = builder.set_type(&path.path.segments[0].ident.to_string());
                    }
                    _ => {
                        panic!("Unextpected expression for etype: {:?}", value);
                    }
                },
                "uuid16" => builder = builder.set_uuid16(value),
                "uuid128" => builder = builder.set_uuid128(value),
                "length" => builder = builder.set_length(value),
                "perm" => builder = builder.set_perm(value),
                "user_description" => builder = builder.set_user_description(value),
                _ => {
                    panic!("Unexpected field: {} = {:?}", key.to_string(), value);
                }
            }
        }

        Ok(builder.build().unwrap())
    }
}

#[derive(Debug)]
struct DatabaseDefinition {
    entries: Vec<DatabaseEntry>,
}

impl Parse for DatabaseDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let raw_entries: Punctuated<DatabaseEntry, Token![,]> =
            input.parse_terminated(DatabaseEntry::parse)?;

        Ok(Self {
            entries: raw_entries.into_iter().collect(),
        })
    }
}

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
