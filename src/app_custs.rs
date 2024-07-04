use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    ops::Deref,
};

use indexmap::IndexMap;
use proc_macro2::{Group, Ident, Span};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    token, Error, LitInt, LitStr, Path, Token,
};

#[derive(Debug)]
pub enum Uuid {
    Uuid16(u16),
    Uuid128([u8; 16]),
}

impl ToTokens for Uuid {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Uuid::Uuid16(uuid16) => {
                tokens.extend(quote!(
                    #uuid16
                ));
            }
            Uuid::Uuid128(uuid128) => {
                tokens.extend(quote!(
                    [ #(#uuid128),* ]
                ));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PermissionVariants {
    Disabled = 0,
    Enabled = 1,
    Unauth = 2,
    Auth = 3,
    Secure = 4,
}

impl Default for PermissionVariants {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum UuidLength {
    L16 = 0,
    L32 = 1,
    L128 = 2,
    RFU = 3,
}

impl Default for UuidLength {
    fn default() -> Self {
        Self::L16
    }
}

#[derive(Default, Clone, Debug)]
pub struct Permissions {
    read: PermissionVariants,
    write: PermissionVariants,
    indication: PermissionVariants,
    notification: PermissionVariants,
    extended_properties_present: bool,
    broadcast_permission: bool,
    encryption_key_length_16_bytes: bool,
    write_command_accepted: bool,
    write_signed_accepted: bool,
    write_request_accepted: bool,
    uuid_length: UuidLength,
}

impl Permissions {
    pub fn get_bits(&self) -> u32 {
        let mut bits = 0u32;

        bits |= self.read as u32;
        bits |= (self.write as u32) << 3;
        bits |= (self.indication as u32) << 6;
        bits |= (self.notification as u32) << 9;
        bits |= (self.extended_properties_present as u32) << 12;
        bits |= (self.broadcast_permission as u32) << 13;
        bits |= (self.encryption_key_length_16_bytes as u32) << 14;
        bits |= (self.write_command_accepted as u32) << 15;
        bits |= (self.write_signed_accepted as u32) << 16;
        bits |= (self.write_request_accepted as u32) << 17;
        bits |= (self.uuid_length as u32) << 18;

        bits
    }

    pub fn is_readable(&self) -> bool {
        self.read != PermissionVariants::Disabled
    }
    pub fn is_writable(&self) -> bool {
        self.write != PermissionVariants::Disabled
    }
}

impl ToTokens for Permissions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let bits = self.get_bits();

        tokens.extend(quote!(
            #bits
        ));
    }
}

#[derive(Debug)]
pub enum CharacteristicLength {
    Int(u16),
    Path(Path),
}

impl ToTokens for CharacteristicLength {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CharacteristicLength::Int(int) => {
                tokens.extend(quote!(
                    #int
                ));
            }
            CharacteristicLength::Path(path) => {
                tokens.extend(quote!(
                    #path
                ));
            }
        }
    }
}

#[derive(Debug)]
pub struct Characteristic {
    span: Span,
    name: String,
    permissions: Permissions,
    uuid: Uuid,
    length: CharacteristicLength,
    user_description: Option<String>,
    write_handler: Option<Path>,
    read_handler: Option<Path>,
}

impl Characteristic {
    fn parse_uuid(records: &Records) -> syn::Result<Uuid> {
        if let Some(uuid) = records.get("uuid") {
            if let RecordValueData::LitInt(uuid) = &uuid.data {
                let uuid: u16 = uuid.base10_parse()?;

                return Ok(Uuid::Uuid16(uuid));
            } else {
                return Err(Error::new(uuid.span, "expected integer literal"));
            }
        } else {
            return Err(Error::new(records.span, "missing `uuid`"));
        }
    }

