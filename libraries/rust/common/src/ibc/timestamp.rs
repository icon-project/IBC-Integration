use crate::ibc::prelude::*;

use core::fmt::{Display, Error as FmtError, Formatter};
use core::hash::{Hash, Hasher};
use core::num::ParseIntError;
use core::ops::{Add, Sub};
use core::str::FromStr;
use core::time::Duration;

use displaydoc::Display;
use tendermint::Time;
use time::OffsetDateTime;

pub const ZERO_DURATION: Duration = Duration::from_secs(0);

/// A newtype wrapper over `Option<Time>` to keep track of
/// IBC packet timeout.
///
/// We use an explicit `Option` type to distinguish this when converting between
/// a `u64` value and a raw timestamp. In protocol buffer, the timestamp is
/// represented as a `u64` Unix timestamp in nanoseconds, with 0 representing the absence
/// of timestamp.

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Timestamp {
    time: Option<Time>,
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for Timestamp {
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> borsh::maybestd::io::Result<()> {
        let timestamp = if let Some(time) = self.time {
            time.unix_timestamp_nanos()
        } else {
            // When the value in `Time` is `None` we give the timestamp a default value of 0
            0
        };
        borsh::BorshSerialize::serialize(&timestamp, writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for Timestamp {
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> borsh::maybestd::io::Result<Self> {
        let timestamp = u64::deserialize_reader(reader)?;
        Ok(Timestamp::from_nanoseconds(timestamp)
            .map_err(|_| borsh::maybestd::io::ErrorKind::Other)?)
    }
}

#[cfg(feature = "parity-scale-codec")]
impl parity_scale_codec::Encode for Timestamp {
    fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, writer: &mut T) {
        let timestamp = if let Some(time) = self.time {
            time.unix_timestamp_nanos()
        } else {
            // When the value in `Time` is `None` we give the timestamp a default value of 0
            0
        };

        timestamp.encode_to(writer);
    }
}
#[cfg(feature = "parity-scale-codec")]
impl parity_scale_codec::Decode for Timestamp {
    fn decode<I: parity_scale_codec::Input>(
        input: &mut I,
    ) -> Result<Self, parity_scale_codec::Error> {
        let timestamp = u64::decode(input)?;
        Timestamp::from_nanoseconds(timestamp)
            .map_err(|_| parity_scale_codec::Error::from("from nanoseconds error"))
    }
}

#[cfg(feature = "parity-scale-codec")]
impl scale_info::TypeInfo for Timestamp {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        scale_info::Type::builder()
            .path(scale_info::Path::new("Timestamp", module_path!()))
            // i128 is chosen before we represent the timestamp is nanoseconds, which is represented as a i128 by Time
            .composite(scale_info::build::Fields::named().field(|f| {
                f.ty::<Option<i128>>()
                    .name("time")
                    .type_name("Option<i128>")
            }))
    }
}

// TODO: derive when tendermint::Time supports it:
// https://github.com/informalsystems/tendermint-rs/pull/1054
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Timestamp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let odt: Option<OffsetDateTime> = self.time.map(Into::into);
        odt.hash(state);
    }
}

/// The expiry result when comparing two timestamps.
/// - If either timestamp is invalid (0), the result is `InvalidTimestamp`.
/// - If the left timestamp is strictly after the right timestamp, the result is `Expired`.
/// - Otherwise, the result is `NotExpired`.
///
/// User of this result may want to determine whether error should be raised,
/// when either of the timestamp being compared is invalid.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Expiry {
    Expired,
    NotExpired,
    InvalidTimestamp,
}

impl Timestamp {
    /// The IBC protocol represents timestamps as u64 Unix
    /// timestamps in nanoseconds.
    ///
    /// A protocol value of 0 indicates that the timestamp
    /// is not set. In this case, our domain type takes the
    /// value of None.
    ///
    pub fn from_nanoseconds(nanoseconds: u64) -> Result<Timestamp, ParseTimestampError> {
        if nanoseconds == 0 {
            Ok(Timestamp { time: None })
        } else {
            // As the `u64` representation can only represent times up to
            // about year 2554, there is no risk of overflowing `Time`
            // or `OffsetDateTime`.
            let ts = OffsetDateTime::from_unix_timestamp_nanos(nanoseconds as i128)
                .unwrap()
                .try_into()
                .unwrap();
            Ok(Timestamp { time: Some(ts) })
        }
    }

    /// Returns a `Timestamp` representation of the current time.
    #[cfg(feature = "std")]
    pub fn now() -> Timestamp {
        Time::now().into()
    }

    /// Returns a `Timestamp` representation of a timestamp not being set.
    pub fn none() -> Self {
        Timestamp { time: None }
    }

