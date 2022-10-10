use frame_support::{pallet_prelude::*};
use scale_info::TypeInfo;
use frame_support::sp_std::{vec::Vec};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Post<PostId, AccountId> {
    pub id: PostId,

    pub edited: bool,

    pub owner: AccountId,

    pub content: Vec<u8>,

    pub hidden: bool,

    pub upvotes_count: u32,

    pub downvotes_count: u32,


}
