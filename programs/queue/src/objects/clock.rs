use {
    anchor_lang::{
        prelude::borsh::BorshSchema,
        prelude::*,
        solana_program::{clock::UnixTimestamp, slot_history::Slot, stake_history::Epoch},
        AnchorDeserialize,
    },
    std::convert::TryFrom,
};

/// The clock object, representing a specific moment in time recorded by a Solana cluster.
#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct ClockData {
    /// The current slot.
    pub slot: Slot,
    /// The timestamp of the first slot in this Solana epoch.
    pub epoch_start_timestamp: UnixTimestamp,
    /// The bank epoch.
    pub epoch: Epoch,
    /// The future epoch for which the leader schedule has most recently been calculated.
    pub leader_schedule_epoch: Epoch,
    /// Originally computed from genesis creation time and network time
    /// in slots (drifty); corrected using validator timestamp oracle as of
    /// timestamp_correction and timestamp_bounding features.
    pub unix_timestamp: UnixTimestamp,
}

impl From<Clock> for ClockData {
    fn from(clock: Clock) -> Self {
        ClockData {
            slot: clock.slot,
            epoch_start_timestamp: clock.epoch_start_timestamp,
            epoch: clock.epoch,
            leader_schedule_epoch: clock.leader_schedule_epoch,
            unix_timestamp: clock.unix_timestamp,
        }
    }
}

impl From<&ClockData> for Clock {
    fn from(clock: &ClockData) -> Self {
        Clock {
            slot: clock.slot,
            epoch_start_timestamp: clock.epoch_start_timestamp,
            epoch: clock.epoch,
            leader_schedule_epoch: clock.leader_schedule_epoch,
            unix_timestamp: clock.unix_timestamp,
        }
    }
}

impl TryFrom<Vec<u8>> for ClockData {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(
            borsh::try_from_slice_with_schema::<ClockData>(data.as_slice())
                .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?,
        )
    }
}
