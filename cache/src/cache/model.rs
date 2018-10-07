use int_hash::IntHashMap;

use common::model::types::CategoryCacheIndex;
use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::LocationCacheIndex;
use common::model::types::LocationCategoryCacheIndex;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::WeekId;

pub struct CachePeriodIds {
    pub dayAfterTomorrowsVcDayId: DayId,
    pub dayB4YesterdaysVcDayId: DayId,
    pub thisMonthsVcMonthId: MonthId,
    pub thisWeeksVcWeekId: WeekId,
    pub lastMonthsVcMonthId: MonthId,
    pub lastWeeksVcWeekId: WeekId,
    pub nextMonthsVcMonthId: WeekId,
    pub nextWeeksVcWeekId: WeekId,
    pub todaysVcDayId: DayId,
    pub tomorrowsVcDayId: DayId,
    pub yesterdaysVcDayId: DayId,
}

impl CachePeriodIds {
    pub fn new() -> CachePeriodIds {
        // FIXME: implement based on current day (day of creation)
        CachePeriodIds {
            dayAfterTomorrowsVcDayId: 0,
            dayB4YesterdaysVcDayId: 0,
            thisMonthsVcMonthId: 0,
            thisWeeksVcWeekId: 0,
            lastMonthsVcMonthId: 0,
            lastWeeksVcWeekId: 0,
            nextMonthsVcMonthId: 0,
            nextWeeksVcWeekId: 0,
            todaysVcDayId: 0,
            tomorrowsVcDayId: 0,
            yesterdaysVcDayId: 0,
        };
    }
}

pub struct CategoryPeriodPollRankings {
    pub maxPollNumberBytes: u32,
    pub numPollsInPeriod: u32,
    pub voteCountsByCategoryIndex: Vec<Vec<VoteCount>>,
}

impl CategoryPeriodPollRankings {
    pub fn new(
        maxPollNumberBytes: u32,
        numPollsInPeriod: u32,
        numCategoriesInPeriod: usize,
    ) -> CategoryPeriodPollRankings {
        CategoryPeriodPollRankings {
            maxPollNumberBytes,
            numPollsInPeriod,
            voteCountsByCategoryIndex: Vec::with_capacity(numCategoriesInPeriod),
        }
    }
}

/**
 * Random access data structure needed for initial lookup of a Location+Category poll rankings.
 * Contains time period specific array index of the Location
 *      and a map (by Global Id) of the category indexes for same time period
 */
pub struct LocationPeriodIds {
    pub locationCategoryCacheIndexMap: IntHashMap<CategoryId, LocationCategoryCacheIndex>,
    pub locationCacheIndex: CategoryCacheIndex,
}

impl LocationPeriodIds {
    pub fn new(
        locationIndex: LocationCacheIndex,
        numCategories: usize,
    ) -> LocationPeriodIds {
        LocationPeriodIds {
            locationCacheIndex: locationIndex,
            locationCategoryCacheIndexMap: IntHashMap::with_capacity(numCategories),
        }
    }
}


/**
Split by timezone:
*/

/**
 *  Vote count data structure needed for looking up Poll Rankings by Vote Count
 *  Contains ranked vote counts for a particular location
 *      and an array (by time period+location specific category index) of location+category
 *          ranked vote counts
 */
pub struct LocationPollRankings {
    pub location: Vec<VoteCount>,
    pub categoryLocations: Vec<Vec<VoteCount>>,
}

/**
 *  Ordered list of latest added polls (in a future time period).
 *     Contains time ordered polls (in order of creation) for a particular location
 *         and a map/tree (by Global Category Id) of time ordered polls for location+category
 */
pub struct LocationPollPrependLists {
    // Inner vector is a page/frame (Ex: capped @ 1024) and outer vector grows
    pub location: Vec<Vec<PollId>>,
    // Custom fast no rehashing, fast insert datastructure
    // for managing an unknown number of categories in a given location
    pub categoryLocations: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
}


/**
 * Transmission details - for future poll time ordered lists a single header with the number of
 * bytes per id is acceptable.  This is because the global poll ids will have close ids (due to
 * creation order) and can be assumed to take up a roughly equal amount of bits for storage.
 * A page level byte counter can be used to pre-compute it (at insertion time).
 *
 * Note for current/past periods same type counter can be used for per timezone/period, computed
 * at creation of the period.
 */



/*
 * With 64bit Dimension Direction Sums:
 *
 * At least upper 3 bytes in sums will be free, we can use this space for
 * additional threshold counts and flags.  Also the total sum of free
 * bytes will be at least 6 to 18.  This could be used to store additional
 * information about the poll.
 *
 * For example, the positional configuration of a 3D poll can be encoded
 * into a number of configurations.  Lets assume that it would take 2 bytes
 * (64K configurations).  In the
 *
 * With 32 bit sums, they will loose precision after about 300M polls (given
 * that vote could take up to 30 spaces (2*3*5), so may need overflow bytes
 * to keep track of overflow and additional computation is needed:
 *
 * let newVal = oldVal + 24
 * if(newVal < oldVal) {
 *  overflow += 1;
 * }
 *
 * Note for pipe compression having 8u + u32 is actually faster, because
 * only 5 bytes need to be checked and serialized vs 8
 */

/**
 * Count of votes contains:
 *   PollType + Timezone - unified in a byte
 *   Id of the poll for that Timezone+period
 *   count of votes
 *   TODO: revisit poll count size if and when needed (perhaps adding an overflow bit)
 */
pub struct VoteCount {
    /**
    First 5 bits are for timezone, last 2 for for Type
    */
    pub pollTypeAndTz: u8,
    pub pollId: PollId,
    pub count: u32,
}

/*
 * Poll sums and counts for a 3 dimensional poll.
 */
pub struct ThreeDPoll {
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim2dir1Over: u8,
    dim2dir2Over: u8,
    dim3dir1Over: u8,
    dim3dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    dim2dir1Sum: u32,
    dim2dir2Sum: u32,
    dim3dir1Sum: u32,
    dim3dir2Sum: u32,
    voteCount: VoteCount,
}

/*
 * Poll sums and counts for a 2 dimensional poll.
 */
pub struct TwoDPoll {
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim2dir1Over: u8,
    dim2dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    dim2dir1Sum: u32,
    dim2dir2Sum: u32,
    voteCount: VoteCount,
}

/*
 * Poll sums and counts for a 1 dimensional poll.
 */
pub struct OneDPoll {
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    voteCount: VoteCount,
}