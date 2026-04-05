#![forbid(unsafe_code)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(ambiguous_glob_reexports)]

use ::core::convert::TryInto;

pub mod bool_enum;
pub mod cancel_restrictions;
pub mod contingency_type;
pub mod cxl_rej_response_to;
pub mod exec_inst;
pub mod exec_type;
pub mod execution_report_ack_codec;
pub mod execution_report_codec;
pub mod execution_report_type;
pub mod group_size_16_encoding_codec;
pub mod group_size_32_encoding_codec;
pub mod group_size_8_encoding_codec;
pub mod heartbeat_codec;
pub mod instrument_list_codec;
pub mod instrument_list_request_codec;
pub mod instrument_list_request_type;
pub mod limit_query_codec;
pub mod limit_reset_interval_resolution;
pub mod limit_response_codec;
pub mod limit_type;
pub mod list_order_status;
pub mod list_reject_reason;
pub mod list_status_codec;
pub mod list_status_type;
pub mod list_trigger_action;
pub mod list_trigger_type;
pub mod logon_ack_codec;
pub mod logon_codec;
pub mod logout_codec;
pub mod market_data_incremental_book_ticker_codec;
pub mod market_data_incremental_depth_codec;
pub mod market_data_incremental_trade_codec;
pub mod market_data_request_codec;
pub mod market_data_request_reject_codec;
pub mod market_data_snapshot_codec;
pub mod mass_cancel_reject_reason;
pub mod mass_cancel_request_type;
pub mod mass_cancel_response;
pub mod match_type;
pub mod md_entry_type;
pub mod md_req_rej_reason;
pub mod message_handling;
pub mod message_header_codec;
pub mod misc_fee_type;
pub mod new_order_list_codec;
pub mod new_order_single_codec;
pub mod news_codec;
pub mod optional_var_string_8_codec;
pub mod optional_var_string_codec;
pub mod ord_rej_reason;
pub mod ord_status;
pub mod ord_type;
pub mod order_amend_keep_priority_request_codec;
pub mod order_amend_reject_codec;
pub mod order_cancel_reject_codec;
pub mod order_cancel_request_and_new_order_single_codec;
pub mod order_cancel_request_and_new_order_single_mode;
pub mod order_cancel_request_codec;
pub mod order_capacity;
pub mod order_mass_cancel_report_codec;
pub mod order_mass_cancel_request_codec;
pub mod order_rate_limit_exceeded_mode;
pub mod peg_move_type;
pub mod peg_offset_type;
pub mod peg_price_type;
pub mod reject_codec;
pub mod response_mode;
pub mod self_trade_prevention_mode;
pub mod session_reject_reason;
pub mod side;
pub mod small_group_size_16_encoding_codec;
pub mod small_group_size_8_encoding_codec;
pub mod subscription_request_type;
pub mod test_request_codec;
pub mod time_in_force;
pub mod trigger_action;
pub mod trigger_price_direction;
pub mod trigger_price_type;
pub mod trigger_type;
pub mod var_string_8_codec;
pub mod var_string_codec;
pub mod working_floor;

pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 0;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub type SbeResult<T> = core::result::Result<T, SbeErr>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SbeErr {
    ParentNotSet,
}
impl core::fmt::Display for SbeErr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for SbeErr {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait Writer<'a>: Sized {
    fn get_buf_mut(&mut self) -> &mut WriteBuf<'a>;
}

pub trait Encoder<'a>: Writer<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
    fn nullify_optional_fields(&mut self) -> &mut Self {
        self
    }
}

pub trait ActingVersion {
    fn acting_version(&self) -> u16;
}

pub trait Reader<'a>: Sized {
    fn get_buf(&self) -> &ReadBuf<'a>;
}

pub trait Decoder<'a>: Reader<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadBuf<'a> {
    data: &'a [u8],
}
impl<'a> Reader<'a> for ReadBuf<'a> {
    #[inline]
    fn get_buf(&self) -> &ReadBuf<'a> {
        self
    }
}
#[allow(dead_code)]
impl<'a> ReadBuf<'a> {
    #[inline]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub(crate) fn get_bytes_at<const N: usize>(slice: &[u8], index: usize) -> [u8; N] {
        slice[index..index + N]
            .try_into()
            .expect("slice with incorrect length")
    }

    #[inline]
    pub const fn get_u8_at(&self, index: usize) -> u8 {
        self.data[index]
    }

    #[inline]
    pub fn get_i8_at(&self, index: usize) -> i8 {
        i8::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i16_at(&self, index: usize) -> i16 {
        i16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i32_at(&self, index: usize) -> i32 {
        i32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i64_at(&self, index: usize) -> i64 {
        i64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u16_at(&self, index: usize) -> u16 {
        u16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u32_at(&self, index: usize) -> u32 {
        u32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u64_at(&self, index: usize) -> u64 {
        u64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f32_at(&self, index: usize) -> f32 {
        f32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f64_at(&self, index: usize) -> f64 {
        f64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_slice_at(&self, index: usize, len: usize) -> &[u8] {
        &self.data[index..index + len]
    }
}

#[derive(Debug, Default)]
pub struct WriteBuf<'a> {
    data: &'a mut [u8],
}
impl<'a> WriteBuf<'a> {
    pub const fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn put_bytes_at<const COUNT: usize>(&mut self, index: usize, bytes: &[u8; COUNT]) -> usize {
        self.data[index..index + COUNT].copy_from_slice(bytes);
        COUNT
    }

    #[inline]
    pub const fn put_u8_at(&mut self, index: usize, value: u8) {
        self.data[index] = value;
    }

    #[inline]
    pub fn put_i8_at(&mut self, index: usize, value: i8) {
        self.put_bytes_at(index, &i8::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i16_at(&mut self, index: usize, value: i16) {
        self.put_bytes_at(index, &i16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i32_at(&mut self, index: usize, value: i32) {
        self.put_bytes_at(index, &i32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i64_at(&mut self, index: usize, value: i64) {
        self.put_bytes_at(index, &i64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u16_at(&mut self, index: usize, value: u16) {
        self.put_bytes_at(index, &u16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u32_at(&mut self, index: usize, value: u32) {
        self.put_bytes_at(index, &u32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u64_at(&mut self, index: usize, value: u64) {
        self.put_bytes_at(index, &u64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f32_at(&mut self, index: usize, value: f32) {
        self.put_bytes_at(index, &f32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f64_at(&mut self, index: usize, value: f64) {
        self.put_bytes_at(index, &f64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_slice_at(&mut self, index: usize, src: &[u8]) -> usize {
        let len = src.len();
        let dest = self.data.split_at_mut(index).1.split_at_mut(len).0;
        dest.clone_from_slice(src);
        len
    }
}
impl<'a> From<&'a mut WriteBuf<'a>> for &'a mut [u8] {
    #[inline]
    fn from(buf: &'a mut WriteBuf<'a>) -> &'a mut [u8] {
        buf.data
    }
}
