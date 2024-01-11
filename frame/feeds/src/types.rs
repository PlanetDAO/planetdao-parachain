
use frame_support::pallet_prelude::*;


use super::*;



#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FEED {
	pub feed_id: u32,

    pub date: String,
    pub Type: String,
    pub data: String
}

impl FEED {
    pub fn new(
		feed_id: u32,
		date: String,
		_type: String,
		data: String
    ) -> Self {
        DAO {
            feed_id,
            date:date,
			Type:_type,
			data:data
        }
    }
}