    fn parse_permissions(records: &Records) -> syn::Result<Permissions> {
        if let Some(permissions) = records.get("permissions") {
            if let RecordValueData::Flags(permission_flags) = &permissions.data {
                let span = permissions.span;
                let permission_flags = permission_flags.deref();
                let mut permissions = Permissions::default();

                if permission_flags
                    .iter()
                    .filter(|p| p.starts_with("READ_"))
                    .count()
                    > 1
                {
                    return Err(Error::new(
                        span,
                        &format!("defined multiple read permissions: {permission_flags:?}"),
                    ));
                }

                if permission_flags
                    .iter()
                    .filter(|p| p.starts_with("WRITE_") && !p.ends_with("_ACCEPTED"))
                    .count()
                    > 1
                {
                    return Err(Error::new(
                        span,
                        &format!("defined multiple write permissions: {permission_flags:?}"),
                    ));
                }

                if permission_flags
                    .iter()
                    .filter(|p| p.starts_with("INDICATION_"))
                    .count()
                    > 1
                {
                    return Err(Error::new(
                        span,
                        &format!("defined multiple indication permissions: {permission_flags:?}"),
                    ));
                }

                if permission_flags
                    .iter()
                    .filter(|p| p.starts_with("NOTIFICATION_"))
                    .count()
                    > 1
                {
                    return Err(Error::new(
                        span,
                        &format!("defined multiple notification permissions: {permission_flags:?}"),
                    ));
                }

                for permission_flag in permission_flags {
                    match permission_flag.as_str() {
                        "READ_ENABLED" => {
                            permissions.read = PermissionVariants::Enabled;
                        }
                        "READ_AUTH" => {
                            permissions.read = PermissionVariants::Auth;
                        }
                        "READ_UNAUTH" => {
                            permissions.read = PermissionVariants::Unauth;
                        }
                        "READ_SECURE" => {
                            permissions.read = PermissionVariants::Secure;
                        }
                        "WRITE_ENABLED" => {
                            permissions.write = PermissionVariants::Enabled;
                        }
                        "WRITE_AUTH" => {
                            permissions.write = PermissionVariants::Auth;
                        }
                        "WRITE_UNAUTH" => {
                            permissions.write = PermissionVariants::Unauth;
                        }
                        "WRITE_SECURE" => {
                            permissions.write = PermissionVariants::Secure;
                        }
                        "INDICATION_ENABLED" => {
                            permissions.indication = PermissionVariants::Enabled;
                        }
                        "INDICATION_AUTH" => {
                            permissions.indication = PermissionVariants::Auth;
                        }
                        "INDICATION_UNAUTH" => {
                            permissions.indication = PermissionVariants::Unauth;
                        }
                        "INDICATION_SECURE" => {
                            permissions.indication = PermissionVariants::Secure;
                        }
                        "NOTIFICATION_ENABLED" => {
                            permissions.notification = PermissionVariants::Enabled;
                        }
                        "NOTIFICATION_AUTH" => {
                            permissions.notification = PermissionVariants::Auth;
                        }
                        "NOTIFICATION_UNAUTH" => {
                            permissions.notification = PermissionVariants::Unauth;
                        }
                        "NOTIFICATION_SECURE" => {
                            permissions.notification = PermissionVariants::Secure;
                        }
                        "WRITE_COMMAND_ACCEPTED" => {
                            permissions.write_command_accepted = true;
                        }
                        "WRITE_REQUEST_ACCEPTED" => {
                            permissions.write_request_accepted = true;
                        }
                        _ => {
                            return Err(Error::new(
                                span,
                                &format!("unknown flag: {permission_flag}"),
                            ));
                        }
                    }
                }

                return Ok(permissions);
            } else {
                return Err(Error::new(permissions.span, "expected flags"));
            }
        } else {
            return Err(Error::new(records.span, "missing `permissions`"));
        }
    }

    fn parse_length(records: &Records) -> syn::Result<CharacteristicLength> {
        if let Some(length) = records.get("length") {
            match &length.data {
                RecordValueData::LitInt(length) => {
                    let length: u16 = length.base10_parse()?;

                    return Ok(CharacteristicLength::Int(length));
                }
                RecordValueData::Path(path) => {
                    return Ok(CharacteristicLength::Path(path.clone()));
                }
                _ => return Err(Error::new(length.span, "expected integer literal")),
            }
        } else {
            return Err(Error::new(records.span, "missing `length`"));
        }
    }

    fn parse_user_description(records: &Records) -> syn::Result<Option<String>> {
        if let Some(user_description) = records.get("user_description") {
            if let RecordValueData::LitStr(user_description) = &user_description.data {
                return Ok(Some(user_description.token().to_string()));
            } else {
                return Err(Error::new(user_description.span, "expected string literal"));
            }
        } else {
            return Ok(None);
        }
    }

    fn parse_read_handler(records: &Records) -> syn::Result<Option<Path>> {
        if let Some(read_handler) = records.get("read_handler") {
            if let RecordValueData::Path(read_handler) = &read_handler.data {
                return Ok(Some(read_handler.clone()));
            } else {
                return Err(Error::new(read_handler.span, "expected path"));
            }
        } else {
            return Ok(None);
        }
    }

