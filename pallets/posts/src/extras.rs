use crate::*;

impl<T: Config> Post<T> {
	pub fn new(id: PostId, created_by: AccountIdOf<T>, content: Vec<u8>) -> Self {
		Post { id, edited: false, owner: created_by, content, hidden: false, upvotes_count: 0, downvotes_count: 0 }
	}
}

impl<T: Config> Pallet<T> {}
