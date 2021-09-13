//! There is still a lot we can optimize VStr
//! * instead of using &str, we can use intern to cache static attr name.
//! * we can also cache camelize/capitalize result.
//! * if VStr raw already satisfy StrOps, setting the ops flag is noop.
//! * interning/cache can be optional, e.g. Text Token can skip it at all.
use super::is_event_prop;
use bitflags::bitflags;
#[cfg(test)]
use serde::Serialize;
use std::{fmt::Write, ops::Deref};

bitflags! {
    /// Represents idempotent string manipulation.
    // Idempotency is required since op is a bitflag.
    #[cfg_attr(test, derive(Serialize))]
    pub struct StrOps: u8 {
        const COMPRESS_WHITESPACE = 1 << 0;
        const DECODE_ENTITY       = 1 << 1;
        const CAMEL_CASE          = 1 << 2;
        const PASCAL_CASE         = 1 << 3;
        const IS_ATTR             = 1 << 4;
        const HANDLER_KEY         = 1 << 5;
        const VALID_ASSET         = 1 << 6;
        const SELF_SUFFIX         = 1 << 7; // not idempotent but called only once
    }
}

impl StrOps {
    // ideally it should be str.satisfy(op) but adding a trait
    // to str is too much. Use passive voice.
    fn is_satisfied_by(&self, s: &str) -> bool {
        todo!()
    }
    fn write_ops<W: Write>(&self, s: &str, w: W) {
        todo!()
    }
}

/// A str for Vue compiler's internal modification.
/// Instead of returning a Cow<str>, StrOp is recorded in the VStr
/// and will be processed later in codegen phase.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Serialize))]
pub struct VStr<'a> {
    pub raw: &'a str,
    pub ops: StrOps,
}

impl<'a> VStr<'a> {
    // adjective and is_xx for static method
    pub fn raw(raw: &'a str) -> Self {
        Self {
            raw,
            ops: StrOps::empty(),
        }
    }
    pub fn is_handler(s: &VStr) -> bool {
        if s.ops.contains(StrOps::HANDLER_KEY) {
            return true;
        }
        is_event_prop(s.raw)
    }
}
impl<'a> VStr<'a> {
    // verb is instance method
    pub fn decode(&mut self, is_attr: bool) -> &mut Self {
        let ops = if is_attr {
            StrOps::DECODE_ENTITY | StrOps::IS_ATTR
        } else {
            StrOps::DECODE_ENTITY
        };
        self.ops |= ops;
        self
    }
    pub fn camelize(&mut self) -> &mut Self {
        self.ops |= StrOps::CAMEL_CASE;
        self
    }
    pub fn capitalize(&mut self) -> &mut Self {
        self.ops |= StrOps::PASCAL_CASE;
        self
    }
    pub fn compress_whitespace(&mut self) -> &mut Self {
        self.ops |= StrOps::COMPRESS_WHITESPACE;
        self
    }
    /// convert v-on arg to handler key: click -> onClick
    pub fn be_handler(&mut self) -> &mut Self {
        self.ops |= StrOps::HANDLER_KEY;
        self
    }
    /// add __self suffix for self referring component
    pub fn suffix_self(&mut self) -> &mut Self {
        self.ops |= StrOps::SELF_SUFFIX;
        self
    }
    /// convert into a valid asset id
    pub fn be_asset(&mut self) -> &mut Self {
        self.ops |= StrOps::VALID_ASSET;
        self
    }
    pub fn into_string(self) -> String {
        todo!()
    }
}

impl<'a> Deref for VStr<'a> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.raw
    }
}
