use frame_support::{pallet_prelude::*};
use scale_info::TypeInfo;
use frame_support::sp_std::{vec::Vec};
use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Post<T:Config> {
    pub id: PostId,

    pub edited: bool,

    pub owner: AccountIdOf<T>,

    pub content: Vec<u8>,

    pub hidden: bool,

    pub upvotes_count: u32,

    pub downvotes_count: u32,

}
