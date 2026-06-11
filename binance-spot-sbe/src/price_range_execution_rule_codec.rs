use crate::*;

pub use decoder::PriceRangeExecutionRuleDecoder;
pub use encoder::PriceRangeExecutionRuleEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 33;
pub const SBE_TEMPLATE_ID: u16 = 22;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct PriceRangeExecutionRuleEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for PriceRangeExecutionRuleEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for PriceRangeExecutionRuleEncoder<'a> {
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
            self.bid_limit_mult_up_opt(None);
            self.bid_limit_mult_down_opt(None);
            self.ask_limit_mult_up_opt(None);
            self.ask_limit_mult_down_opt(None);
            self
        }
    }

    impl<'a> PriceRangeExecutionRuleEncoder<'a> {
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

        // skipping CONSTANT enum 'ruleType'

        /// primitive field 'multiplierExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn multiplier_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'bidLimitMultUp'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn bid_limit_mult_up(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'bidLimitMultUp'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn bid_limit_mult_up_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.bid_limit_mult_up(value),
                None => self.bid_limit_mult_up(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'bidLimitMultDown'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 9
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn bid_limit_mult_down(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 9;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'bidLimitMultDown'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 9
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn bid_limit_mult_down_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.bid_limit_mult_down(value),
                None => self.bid_limit_mult_down(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'askLimitMultUp'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 17
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn ask_limit_mult_up(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 17;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'askLimitMultUp'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 17
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn ask_limit_mult_up_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.ask_limit_mult_up(value),
                None => self.ask_limit_mult_up(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'askLimitMultDown'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 25
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn ask_limit_mult_down(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 25;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'askLimitMultDown'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 25
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn ask_limit_mult_down_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.ask_limit_mult_down(value),
                None => self.ask_limit_mult_down(-9223372036854775808_i64),
            };
            self
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct PriceRangeExecutionRuleDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for PriceRangeExecutionRuleDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for PriceRangeExecutionRuleDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for PriceRangeExecutionRuleDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PriceRangeExecutionRuleDecoder<'a> {
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

        /// CONSTANT enum
        #[inline]
        pub fn rule_type(&self) -> execution_rule_type::ExecutionRuleType {
            execution_rule_type::ExecutionRuleType::PriceRange
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn multiplier_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn bid_limit_mult_up(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 1);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn bid_limit_mult_down(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 9);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn ask_limit_mult_up(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 17);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn ask_limit_mult_down(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 25);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }
    }
} // end decoder
