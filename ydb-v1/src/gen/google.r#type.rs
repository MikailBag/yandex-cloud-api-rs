/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are \[google.type.Date\](<https://github.com/googleapis/googleapis/blob/master/google/type/date.proto>) and \[google.protobuf.Timestamp\](<https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/timestamp.proto>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    #[prost(int32, tag="1")]
    pub hours: i32,
    /// Minutes of hour of day. Must be from 0 to 59.
    #[prost(int32, tag="2")]
    pub minutes: i32,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    #[prost(int32, tag="3")]
    pub seconds: i32,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    #[prost(int32, tag="4")]
    pub nanos: i32,
}
/// Represents a day of week.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DayOfWeek {
    /// The unspecified day-of-week.
    Unspecified = 0,
    /// The day-of-week of Monday.
    Monday = 1,
    /// The day-of-week of Tuesday.
    Tuesday = 2,
    /// The day-of-week of Wednesday.
    Wednesday = 3,
    /// The day-of-week of Thursday.
    Thursday = 4,
    /// The day-of-week of Friday.
    Friday = 5,
    /// The day-of-week of Saturday.
    Saturday = 6,
    /// The day-of-week of Sunday.
    Sunday = 7,
}
