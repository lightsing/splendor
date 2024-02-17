#![allow(improper_ctypes_definitions)]

use abi_stable::{
    declare_root_module_statics,
    library::{LibraryError, RootModule},
    package_version_strings, sabi_trait,
    sabi_types::VersionStrings,
    std_types::{RBox, RBoxError},
    DynTrait, StableAbi,
};
use splendor_core::{DropTokensAction, GameSnapshot, PlayerAction, SelectNoblesAction};
use std::path::Path;

#[sabi_trait]
pub trait PlayerActor {
    fn get_action(&self, snapshot: GameSnapshot) -> PlayerAction;

    fn drop_tokens(&self, snapshot: GameSnapshot) -> DropTokensAction;

    fn select_noble(&self, snapshot: GameSnapshot) -> SelectNoblesAction;
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ActorModRef)))]
#[sabi(missing_field(panic))]
pub struct ActorMod {
    pub new_actor: extern "C" fn() -> BoxedActor<'static>,

    pub get_action:
        extern "C" fn(actor: &BoxedActor<'static>, snapshot: GameSnapshot) -> PlayerAction,
    pub drop_tokens:
        extern "C" fn(actor: &BoxedActor<'static>, snapshot: GameSnapshot) -> DropTokensAction,
    #[sabi(last_prefix_field)]
    pub select_noble:
        extern "C" fn(actor: &BoxedActor<'static>, snapshot: GameSnapshot) -> SelectNoblesAction,
}

impl RootModule for ActorModRef {
    declare_root_module_statics! {ActorModRef}
    const BASE_NAME: &'static str = "actor_mod";
    const NAME: &'static str = "actor_mod";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_module_from_file(filename: &Path) -> Result<ActorModRef, LibraryError> {
    ActorModRef::load_from_file(filename)
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug))]
pub struct ActorInterface;

/// An alias for the trait object
pub type BoxedActor<'borr> = DynTrait<'borr, RBox<()>, ActorInterface>;

pub struct WrappedActor {
    module: ActorModRef,
    inner: BoxedActor<'static>,
}

impl WrappedActor {
    pub fn new(module: ActorModRef) -> Result<Self, RBoxError> {
        let inner = module.new_actor()();
        Ok(Self { module, inner })
    }
}

impl PlayerActor for WrappedActor {
    fn get_action(&self, snapshot: GameSnapshot) -> PlayerAction {
        self.module.get_action()(&self.inner, snapshot)
    }

    fn drop_tokens(&self, snapshot: GameSnapshot) -> DropTokensAction {
        self.module.drop_tokens()(&self.inner, snapshot)
    }

    fn select_noble(&self, snapshot: GameSnapshot) -> SelectNoblesAction {
        self.module.select_noble()(&self.inner, snapshot)
    }
}

#[macro_export]
macro_rules! declare_module {
    ($name: ident) => {
        #[abi_stable::export_root_module]
        pub fn get_library() -> $crate::ActorModRef {
            use abi_stable::prefix_type::PrefixTypeTrait;
            $crate::ActorMod {
                new_actor,
                get_action,
                drop_tokens,
                select_noble,
            }
            .leak_into_prefix()
        }

        #[abi_stable::sabi_extern_fn]
        fn new_actor() -> $crate::BoxedActor<'static> {
            abi_stable::DynTrait::from_value($name::new())
        }

        #[abi_stable::sabi_extern_fn]
        fn get_action(
            actor: &$crate::BoxedActor<'static>,
            snapshot: splendor_core::GameSnapshot,
        ) -> splendor_core::PlayerAction {
            actor.downcast_as::<$name>().unwrap().get_action(snapshot)
        }

        #[abi_stable::sabi_extern_fn]
        fn drop_tokens(
            actor: &$crate::BoxedActor<'static>,
            snapshot: splendor_core::GameSnapshot,
        ) -> splendor_core::DropTokensAction {
            actor.downcast_as::<$name>().unwrap().drop_tokens(snapshot)
        }

        #[abi_stable::sabi_extern_fn]
        fn select_noble(
            actor: &$crate::BoxedActor<'static>,
            snapshot: splendor_core::GameSnapshot,
        ) -> splendor_core::SelectNoblesAction {
            actor.downcast_as::<$name>().unwrap().select_noble(snapshot)
        }
    };
}
