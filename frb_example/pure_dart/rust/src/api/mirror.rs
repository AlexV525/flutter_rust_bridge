// Mirroring example:
// The goal of mirroring is to use external objects without needing to convert them with an intermediate type
// In this case, the struct ApplicationSettings is defined in another crate (called external-lib)

// To use an external type with mirroring, it MUST be imported publicly (aka. re-export)
use crate::auxiliary::sample_types::{MyEnum, MyStruct};
use flutter_rust_bridge::{frb, StreamSink};
pub use frb_example_pure_dart_exapmle_external_lib::{
    ApplicationEnv, ApplicationEnvVar, ApplicationMessage, ApplicationMode, ApplicationSettings,
    ListOfNestedRawStringMirrored, NestedRawStringMirrored, Numbers, RawStringEnumMirrored,
    RawStringMirrored, Sequences,
};

// To mirror an external struct, you need to define a placeholder type with the same definition
#[frb(mirror(ApplicationSettings))]
pub struct _ApplicationSettingsTwinNormal {
    pub name: String,
    pub version: String,
    pub mode: ApplicationMode,
    pub env: Box<ApplicationEnv>,
    pub env_optional: Option<ApplicationEnv>,
}

#[frb(mirror(ApplicationMode))]
pub enum _ApplicationModeTwinNormal {
    Standalone,
    Embedded,
}

#[frb(mirror(ApplicationEnvVar))]
pub struct _ApplicationEnvVarTwinNormal(pub String, pub bool);

#[frb(mirror(ApplicationEnv))]
pub struct _ApplicationEnvTwinNormal {
    pub vars: Vec<ApplicationEnvVar>,
}

// This function can directly return an object of the external type ApplicationSettings because it has a mirror
pub fn get_app_settings_twin_normal() -> ApplicationSettings {
    frb_example_pure_dart_exapmle_external_lib::get_app_settings()
}

// This function can return a Result, that includes an object of the external type ApplicationSettings because it has a mirror
pub fn get_fallible_app_settings_twin_normal() -> anyhow::Result<ApplicationSettings> {
    Ok(frb_example_pure_dart_exapmle_external_lib::get_app_settings())
}

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
pub fn is_app_embedded_twin_normal(app_settings: ApplicationSettings) -> bool {
    // info!("env: {:?}", app_settings.env.vars);
    matches!(app_settings.mode, ApplicationMode::Embedded)
}

// use a stream of a mirrored type
pub fn app_settings_stream_twin_normal(sink: StreamSink<ApplicationSettings>) {
    let app_settings = frb_example_pure_dart_exapmle_external_lib::get_app_settings();
    sink.add(app_settings);
    sink.close();
}

// use a stream of a vec of mirrored type
pub fn app_settings_vec_stream_twin_normal(sink: StreamSink<Vec<ApplicationSettings>>) {
    let app_settings = vec![
        frb_example_pure_dart_exapmle_external_lib::get_app_settings(),
        frb_example_pure_dart_exapmle_external_lib::get_app_settings(),
    ];
    sink.add(app_settings);
    sink.close();
}

pub struct MirrorStructTwinNormal {
    pub a: ApplicationSettings,
    pub b: MyStruct,
    pub c: Vec<MyEnum>,
    pub d: Vec<ApplicationSettings>,
}

// use a Struct consisting of mirror types as argument to a Stream
pub fn mirror_struct_stream_twin_normal(sink: StreamSink<MirrorStruct>) {
    let val = MirrorStruct {
        a: frb_example_pure_dart_exapmle_external_lib::get_app_settings(),
        b: MyStruct { content: true },
        c: vec![MyEnum::True, MyEnum::False],
        d: vec![
            frb_example_pure_dart_exapmle_external_lib::get_app_settings(),
            frb_example_pure_dart_exapmle_external_lib::get_app_settings(),
        ],
    };
    sink.add(val);
    sink.close();
}

// usa a tuple of Mirror types for a StreamSink
pub fn mirror_tuple_stream_twin_normal(
    sink: StreamSink<(ApplicationSettings, RawStringEnumMirrored)>,
) {
    let tuple = (
        frb_example_pure_dart_exapmle_external_lib::get_app_settings(),
        RawStringEnumMirrored::Raw(RawStringMirrored {
            value: String::from("test"),
        }),
    );
    sink.add(tuple);
    sink.close();
}