    fn parse_write_handler(records: &Records) -> syn::Result<Option<Path>> {
        if let Some(write_handler) = records.get("write_handler") {
            if let RecordValueData::Path(write_handler) = &write_handler.data {
                return Ok(Some(write_handler.clone()));
            } else {
                return Err(Error::new(write_handler.span, "expected path"));
            }
        } else {
            return Ok(None);
        }
    }

    fn parse(span: Span, name: &str, records: &Records) -> syn::Result<Self> {
        Ok(Self {
            span,
            name: name.into(),
            permissions: Self::parse_permissions(records)?,
            uuid: Self::parse_uuid(records)?,
            length: Self::parse_length(records)?,
            user_description: Self::parse_user_description(records)?,
            write_handler: Self::parse_write_handler(records)?,
            read_handler: Self::parse_read_handler(records)?,
        })
    }
}

#[derive(Debug)]
pub struct Service {
    name: String,
    characteristics: Vec<Characteristic>,
    uuid: Uuid,
}

impl Service {
    fn parse_uuid(records: &Records) -> syn::Result<Uuid> {
        if let Some(uuid) = records.get("uuid") {
            if let RecordValueData::LitInt(uuid) = &uuid.data {
                let uuid: u16 = uuid.base10_parse()?;

                return Ok(Uuid::Uuid16(uuid));
            } else {
                return Err(Error::new(uuid.span, "expected integer literal"));
            }
        } else {
            return Err(Error::new(records.span, "missing `uuid`"));
        }
    }

    fn parse_characteristics(records: &Records) -> syn::Result<Vec<Characteristic>> {
        if let Some(characteristics) = records.get("characteristics") {
            if let RecordValueData::Records(characteristics) = &characteristics.data {
                return Ok(characteristics
                    .iter()
                    .map(|(name, records)| match &records.data {
                        RecordValueData::Records(records) => {
                            Characteristic::parse(records.span, name, records)
                        }
                        _ => Err(Error::new(records.span, "expected record")),
                    })
                    .try_collect()?);
            } else {
                return Err(Error::new(
                    characteristics.span,
                    "expected characteristics records",
                ));
            }
        } else {
            return Err(Error::new(records.span, "missing `characteristics`"));
        }
    }

    fn parse(name: &str, records: &Records) -> syn::Result<Self> {
        let uuid = Self::parse_uuid(records)?;
        let characteristics = Self::parse_characteristics(records)?;

        Ok(Self {
            name: name.to_string(),
            characteristics,
            uuid,
        })
    }
}

#[derive(Debug)]
pub struct Flags(HashSet<String>);

impl std::ops::Deref for Flags {
    type Target = HashSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for Flags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group = input.call(Group::parse)?;
        let stream = group.stream();
        let fields = Parser::parse2(Punctuated::<Ident, token::Or>::parse_terminated, stream)?;
        Ok(Self(
            fields.into_iter().map(|ident| ident.to_string()).collect(),
        ))
    }
}

#[derive(Debug)]
pub enum RecordValueData {
    Records(Records),
    Flags(Flags),
    LitInt(LitInt),
    LitStr(LitStr),
    Path(Path),
}

#[derive(Debug)]
pub struct RecordValue {
    span: Span,
    data: RecordValueData,
}

impl Parse for RecordValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let _colon_token: Token![:] = input.parse()?;

        let forked_input = input.fork();

        if let Ok(group) = forked_input.parse::<Group>() {
            let stream = group.stream();

            if let Ok(fields) =
                Parser::parse2(Punctuated::<Record, token::Comma>::parse_terminated, stream)
            {
                input.advance_to(&forked_input);

                let records = fields.into_iter().map(|r| (r.key.key, r.value)).collect();

                return Ok(Self {
                    span,
                    data: RecordValueData::Records(Records { span, map: records }),
                });
            }
        }

        let forked_input = input.fork();
        if let Ok(lit_int) = forked_input.parse::<LitInt>() {
            input.advance_to(&forked_input);
            let span = lit_int.span();
            return Ok(Self {
                span,
                data: RecordValueData::LitInt(lit_int),
            });
        }

        let forked_input = input.fork();
        if let Ok(lit_str) = forked_input.parse::<LitStr>() {
            input.advance_to(&forked_input);
            let span = lit_str.span();
            return Ok(Self {
                span,
                data: RecordValueData::LitStr(lit_str),
            });
        }

        let forked_input = input.fork();
        if let Ok(flags) = forked_input.parse::<Flags>() {
            input.advance_to(&forked_input);
            let span = forked_input.span();
            return Ok(Self {
                span,
                data: RecordValueData::Flags(flags),
            });
        }

        let forked_input = input.fork();
        if let Ok(path) = forked_input.parse::<Path>() {
            input.advance_to(&forked_input);
            let span = path.span();
            return Ok(Self {
                span,
                data: RecordValueData::Path(path),
            });
        }

        Err(Error::new(
            input.span(),
            &format!("unexpected input: {input:?}"),
        ))
    }
}