    /// Computes the duration difference of another `Timestamp` from the current one.
    /// Returns the difference in time as an [`core::time::Duration`].
    /// Returns `None` if the other `Timestamp` is more advanced
    /// than the current or if either of the `Timestamp`s is not set.
    pub fn duration_since(&self, other: &Timestamp) -> Option<Duration> {
        match (self.time, other.time) {
            (Some(time1), Some(time2)) => time1.duration_since(time2).ok(),
            _ => None,
        }
    }

    /// Convert a `Timestamp` to `u64` value in nanoseconds. If no timestamp
    /// is set, the result is 0.
    ///
    #[deprecated(since = "0.9.1", note = "use `nanoseconds` instead")]
    pub fn as_nanoseconds(&self) -> u64 {
        (*self).nanoseconds()
    }

    /// Convert a `Timestamp` to `u64` value in nanoseconds. If no timestamp
    /// is set, the result is 0.
    /// ```
    /// use common::ibc::timestamp::Timestamp;
    ///
    /// let max = u64::MAX;
    /// let tx = Timestamp::from_nanoseconds(max).unwrap();
    /// let utx = tx.nanoseconds();
    /// assert_eq!(utx, max);
    /// let min = u64::MIN;
    /// let ti = Timestamp::from_nanoseconds(min).unwrap();
    /// let uti = ti.nanoseconds();
    /// assert_eq!(uti, min);
    /// let tz = Timestamp::default();
    /// let utz = tz.nanoseconds();
    /// assert_eq!(utz, 0);
    /// ```
    pub fn nanoseconds(self) -> u64 {
        self.time.map_or(0, |time| {
            let t: OffsetDateTime = time.into();
            let s = t.unix_timestamp_nanos();
            assert!(s >= 0, "time {time:?} has negative `.timestamp()`");
            s.try_into().unwrap()
        })
    }

    /// Convert a `Timestamp` to an optional [`OffsetDateTime`]
    pub fn into_datetime(self) -> Option<OffsetDateTime> {
        self.time.map(Into::into)
    }

    /// Convert a `Timestamp` to an optional [`tendermint::Time`]
    pub fn into_tm_time(self) -> Option<Time> {
        self.time
    }

    /// Checks whether the timestamp has expired when compared to the
    /// `other` timestamp. Returns an [`Expiry`] result.
    pub fn check_expiry(&self, other: &Timestamp) -> Expiry {
        match (self.time, other.time) {
            (Some(time1), Some(time2)) => {
                if time1 > time2 {
                    Expiry::Expired
                } else {
                    Expiry::NotExpired
                }
            }
            _ => Expiry::InvalidTimestamp,
        }
    }

    /// Checks whether the current timestamp is strictly more advanced
    /// than the `other` timestamp. Return true if so, and false
    /// otherwise.
    pub fn after(&self, other: &Timestamp) -> bool {
        match (self.time, other.time) {
            (Some(time1), Some(time2)) => time1 > time2,
            _ => false,
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "Timestamp({})",
            self.time
                .map_or("NoTimestamp".to_string(), |time| time.to_rfc3339())
        )
    }
}

#[derive(Debug, Display)]
pub enum TimestampOverflowError {
    /// Timestamp overflow when modifying with duration
    TimestampOverflow,
}

#[cfg(feature = "std")]
impl std::error::Error for TimestampOverflowError {}

impl Add<Duration> for Timestamp {
    type Output = Result<Timestamp, TimestampOverflowError>;

    fn add(self, duration: Duration) -> Result<Timestamp, TimestampOverflowError> {
        match self.time {
            Some(time) => {
                let time =
                    (time + duration).map_err(|_| TimestampOverflowError::TimestampOverflow)?;
                Ok(Timestamp { time: Some(time) })
            }
            None => Ok(self),
        }
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Result<Timestamp, TimestampOverflowError>;

    fn sub(self, duration: Duration) -> Result<Timestamp, TimestampOverflowError> {
        match self.time {
            Some(time) => {
                let time =
                    (time - duration).map_err(|_| TimestampOverflowError::TimestampOverflow)?;
                Ok(Timestamp { time: Some(time) })
            }
            None => Ok(self),
        }
    }
}

#[derive(Debug, Display)]
pub enum ParseTimestampError {
    /// parsing u64 integer from string error: `{0}`
    ParseInt(ParseIntError),
}

#[cfg(feature = "std")]
impl std::error::Error for ParseTimestampError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            ParseTimestampError::ParseInt(e) => Some(e),
        }
    }
}

impl FromStr for Timestamp {
    type Err = ParseTimestampError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nanoseconds = u64::from_str(s).map_err(ParseTimestampError::ParseInt)?;

        Timestamp::from_nanoseconds(nanoseconds)
    }
}

impl From<Time> for Timestamp {
    fn from(tendermint_time: Time) -> Timestamp {
        Timestamp {
            time: Some(tendermint_time),
        }
    }
}