#[frb(mirror(ApplicationMessage))]
pub enum _ApplicationMessageTwinNormal {
    DisplayMessage(String),
    RenderPixel { x: i32, y: i32 },
    Exit,
}

pub fn get_message_twin_normal() -> ApplicationMessage {
    frb_example_pure_dart_exapmle_external_lib::poll_messages()[1].clone()
}

#[frb(mirror(Numbers, Sequences))]
pub struct _NumbersTwinNormal(pub Vec<i32>);

pub fn repeat_number_twin_normal(num: i32, times: usize) -> Numbers {
    frb_example_pure_dart_exapmle_external_lib::repeat_number(num, times)
}

pub fn repeat_sequence_twin_normal(seq: i32, times: usize) -> Sequences {
    frb_example_pure_dart_exapmle_external_lib::repeat_sequences(seq, times)
}

pub fn first_number_twin_normal(nums: Numbers) -> Option<i32> {
    nums.0.first().copied()
}

pub fn first_sequence_twin_normal(seqs: Sequences) -> Option<i32> {
    seqs.0.first().copied()
}

#[frb(mirror(RawStringMirrored))]
pub struct _RawStringMirroredTwinNormal {
    pub r#value: String,
}

#[frb(mirror(NestedRawStringMirrored))]
pub struct _NestedRawStringMirroredTwinNormal {
    pub raw: RawStringMirrored,
}

#[frb(mirror(RawStringEnumMirrored))]
pub enum _RawStringEnumMirroredTwinNormal {
    Raw(RawStringMirrored),
    Nested(NestedRawStringMirrored),
    ListOfNested(ListOfNestedRawStringMirrored),
}

#[frb(mirror(ListOfNestedRawStringMirrored))]
pub struct _ListOfRawNestedStringMirroredTwinNormal {
    pub raw: Vec<NestedRawStringMirrored>,
}

pub fn test_raw_string_mirrored_twin_normal() -> RawStringMirrored {
    RawStringMirrored {
        r#value: "test".to_owned(),
    }
}

pub fn test_nested_raw_string_mirrored_twin_normal() -> NestedRawStringMirrored {
    NestedRawStringMirrored {
        raw: RawStringMirrored {
            r#value: "test".to_owned(),
        },
    }
}

pub fn test_raw_string_enum_mirrored_twin_normal(nested: bool) -> RawStringEnumMirrored {
    if nested {
        RawStringEnumMirrored::Nested(NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        })
    } else {
        RawStringEnumMirrored::Raw(RawStringMirrored {
            r#value: "test".to_owned(),
        })
    }
}

pub fn test_list_of_raw_nested_string_mirrored_twin_normal() -> ListOfNestedRawStringMirrored {
    ListOfNestedRawStringMirrored {
        raw: vec![NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        }],
    }
}

pub fn test_fallible_of_raw_string_mirrored_twin_normal() -> anyhow::Result<Vec<RawStringMirrored>>
{
    Ok(vec![RawStringMirrored {
        r#value: "test".to_owned(),
    }])
}

pub fn test_list_of_nested_enums_mirrored_twin_normal() -> Vec<RawStringEnumMirrored> {
    vec![
        RawStringEnumMirrored::Nested(NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        }),
        RawStringEnumMirrored::Raw(RawStringMirrored {
            r#value: "test".to_owned(),
        }),
    ]
}

// TODO rm (use the auto-generated sync code)
// pub fn sync_return_mirror_twin_normal() -> SyncReturn<ApplicationSettings> {
//     SyncReturn(frb_example_pure_dart_exapmle_external_lib::get_app_settings())
// }

pub struct AnotherTwinNormal {
    pub a: String,
}

pub struct ContainsMirroredSubStructTwinNormal {
    pub test: RawStringMirrored,
    pub test2: Another,
}

pub fn test_contains_mirrored_sub_struct_twin_normal() -> ContainsMirroredSubStruct {
    ContainsMirroredSubStruct {
        test: RawStringMirrored {
            r#value: "test".to_owned(),
        },
        test2: Another {
            a: "test".to_owned(),
        },
    }
}