#[derive(Debug)]
pub struct RecordKey {
    span: Span,
    key: String,
}

impl Parse for RecordKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let key: Ident = input.parse()?;
        let key = key.to_string();
        Ok(Self { span, key })
    }
}

#[derive(Debug)]
pub struct Record {
    span: Span,
    key: RecordKey,
    value: RecordValue,
}

impl Parse for Record {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let key: RecordKey = input.parse()?;
        let value: RecordValue = input.parse()?;

        Ok(Self { span, key, value })
    }
}

#[derive(Debug)]
pub struct Records {
    span: Span,
    map: IndexMap<String, RecordValue>,
}

impl std::ops::Deref for Records {
    type Target = IndexMap<String, RecordValue>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Parse for Records {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let fields: Punctuated<Record, token::Comma> = input.parse_terminated(Record::parse)?;

        Ok(Records {
            span,
            map: fields.into_iter().map(|r| (r.key.key, r.value)).collect(),
        })
    }
}

#[derive(Debug)]
pub struct CustomServer1ServiceConfiguration {
    services: Vec<Service>,
    service_idxs: Vec<u8>,
    char_idx_map: HashMap<String, usize>,
    write_handlers: Vec<(u16, Path)>,
    read_handlers: Vec<(u16, Path)>,
}

impl Parse for CustomServer1ServiceConfiguration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let records: Records = input.parse()?;

        let services = records
            .iter()
            .map(|(name, records)| match &records.data {
                RecordValueData::Records(records) => Service::parse(name, records),
                _ => Err(Error::new(records.span, "expected record")),
            })
            .try_collect()?;

        Ok(Self {
            services,
            service_idxs: Vec::new(),
            char_idx_map: HashMap::new(),
            write_handlers: Vec::new(),
            read_handlers: Vec::new(),
        })
    }
}

impl CustomServer1ServiceConfiguration {
    fn generate_att_db_records(&mut self) -> syn::Result<Vec<proc_macro2::TokenStream>> {
        let mut records = Vec::new();

        for service in &self.services {
            let uuid = &service.uuid;
            self.service_idxs.push(records.len() as u8);

            let mut read_permission = Permissions::default();

            read_permission.read = PermissionVariants::Enabled;

            records.push(quote!(
                da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                    uuid: &da14531_sdk::ble_stack::host::att::ATT_DECL_PRIMARY_SERVICE as *const _ as *const u8,
                    uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                    perm: #read_permission,
                    max_length: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u16,
                    length: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u16,
                    value: &(#uuid) as *const _ as *const u8,
                }
            ));

            for characteristic in &service.characteristics {
                let perm = &characteristic.permissions;
                let perm_dbg = format!("Permissions: {perm:?}");
                let perm_dbg_bits = format!("Permissions: {:#032b}", perm.get_bits());
                let uuid = &characteristic.uuid;
                let length = &characteristic.length;
                let mut trigger_read_indication = quote!();
                records.push(quote!(
                    da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                        uuid: &da14531_sdk::ble_stack::host::att::ATT_DECL_CHARACTERISTIC
                            as *const _ as *const u8,
                        uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                        perm: #read_permission,
                        max_length: 0,
                        length: 0,
                        value: core::ptr::null(),
                    }
                ));
                self.char_idx_map
                    .insert(characteristic.name.clone(), records.len());
                if perm.is_readable() {
                    if let Some(read_handler) = &characteristic.read_handler {
                        self.read_handlers
                            .push((records.len() as u16, read_handler.clone()));
                        trigger_read_indication = quote!(
                            (1<<15) |
                        );
                    } else {
                        return Err(Error::new(
                            characteristic.span,
                            "characteristic has read permission but no read handler",
                        ));
                    }
                }
                if perm.is_writable() {
                    if let Some(write_handler) = &characteristic.write_handler {
                        self.write_handlers
                            .push((records.len() as u16, write_handler.clone()));
                        trigger_read_indication = quote!(
                            (1<<15) |
                        );
                    } else {
                        return Err(Error::new(
                            characteristic.span,
                            "characteristic has write permission but no write handler",
                        ));
                    }
                }
                records.push(quote!(
                    #[doc = #perm_dbg]
                    #[doc = #perm_dbg_bits]
                    da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                        uuid: &(#uuid) as *const _ as *const u8,
                        uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                        perm: #perm,
                        max_length: #trigger_read_indication #length,
                        length: 0,
                        value: core::ptr::null(),
                    }
                ));

                if let Some(user_description) = &characteristic.user_description {
                    let user_description = user_description.as_bytes();
                    let user_description_len = user_description.len() as u16;
                    records.push(quote!(
                        da14531_sdk::ble_stack::host::att::attm::AttmDesc128 {
                            uuid: &da14531_sdk::ble_stack::host::att::ATT_DESC_CHAR_USER_DESCRIPTION as *const _ as *const u8,
                            uuid_size: da14531_sdk::ble_stack::host::att::ATT_UUID_16_LEN as u8,
                            perm: #read_permission,
                            max_length: #user_description_len,
                            length: #user_description_len,
                            value: &[#(#user_description),*] as *const _ as *const u8,
                        }
                    ));
                }
            }
        }

        Ok(records)
    }

