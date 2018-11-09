use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::timezone::NUM_TIMEZONES;
use common::model::timezone::NUM_TIMEZONES_WITH_GLOBAL_CATEGORY;
use common::model::types::CategoryCacheIndex;
use common::model::types::CategoryId;
use common::model::types::LocationId;
use common::model::types::PollId;

use super::category_index_map::CategoryIndexMap;
use super::category_poll_rankings::CategoryPollRankings;
use super::location_category_index_map::LocationCategoryIndexMap;
use super::locations_poll_rankings::LocationsPollRankings;
use super::location_index_map::LocationIndexMap;
use super::model::CachePeriodIds;
use super::model::LocationPeriodIds;
use super::model::LocationPollPrependLists;
use super::model::LocationPollRankings;
use super::model::OneDPoll;
use super::model::ThreeDPoll;
use super::model::TwoDPoll;
use super::model::VoteCount;
use super::poll_id_byte_counts::PollIdByteCounts;
use super::polls::Polls;
use super::polls_by_category::PollsByCategory;
use super::polls_by_location::PollsByLocation;
use super::time_period_ids::TimePeriodIds;

pub struct Cache {
    /**
     * Ids of currently cached time periods, across all timezones
     */
    pub category_cache_period_ids: CachePeriodIds,
    /**
     * Ids of currently cached time periods, per timezone
     */
    pub per_timezone_cache_period_ids: [CachePeriodIds; NUM_TIMEZONES as usize],
    pub poll_id_byte_counts: PollIdByteCounts,
    pub time_period_ids: TimePeriodIds,
    // Keeps track of when a timezone is being modified
    pub time_zone_modification_flags: [bool; 38],

    pub category_index_map: CategoryIndexMap,
    pub location_category_index_map: LocationCategoryIndexMap,
    pub location_index_map: LocationIndexMap,

    pub category_poll_rankings: CategoryPollRankings,
    pub location_poll_rankings: LocationsPollRankings,

    pub polls_by_category: PollsByCategory,
    pub polls_by_location: PollsByLocation,

    pub polls_1_d: Polls<OneDPoll>,
    pub polls_2_d: Polls<TwoDPoll>,
    pub polls_3_d: Polls<ThreeDPoll>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            category_cache_period_ids: CachePeriodIds::new(),
            per_timezone_cache_period_ids: [
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new()
            ],
            poll_id_byte_counts: PollIdByteCounts::new(),
            time_period_ids: TimePeriodIds::new(),
            time_zone_modification_flags: [
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false
            ],

            category_index_map: CategoryIndexMap::new(),
            location_category_index_map: LocationCategoryIndexMap::new(),
            location_index_map: LocationIndexMap::new(),

            category_poll_rankings: CategoryPollRankings::new(),
            location_poll_rankings: LocationsPollRankings::new(),

            polls_by_category: PollsByCategory::new(),
            polls_by_location: PollsByLocation::new(),

            polls_1_d: Polls::new(),
            polls_2_d: Polls::new(),
            polls_3_d: Polls::new(),
        }
    }
}

//pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
//pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();

/*
 * Random access current poll maps, needed for count and sum increments by the voting servers.
 *    Indexed by global PollIds
 *
 *  TODO: many not be needed, assuming that timezone is known at the time of
 *  count and sum increments
 */
//pub static mut TODAY_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut TODAY_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut TODAY_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_WEEK_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_WEEK_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_WEEK_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_MONTH_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_MONTH_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_MONTH_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = HashMap::with_capacity_and_hasher(2000, b);
