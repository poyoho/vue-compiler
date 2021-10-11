//! This module defines a collection of flags used for Vue's runtime.
//! Currently it includes preamble helper and vnode patch flags.
//! Ideally we can make flags extensible by extracting them to trait.
//! But currently it works well enough and adding traits makes compiler
//! bloated with too many generic parameters.

use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::Serialize;

bitflags! {
    #[derive(Default)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    /// Patch flags are optimization hints generated by the compiler.
    /// when a block with dynamicChildren is encountered during diff, the algorithm
    /// enters "optimized mode". In this mode, we know that the vdom is produced by
    /// a render function generated by the compiler, so the algorithm only needs to
    /// handle updates explicitly marked by these patch flags.
    ///
    /// Check the `patchElement` function in '../../runtime-core/src/renderer.ts' to see how the
    /// flags are handled during diff.
    pub struct PatchFlag: i32 {
        /// Indicates an element with dynamic textContent (children fast path)
        const TEXT = 1;
        /// Indicates an element with dynamic class binding.
        const CLASS = 1 << 1;

        /// Indicates an element with dynamic style
        /// The compiler pre-compiles static string styles into static objects
        /// and detects and hoists inline static objects
        /// e.g. style="color: red" and :style="{ color: 'red' }" both get hoisted as
        ///   const style = { color: 'red' }
        ///   render() { return e('div', { style }) }
        const STYLE = 1 << 2;

        /// Indicates an element that has non-class/style dynamic props.
        /// Can also be on a component that has any dynamic props (includes
        /// class/style). when this flag is present, the vnode also has a dynamicProps
        /// array that contains the keys of the props that may change so the runtime
        /// can diff them faster (without having to worry about removed props)
        const PROPS = 1 << 3;

        /// Indicates an element with props with dynamic keys. When keys change, a full
        /// diff is always needed to remove the old key. This flag is mutually
        /// exclusive with CLASS, STYLE and PROPS.
        const FULL_PROPS = 1 << 4;

        /// Indicates an element with event listeners (which need to be attached during hydration)
        const HYDRATE_EVENTS = 1 << 5;

        /// Indicates a fragment whose children order doesn't change.
        const STABLE_FRAGMENT = 1 << 6;

        /// Indicates a fragment with keyed or partially keyed children
        const KEYED_FRAGMENT = 1 << 7;

        /// Indicates a fragment with unkeyed children.
        const UNKEYED_FRAGMENT = 1 << 8;

        /// Indicates an element that only needs non-props patching, e.g. ref or
        /// directives (onVnodeXXX hooks). since every patched vnode checks for refs
        /// and onVnodeXXX hooks, it simply marks the vnode so that a parent block
        /// will track it.
        const NEED_PATCH = 1 << 9;

        /// Indicates a component with dynamic slots (e.g. slot that references a v-for
        /// iterated value, or dynamic slot names).
        /// Components with this flag are always force updated.
        const DYNAMIC_SLOTS = 1 << 10;

        /// Indicates a fragment that was created only because the user has placed
        /// comments at the root level of a template. This is a dev-only flag since
        /// comments are stripped in production.
        const DEV_ROOT_FRAGMENT = 1 << 11;

        /// SPECIAL FLAGS -------------------------------------------------------------
        /// Special flags are negative integers. They are never matched against using
        /// bitwise operators (bitwise matching should only happen in branches where
        /// patchFlag > 0), and are mutually exclusive. When checking for a special
        /// flag, simply check patchFlag === FLAG.

        /// Indicates a hoisted static vnode. This is a hint for hydration to skip
        /// the entire sub tree since static content never needs to be updated.
        const HOISTED = -1;
        /// A special flag that indicates that the diffing algorithm should bail out
        /// of optimized mode. For example, on block fragments created by renderSlot()
        /// when encountering non-compiler generated slots (i.e. manually written
        /// render functions, which should always be fully diffed)
        /// OR manually cloneVNodes
        const BAIL = -2;
    }
}

