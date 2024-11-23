use crate as bevy_reflect;

use crate::{ReflectDeserialize, ReflectSerialize, std_traits::ReflectDefault};
use bevy_reflect_derive::impl_reflect_opaque;

impl_reflect_opaque!(::uuid::Uuid(
    Serialize,
    Deserialize,
    Default,
    Debug,
    PartialEq,
    Hash
));