    fn generate_att_db(&mut self) -> syn::Result<proc_macro2::TokenStream> {
        let records = self.generate_att_db_records()?;
        let record_count = records.len();
        let record_count_u8 = record_count as u8;
        let service_idxs = &self.service_idxs;
        let services_len = service_idxs.len();
        Ok(quote!(
            #[export_name = "custs1_att_db"]
            pub(crate) static CUSTS1_ATT_DB: [da14531_sdk::ble_stack::host::att::attm::AttmDesc128;
                #record_count] = [
                    #(#records),*
            ];
            const CUSTS1_ATT_DB_LEN: u8 = #record_count_u8;
            #[export_name = "custs1_services"]
            static CUSTS1_SERVICES: [u8; #services_len + 1] = [#(#service_idxs),* , #record_count_u8];
            #[export_name = "custs1_services_size"]
            static CUSTS1_SERVICES_SIZE: u32 = #services_len as u32;



            /// Setup custom profile funcs
            #[no_mangle]
            pub static CUST_PRF_FUNCS: [CustPrfFuncCallbacks; 1] = [CustPrfFuncCallbacks {
                task_id: TASK_ID_CUSTS1,
                att_db: &CUSTS1_ATT_DB as *const _ as *const da14531_sdk::bindings::attm_desc_128,
                max_nb_att: CUSTS1_ATT_DB_LEN,
                db_create_func: Some(app_custs1_create_db),
                enable_func: None,
                init_func: None,
                value_wr_validation_func: None,
            }];


            #[no_mangle]
            pub extern "C" fn custs_get_func_callbacks(task_id: da14531_sdk::platform::core_modules::rwip::KeApiId) -> *const da14531_sdk::app_modules::app_custs::CustPrfFuncCallbacks {
                for pfcb in &CUST_PRF_FUNCS {
                    if pfcb.task_id == task_id {

                        let pfcb_ptr = pfcb as *const _ as *const da14531_sdk::app_modules::app_custs::CustPrfFuncCallbacks;

                        return pfcb_ptr;
                    } else if pfcb.task_id == da14531_sdk::platform::core_modules::rwip::TASK_ID_INVALID {
                        break;
                    }
                }
                core::ptr::null()
            }

            #[export_name = "rom_cust_prf_cfg"]
            static ROM_CUST_PRF_CFG: da14531_sdk::ble_stack::profiles::custom::custs::RomCustPrfCfg =
                da14531_sdk::ble_stack::profiles::custom::custs::RomCustPrfCfg {
                    custs1_services: CUSTS1_SERVICES.as_ptr(),
                    custs1_services_size: &(#services_len as u8),
                    custs1_att_db: CUSTS1_ATT_DB.as_ptr() as *mut _,
                    custs_get_func_callbacks: Some(
                        custs_get_func_callbacks,
                    ),
                };

        ))
    }

    fn generate_user_catch_rest_handler(&mut self) -> syn::Result<proc_macro2::TokenStream> {
        let write_handlers = self.write_handlers.iter().map(|(idx, handler)| {
            quote!(
                #idx => {#handler(param)}
            )
        });
        let read_handlers = self.read_handlers.iter().map(|(idx, handler)| {
            quote!(
                #idx => {#handler(param)}
            )
        });

        Ok(quote!(
            /// Handles the messages that are not handled by the SDK internal mechanisms.
            ///
            /// # Arguments
            /// * `msg_id` - Id of the message received.
            /// * `param` - Pointer to the parameters of the message.
            /// * `dest_id` - ID of the receiving task instance.
            /// * `src_id` - ID of the sending task instance.
            #[no_mangle]
            pub fn user_catch_rest_hndl(
                msg_id: da14531_sdk::platform::core_modules::ke::msg::KeMsgId,
                param: *const cty::c_void,
                dest_id: da14531_sdk::platform::core_modules::ke::task::KeTaskId,
                src_id: da14531_sdk::platform::core_modules::ke::task::KeTaskId,
            ) {
                // rtt_target::rprintln!(
                //     "user_catch_rest_hndl({}, {:p}, {}, {})",
                //     msg_id,
                //     param,
                //     dest_id,
                //     src_id
                // );

                match msg_id as u32 {
                    da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::CUSTS1_VAL_WRITE_IND => {
                        let param = param as *const da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::Custs1ValWriteInd;
                        let param = unsafe { &*param };
                        match param.handle {
                            #(#write_handlers),*
                            _ => {}
                        }
                    }
                    // da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::CUSTS1_VAL_NTF_CFM => {
                    //     rprintln!("CUSTS1_VAL_NTF_CFM");
                    //     let param = param as *const Custs1ValNtfCfm;
                    //     let param = unsafe { *param };
                    //     match param.handle {
                    //         _ => {}
                    //     }
                    // }
                    // da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::CUSTS1_VAL_IND_CFM => {
                    //     rprintln!("CUSTS1_VAL_IND_CFM");
                    //     let param = param as *const Custs1ValIndCfm;
                    //     let param = unsafe { *param };
                    //     match param.handle {
                    //         _ => {}
                    //     }
                    // }
                    da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::CUSTS1_ATT_INFO_REQ => {
                        let param = param as *const da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::Custs1AttInfoReq;
                        let param = unsafe { &*param };
                        let att_idx = param.att_idx;

                        match att_idx {
                            _ => {
                                let mut response = da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::KeMsgCusts1AttInfoRsp::new(src_id, dest_id);

                                let conidx = da14531_sdk::app_modules::app_env_get_conidx(param.conidx);

                                // Provide the connection index.
                                response.fields().conidx = conidx;

                                // Provide the attribute index.
                                response.fields().att_idx = param.att_idx;

                                // Force current length to zero.
                                response.fields().length = 0;

                                // Provide the ATT error code.
                                response.fields().status = da14531_sdk::ble_stack::rwble_hl::error::HlErr::ATT_ERR_WRITE_NOT_PERMITTED as u8;

                                response.send();
                            }
                        }
                    }
                    da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::CUSTS1_VALUE_REQ_IND => {
                        let param = param as *const da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::Custs1ValueReqInd;
                        let param = unsafe { &*param };
                        let att_idx = param.att_idx;

                        match att_idx {
                            #(#read_handlers),*
                            _ => {
                                let mut response = da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::KeMsgCusts1ValueReqRsp::new(dest_id, src_id);

                                // Provide the connection index.
                                response.fields().conidx = da14531_sdk::app_modules::app_env_get_conidx(param.conidx);

                                // Provide the attribute index.
                                response.fields().att_idx = param.att_idx;

                                // Force current length to zero.
                                response.fields().length = 0;

                                // Provide the ATT error code.
                                response.fields().status = da14531_sdk::ble_stack::rwble_hl::error::HlErr::ATT_ERR_APP_ERROR as u8;

                                response.send();
                            }
                        }
                    }
                    da14531_sdk::ble_stack::host::gap::gapc::task::GAPC_PARAM_UPDATED_IND => {
                        // let param = param as *const da14531_sdk::ble_stack::host::gap::gapc::task::GapcParamUpdatedInd;
                        // let param = unsafe { &*param };
                        // let con_interval = param.con_interval;
                        // let con_latency = param.con_latency;
                        // let sup_to = param.sup_to;
                    }
                    _ => {}
                }
            }
        ))
    }

    pub fn generate(&mut self) -> syn::Result<proc_macro2::TokenStream> {
        let att_db = self.generate_att_db()?;

        let user_catch_rest_handler = self.generate_user_catch_rest_handler()?;

        Ok(quote!(
            #att_db

            #user_catch_rest_handler
        ))
    }
}
