use crate::api::misc_example::WeekdaysTwinNormal;
use flutter_rust_bridge::{frb, ZeroCopyBuffer};
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinNormal {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub fn handle_customized_struct_twin_normal(val: CustomizedTwinNormal) {
    info!("{:#?}", val);
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinNormal {
    #[frb(default = 0)]
    pub value: u32,
}

pub fn next_user_id_twin_normal(user_id: UserIdTwinNormal) -> UserIdTwinNormal {
    UserIdTwinNormal {
        value: user_id.value + 1,
    }
}

// Note: Some attributes are put on `KitchenSinkTwinNormal` currently
// (but we can add more tests in this file later)
