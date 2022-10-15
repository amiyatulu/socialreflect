#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod extras;
mod types;

use frame_support::sp_std::{prelude::*};
// use scale_info::prelude::format;
use types::Post;

pub type PostId = u64;
pub const FIRST_POST_ID: u64 = 1;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::type_value]
    pub fn DefaultForNextPostId() -> PostId {
        FIRST_POST_ID
    }

    /// The next post id.
    #[pallet::storage]
    #[pallet::getter(fn next_post_id)]
    pub type NextPostId<T: Config> = StorageValue<_, PostId, ValueQuery, DefaultForNextPostId>;

    /// Get the details of a post by its' id.
    #[pallet::storage]
    #[pallet::getter(fn post_by_id)]
    pub type PostById<T: Config> = StorageMap<_, Twox64Concat, PostId, Post<T>>;

    /// Get the ids of all direct replies by their parent's post id.
    #[pallet::storage]
    #[pallet::getter(fn reply_ids_by_post_id)]
    pub type ReplyIdsByPostId<T: Config> =
        StorageMap<_, Twox64Concat, PostId, Vec<PostId>, ValueQuery>;

	
	/// Get the ids of all posts that have shared a given original post id.
    #[pallet::storage]
    #[pallet::getter(fn shared_post_ids_by_original_post_id)]
    pub type SharedPostIdsByOriginalPostId<T: Config> =
        StorageMap<_, Twox64Concat, PostId, Vec<PostId>, ValueQuery>;




	
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		PostCreated {
            account: T::AccountId,
            post_id: PostId,
        },
        PostUpdated {
            account: T::AccountId,
            post_id: PostId,
        },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		PostNotFound,
		NotEditor,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
            // let s = format!("The number is {}", 1);
			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10_000)]
		pub fn create_post(
			origin: OriginFor<T>,
			content: Vec<u8>,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;
			let new_post_id = Self::next_post_id();
			let new_post: Post<T> = Post::new(new_post_id, creator.clone(), content.clone());
			PostById::insert(new_post_id, new_post);
			NextPostId::<T>::mutate(|n| {
                *n += 1;
            });
			Self::deposit_event(Event::PostCreated { account: creator, post_id: new_post_id });
			Ok(())

		}

		#[pallet::weight(10_000)]
		pub fn update_post (
			origin: OriginFor<T>,
			post_id: PostId,
			update: Vec<u8>,
		) -> DispatchResult {
             let editor = ensure_signed(origin)?;
			 let mut post = Self::require_post(post_id)?;
			 ensure!(editor == post.owner, Error::<T>::NotEditor);
			 post.content = update;
			 PostById::insert(post_id, post);
			Ok(())
		}
	}
}
