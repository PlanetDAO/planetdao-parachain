#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
use scale_info::prelude::string::String;
pub use weights::*;
// pub mod rpc;
pub mod types;


#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use types::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn _ideas_ids)]
	pub type _ideas_ids<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn _smart_contract_ids)]
	pub type _smart_contract_ids<T> = StorageValue<_, u32>;

	
	/// Get the details of a Smart Contract by its' id.
	#[pallet::storage]
	#[pallet::getter(fn smart_contract_by_id)]
	pub type SmartContractsById<T: Config> = StorageMap<_, Twox64Concat, u32, SmartContract>;


	
	/// Get the details of a ideas by its' id.
	#[pallet::storage]
	#[pallet::getter(fn ideas_by_id)]
	pub type IdeasById<T: Config> = StorageMap<_, Twox64Concat, u32, IDEAS>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_ideas())]
		pub fn create_ideas(
			origin: OriginFor<T>,
			_ideas_uri: String,
			_goal_id: String,
			_dao_id: String,
			_user_id: String,
			_feed: String,
		) -> DispatchResult {

			let mut new_id = 0;
			match <_ideas_ids<T>>::try_get(){
				Ok(old)=>{
					new_id = old;
					<_ideas_ids<T>>::put(new_id + 1);
				}
				Err(_)=>{<_ideas_ids<T>>::put(1);}
			}

			let new_ideas = &mut  IDEAS {
				id: new_id,
				dao_id: _dao_id,
				goal_id: _goal_id,
				ideas_uri: _ideas_uri,
				donation: 0,
			} ;

			IdeasById::<T>::insert(new_id, new_ideas);





			Ok(())
		}



	}
}