/// Static level describes how much an IR node can be statically generated.
/// Higher levels implies lower levels. e.g. a node that can be stringified
/// can always be hoisted and skipped for patch.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum StaticLevel {
    NotStatic,
    CanSkipPatch,
    CanHoist,
    CanStringify,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SlotFlag {
    /// Stable slots that only reference slot props or context state. The slot
    /// can fully capture its own dependencies so when passed down the parent won't
    /// need to force the child to update.
    Stable = 1,
    /// Slots that reference scope variables (v-for or an outer slot prop), or
    /// has conditional structure (v-if, v-for). The parent will need to force
    /// the child to update because the slot does not fully capture its dependencies.
    Dynamic = 2,
    /// `<slot/>` being forwarded into a child component. Whether the parent needs
    /// to update the child is dependent on what kind of slots the parent itself
    /// received. This has to be refined at runtime, when the child's vnode
    /// is being created (in `normalizeChildren`)
    Forwarded = 3,
}

/// PreambleHelper is a collection of JavaScript imports at the head of output
/// e.g. v-for needs a list looping helper to make vdom
/// preamble helper needs collect helper when traversing template ast
/// and generates corresponding JavaScript imports in compilation output
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RuntimeHelper(u8);
use RuntimeHelper as RH;

impl RuntimeHelper {
    pub const FRAGMENT: RH = RH(0);
    pub const TELEPORT: RH = RH(1);
    pub const SUSPENSE: RH = RH(2);
    pub const KEEP_ALIVE: RH = RH(3);
    pub const BASE_TRANSITION: RH = RH(4);
    pub const OPEN_BLOCK: RH = RH(5);
    pub const CREATE_BLOCK: RH = RH(6);
    pub const CREATE_ELEMENT_BLOCK: RH = RH(7);
    pub const CREATE_VNODE: RH = RH(8);
    pub const CREATE_ELEMENT_VNODE: RH = RH(9);
    pub const CREATE_COMMENT: RH = RH(11);
    pub const CREATE_TEXT: RH = RH(12);
    pub const CREATE_STATIC: RH = RH(13);
    pub const RESOLVE_COMPONENT: RH = RH(14);
    pub const RESOLVE_DYNAMIC_COMPONENT: RH = RH(15);
    pub const RESOLVE_DIRECTIVE: RH = RH(16);
    pub const RESOLVE_FILTER: RH = RH(17);
    pub const WITH_DIRECTIVES: RH = RH(18);
    pub const RENDER_LIST: RH = RH(19);
    pub const RENDER_SLOT: RH = RH(20);
    pub const CREATE_SLOTS: RH = RH(21);
    pub const TO_DISPLAY_STRING: RH = RH(22);
    pub const MERGE_PROPS: RH = RH(23);
    pub const NORMALIZE_CLASS: RH = RH(24);
    pub const NORMALIZE_STYLE: RH = RH(25);
    pub const NORMALIZE_PROPS: RH = RH(26);
    pub const GUARD_REACTIVE_PROPS: RH = RH(27);
    pub const TO_HANDLERS: RH = RH(28);
    pub const CAMELIZE: RH = RH(29);
    pub const CAPITALIZE: RH = RH(30);
    pub const TO_HANDLER_KEY: RH = RH(31);
    pub const SET_BLOCK_TRACKING: RH = RH(32);
    pub const PUSH_SCOPE_ID: RH = RH(33);
    pub const POP_SCOPE_ID: RH = RH(34);
    pub const WITH_CTX: RH = RH(35);
    pub const UNREF: RH = RH(36);
    pub const IS_REF: RH = RH(37);
    pub const WITH_MEMO: RH = RH(38);
    pub const IS_MEMO_SAME: RH = RH(39);

    pub const INTERNAL_MAX: u8 = 40;

