use crate::*;

pub use decoder::ExecutionReportAckDecoder;
pub use encoder::ExecutionReportAckEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 31;
pub const SBE_TEMPLATE_ID: u16 = 198;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct ExecutionReportAckEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for ExecutionReportAckEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for ExecutionReportAckEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }

        /// Set all optional fields to their 'null' values.
        #[inline]
        fn nullify_optional_fields(&mut self) -> &mut Self {
            self.order_id_opt(None);
            self.list_id_opt(None);
            self.transact_time_opt(None);
            self.ord_rej_reason_opt(None);
            self.error_code_opt(None);
            self
        }
    }

    impl<'a> ExecutionReportAckEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub const fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive field 'OrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'OrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn order_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.order_id(value),
                None => self.order_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'ListID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn list_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'ListID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn list_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.list_id(value),
                None => self.list_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'TransactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 16
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn transact_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 16;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TransactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 16
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn transact_time_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.transact_time(value),
                None => self.transact_time(-9223372036854775808_i64),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_type(&mut self, value: exec_type::ExecType) -> &mut Self {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&mut self, value: ord_status::OrdStatus) -> &mut Self {
            let offset = self.offset + 25;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_rej_reason(&mut self, value: ord_rej_reason::OrdRejReason) -> &mut Self {
            let offset = self.offset + 26;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'OrdRejReason'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 26
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn ord_rej_reason_opt(
            &mut self,
            value: Option<ord_rej_reason::OrdRejReason>,
        ) -> &mut Self {
            match value {
                Some(value) => self.ord_rej_reason(value),
                None => self.ord_rej_reason(ord_rej_reason::OrdRejReason::NullVal),
            };
            self
        }

        /// primitive field 'ErrorCode'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 27
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn error_code(&mut self, value: i32) -> &mut Self {
            let offset = self.offset + 27;
            self.get_buf_mut().put_i32_at(offset, value);
            self
        }

        /// optional primitive field 'ErrorCode'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 27
        /// - encodedLength: 4
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn error_code_opt(&mut self, value: Option<i32>) -> &mut Self {
            match value {
                Some(value) => self.error_code(value),
                None => self.error_code(-2147483648_i32),
            };
            self
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn cl_ord_id(&mut self, value: &str) -> &mut Self {
            let limit = self.get_limit();
            let data_length = value.len().min((u8::MAX - 1) as usize);
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut()
                .put_slice_at(limit + 1, &value[0..data_length].as_bytes());
            self
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn symbol(&mut self, value: &str) -> &mut Self {
            let limit = self.get_limit();
            let data_length = value.len().min((u8::MAX - 1) as usize);
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut()
                .put_slice_at(limit + 1, &value[0..data_length].as_bytes());
            self
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn error_text(&mut self, value: &str) -> &mut Self {
            let limit = self.get_limit();
            let data_length = value.len().min((u16::MAX - 1) as usize);
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut()
                .put_slice_at(limit + 2, &value[0..data_length].as_bytes());
            self
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct ExecutionReportAckDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for ExecutionReportAckDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for ExecutionReportAckDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for ExecutionReportAckDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> ExecutionReportAckDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub const fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>, offset: usize) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                offset + message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn order_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn list_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 8);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn transact_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 16);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_type(&self) -> exec_type::ExecType {
            self.get_buf().get_u8_at(self.offset + 24).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&self) -> ord_status::OrdStatus {
            self.get_buf().get_u8_at(self.offset + 25).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_rej_reason(&self) -> ord_rej_reason::OrdRejReason {
            self.get_buf().get_u8_at(self.offset + 26).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648_i32' }
        #[inline]
        pub fn error_code(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 27);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn cl_ord_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn cl_ord_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn symbol_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn symbol_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn error_text_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn error_text_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }
} // end decoder