    pub fn helper_str(&self, map: &[&'static str]) -> &'static str {
        match *self {
            RH::FRAGMENT => "Fragment",
            RH::TELEPORT => "Teleport",
            RH::SUSPENSE => "Suspense",
            RH::KEEP_ALIVE => "KeepAlive",
            RH::BASE_TRANSITION => "BaseTransition",
            RH::OPEN_BLOCK => "openBlock",
            RH::CREATE_BLOCK => "createBlock",
            RH::CREATE_ELEMENT_BLOCK => "createElementBlock",
            RH::CREATE_VNODE => "createVNode",
            RH::CREATE_ELEMENT_VNODE => "createElementVNode",
            RH::CREATE_COMMENT => "createCommentVNode",
            RH::CREATE_TEXT => "createTextVNode",
            RH::CREATE_STATIC => "createStaticVNode",
            RH::RESOLVE_COMPONENT => "resolveComponent",
            RH::RESOLVE_DYNAMIC_COMPONENT => "resolveDynamicComponent",
            RH::RESOLVE_DIRECTIVE => "resolveDirective",
            RH::RESOLVE_FILTER => "resolveFilter",
            RH::WITH_DIRECTIVES => "withDirectives",
            RH::RENDER_LIST => "renderList",
            RH::RENDER_SLOT => "renderSlot",
            RH::CREATE_SLOTS => "createSlots",
            RH::TO_DISPLAY_STRING => "toDisplayString",
            RH::MERGE_PROPS => "mergeProps",
            RH::NORMALIZE_CLASS => "normalizeClass",
            RH::NORMALIZE_STYLE => "normalizeStyle",
            RH::NORMALIZE_PROPS => "normalizeProps",
            RH::GUARD_REACTIVE_PROPS => "guardReactiveProps",
            RH::TO_HANDLERS => "toHandlers",
            RH::CAMELIZE => "camelize",
            RH::CAPITALIZE => "capitalize",
            RH::TO_HANDLER_KEY => "toHandlerKey",
            RH::SET_BLOCK_TRACKING => "setBlockTracking",
            RH::PUSH_SCOPE_ID => "pushScopeId",
            RH::POP_SCOPE_ID => "popScopeId",
            RH::WITH_CTX => "withCtx",
            RH::UNREF => "unref",
            RH::IS_REF => "isRef",
            RH::WITH_MEMO => "withMemo",
            RH::IS_MEMO_SAME => "isMemoSame",
            RH(s) => map[(s - RH::INTERNAL_MAX) as usize],
        }
    }
}
#[cfg(feature = "serde")]
impl Serialize for RuntimeHelper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.helper_str(&[]))
    }
}

pub const HELPERS_IN_HOISTED: &[RH] = &[
    RH::CREATE_COMMENT,
    RH::CREATE_ELEMENT_VNODE,
    RH::CREATE_STATIC,
    RH::CREATE_TEXT,
    RH::CREATE_VNODE,
];

#[derive(Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HelperCollector(u64);
impl HelperCollector {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0 || (cfg!(test) && self.0 == !0)
    }
    pub fn collect(&mut self, helper: RuntimeHelper) {
        self.0 |= 1 << helper.0;
    }
    pub fn contains(&self, helper: RuntimeHelper) -> bool {
        (self.0 & (1 << helper.0)) != 0
    }
    pub fn hoist_helpers(&self) -> Self {
        let mut n = Self(0);
        for &rh in HELPERS_IN_HOISTED {
            if self.contains(rh) {
                n.collect(rh);
            }
        }
        n
    }
    // ignore missing helpers in unit testing
    #[cfg(test)]
    pub fn ignore_missing(&mut self) {
        self.0 = !0;
    }
}
pub struct HelperIter(u64);
impl Iterator for HelperIter {
    type Item = RuntimeHelper;
    fn next(&mut self) -> Option<Self::Item> {
        if cfg!(test) && self.0 == !0 {
            return None;
        }
        if self.0 == 0 {
            return None;
        }
        let r = self.0.trailing_zeros();
        self.0 ^= 1 << r;
        Some(RuntimeHelper(r as u8))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let bits = self.0.count_ones() as usize;
        (bits, Some(bits))
    }
}
impl ExactSizeIterator for HelperIter {}

impl IntoIterator for HelperCollector {
    type Item = RuntimeHelper;
    type IntoIter = HelperIter;
    fn into_iter(self) -> Self::IntoIter {
        HelperIter(self.0)
    }
}
