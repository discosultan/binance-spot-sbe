use crate::*;

pub use decoder::ExecutionReportEventDecoder;
pub use encoder::ExecutionReportEventEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 281;
pub const SBE_TEMPLATE_ID: u16 = 603;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct ExecutionReportEventEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for ExecutionReportEventEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for ExecutionReportEventEncoder<'a> {
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
            self.order_creation_time_opt(None);
            self.working_time_opt(None);
            self.order_list_id_opt(None);
            self.trade_id_opt(None);
            self.alloc_id_opt(None);
            self.trailing_delta_opt(None);
            self.trailing_time_opt(None);
            self.trade_group_id_opt(None);
            self.last_prevented_qty_opt(None);
            self.prevented_match_id_opt(None);
            self.prevented_execution_qty_opt(None);
            self.prevented_execution_price_opt(None);
            self.prevented_execution_quote_qty_opt(None);
            self.strategy_type_opt(None);
            self.strategy_id_opt(None);
            self.counter_order_id_opt(None);
            self.subscription_id_opt(None);
            self.peg_price_type_opt(None);
            self.peg_offset_type_opt(None);
            self.peg_offset_value_opt(None);
            self.pegged_price_opt(None);
            self
        }
    }

    impl<'a> ExecutionReportEventEncoder<'a> {
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

        /// primitive field 'eventTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn event_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'transactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn transact_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'priceExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn price_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset + 16;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'qtyExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 17
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn qty_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset + 17;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'commissionExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 18
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn commission_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset + 18;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'orderCreationTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 19
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_creation_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 19;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'orderCreationTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 19
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn order_creation_time_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.order_creation_time(value),
                None => self.order_creation_time(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'workingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 27
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn working_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 27;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'workingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 27
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn working_time_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.working_time(value),
                None => self.working_time(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'orderId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 35
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 35;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'orderListId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 43
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_list_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 43;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'orderListId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 43
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn order_list_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.order_list_id(value),
                None => self.order_list_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'origQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 51
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn orig_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 51;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'price'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 59
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 59;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'origQuoteOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 67
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn orig_quote_order_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 67;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'icebergQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 75
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn iceberg_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 75;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'stopPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 83
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn stop_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 83;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&mut self, value: order_type::OrderType) -> &mut Self {
            let offset = self.offset + 91;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: order_side::OrderSide) -> &mut Self {
            let offset = self.offset + 92;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: time_in_force::TimeInForce) -> &mut Self {
            let offset = self.offset + 93;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn execution_type(&mut self, value: execution_type::ExecutionType) -> &mut Self {
            let offset = self.offset + 94;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_status(&mut self, value: order_status::OrderStatus) -> &mut Self {
            let offset = self.offset + 95;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// primitive field 'tradeId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 96
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trade_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 96;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'tradeId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 96
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trade_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.trade_id(value),
                None => self.trade_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'executionId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 104
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn execution_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 104;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'executedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 112
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn executed_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 112;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'cummulativeQuoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 120
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn cummulative_quote_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 120;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'lastQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 128
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 128;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'lastPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 136
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 136;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'quoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 144
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn quote_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 144;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'commission'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 152
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn commission(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 152;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn is_working(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 160;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn is_maker(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 161;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn is_best_match(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 162;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn match_type(&mut self, value: match_type::MatchType) -> &mut Self {
            let offset = self.offset + 163;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(
            &mut self,
            value: self_trade_prevention_mode::SelfTradePreventionMode,
        ) -> &mut Self {
            let offset = self.offset + 164;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_capacity(&mut self, value: order_capacity::OrderCapacity) -> &mut Self {
            let offset = self.offset + 165;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_floor(&mut self, value: floor::Floor) -> &mut Self {
            let offset = self.offset + 166;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn used_sor(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 167;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// primitive field 'allocId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 168
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn alloc_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 168;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'allocId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 168
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn alloc_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.alloc_id(value),
                None => self.alloc_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'trailingDelta'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 176
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trailing_delta(&mut self, value: u64) -> &mut Self {
            let offset = self.offset + 176;
            self.get_buf_mut().put_u64_at(offset, value);
            self
        }

        /// optional primitive field 'trailingDelta'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 176
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trailing_delta_opt(&mut self, value: Option<u64>) -> &mut Self {
            match value {
                Some(value) => self.trailing_delta(value),
                None => self.trailing_delta(0xffffffffffffffff_u64),
            };
            self
        }

        /// primitive field 'trailingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 184
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trailing_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 184;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'trailingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 184
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trailing_time_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.trailing_time(value),
                None => self.trailing_time(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'tradeGroupId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 192
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trade_group_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 192;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'tradeGroupId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 192
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trade_group_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.trade_group_id(value),
                None => self.trade_group_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'preventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 200
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 200;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'lastPreventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 208
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_prevented_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 208;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'lastPreventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 208
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn last_prevented_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.last_prevented_qty(value),
                None => self.last_prevented_qty(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'preventedMatchId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 216
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_match_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 216;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'preventedMatchId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 216
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn prevented_match_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.prevented_match_id(value),
                None => self.prevented_match_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'preventedExecutionQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 224
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_execution_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 224;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'preventedExecutionQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 224
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn prevented_execution_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.prevented_execution_qty(value),
                None => self.prevented_execution_qty(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'preventedExecutionPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 232
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_execution_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 232;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'preventedExecutionPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 232
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn prevented_execution_price_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.prevented_execution_price(value),
                None => self.prevented_execution_price(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'preventedExecutionQuoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 240
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_execution_quote_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 240;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'preventedExecutionQuoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 240
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn prevented_execution_quote_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.prevented_execution_quote_qty(value),
                None => self.prevented_execution_quote_qty(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'strategyType'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 248
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn strategy_type(&mut self, value: i32) -> &mut Self {
            let offset = self.offset + 248;
            self.get_buf_mut().put_i32_at(offset, value);
            self
        }

        /// optional primitive field 'strategyType'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 248
        /// - encodedLength: 4
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn strategy_type_opt(&mut self, value: Option<i32>) -> &mut Self {
            match value {
                Some(value) => self.strategy_type(value),
                None => self.strategy_type(-2147483648_i32),
            };
            self
        }

        /// primitive field 'strategyId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 252
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn strategy_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 252;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'strategyId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 252
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn strategy_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.strategy_id(value),
                None => self.strategy_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'counterOrderId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 260
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn counter_order_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 260;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'counterOrderId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 260
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn counter_order_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.counter_order_id(value),
                None => self.counter_order_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'subscriptionId'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 0xffff_u16
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 268
        /// - encodedLength: 2
        /// - version: 1
        #[inline]
        pub fn subscription_id(&mut self, value: u16) -> &mut Self {
            let offset = self.offset + 268;
            self.get_buf_mut().put_u16_at(offset, value);
            self
        }

        /// optional primitive field 'subscriptionId'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 0xffff_u16
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 268
        /// - encodedLength: 2
        /// - version: 1
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn subscription_id_opt(&mut self, value: Option<u16>) -> &mut Self {
            match value {
                Some(value) => self.subscription_id(value),
                None => self.subscription_id(0xffff_u16),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_price_type(&mut self, value: peg_price_type::PegPriceType) -> &mut Self {
            let offset = self.offset + 270;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'pegPriceType'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 270
        /// - encodedLength: 1
        /// - version: 1
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn peg_price_type_opt(
            &mut self,
            value: Option<peg_price_type::PegPriceType>,
        ) -> &mut Self {
            match value {
                Some(value) => self.peg_price_type(value),
                None => self.peg_price_type(peg_price_type::PegPriceType::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_offset_type(&mut self, value: peg_offset_type::PegOffsetType) -> &mut Self {
            let offset = self.offset + 271;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'pegOffsetType'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 271
        /// - encodedLength: 1
        /// - version: 1
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn peg_offset_type_opt(
            &mut self,
            value: Option<peg_offset_type::PegOffsetType>,
        ) -> &mut Self {
            match value {
                Some(value) => self.peg_offset_type(value),
                None => self.peg_offset_type(peg_offset_type::PegOffsetType::NullVal),
            };
            self
        }

        /// primitive field 'pegOffsetValue'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 272
        /// - encodedLength: 1
        /// - version: 1
        #[inline]
        pub fn peg_offset_value(&mut self, value: u8) -> &mut Self {
            let offset = self.offset + 272;
            self.get_buf_mut().put_u8_at(offset, value);
            self
        }

        /// optional primitive field 'pegOffsetValue'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 272
        /// - encodedLength: 1
        /// - version: 1
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn peg_offset_value_opt(&mut self, value: Option<u8>) -> &mut Self {
            match value {
                Some(value) => self.peg_offset_value(value),
                None => self.peg_offset_value(0xff_u8),
            };
            self
        }

        /// primitive field 'peggedPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 273
        /// - encodedLength: 8
        /// - version: 1
        #[inline]
        pub fn pegged_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 273;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'peggedPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 273
        /// - encodedLength: 8
        /// - version: 1
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn pegged_price_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.pegged_price(value),
                None => self.pegged_price(-9223372036854775808_i64),
            };
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
        pub fn client_order_id(&mut self, value: &str) -> &mut Self {
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
        pub fn orig_client_order_id(&mut self, value: &str) -> &mut Self {
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
        pub fn commission_asset(&mut self, value: &str) -> &mut Self {
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
        pub fn reject_reason(&mut self, value: &str) -> &mut Self {
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
        pub fn counter_symbol(&mut self, value: &str) -> &mut Self {
            let limit = self.get_limit();
            let data_length = value.len().min((u8::MAX - 1) as usize);
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut()
                .put_slice_at(limit + 1, &value[0..data_length].as_bytes());
            self
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct ExecutionReportEventDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for ExecutionReportEventDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for ExecutionReportEventDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for ExecutionReportEventDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> ExecutionReportEventDecoder<'a> {
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn event_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn transact_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 16)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn qty_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 17)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn commission_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 18)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn order_creation_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 19);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn working_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 27);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 35)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn order_list_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 43);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn orig_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 51)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 59)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn orig_quote_order_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 67)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn iceberg_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 75)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn stop_price(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 83)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&self) -> order_type::OrderType {
            self.get_buf().get_u8_at(self.offset + 91).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> order_side::OrderSide {
            self.get_buf().get_u8_at(self.offset + 92).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> time_in_force::TimeInForce {
            self.get_buf().get_u8_at(self.offset + 93).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn execution_type(&self) -> execution_type::ExecutionType {
            self.get_buf().get_u8_at(self.offset + 94).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_status(&self) -> order_status::OrderStatus {
            self.get_buf().get_u8_at(self.offset + 95).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trade_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 96);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn execution_id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 104)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn executed_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 112)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn cummulative_quote_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 120)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 128)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_price(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 136)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn quote_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 144)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn commission(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 152)
        }

        /// REQUIRED enum
        #[inline]
        pub fn is_working(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 160).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn is_maker(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 161).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn is_best_match(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 162).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn match_type(&self) -> match_type::MatchType {
            self.get_buf().get_u8_at(self.offset + 163).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(
            &self,
        ) -> self_trade_prevention_mode::SelfTradePreventionMode {
            self.get_buf().get_u8_at(self.offset + 164).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_capacity(&self) -> order_capacity::OrderCapacity {
            self.get_buf().get_u8_at(self.offset + 165).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_floor(&self) -> floor::Floor {
            self.get_buf().get_u8_at(self.offset + 166).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn used_sor(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 167).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn alloc_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 168);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xffffffffffffffff_u64' }
        #[inline]
        pub fn trailing_delta(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 176);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trailing_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 184);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trade_group_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 192);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn prevented_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 200)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn last_prevented_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 208);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_match_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 216);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_execution_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 224);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_execution_price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 232);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_execution_quote_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 240);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648_i32' }
        #[inline]
        pub fn strategy_type(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 248);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn strategy_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 252);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn counter_order_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 260);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xffff_u16' }
        #[inline]
        pub fn subscription_id(&self) -> Option<u16> {
            if self.acting_version() < 1 {
                return None;
            }

            let value = self.get_buf().get_u16_at(self.offset + 268);
            if value == 0xffff_u16 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_price_type(&self) -> peg_price_type::PegPriceType {
            if self.acting_version() < 1 {
                return peg_price_type::PegPriceType::default();
            }

            self.get_buf().get_u8_at(self.offset + 270).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_offset_type(&self) -> peg_offset_type::PegOffsetType {
            if self.acting_version() < 1 {
                return peg_offset_type::PegOffsetType::default();
            }

            self.get_buf().get_u8_at(self.offset + 271).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xff_u8' }
        #[inline]
        pub fn peg_offset_value(&self) -> Option<u8> {
            if self.acting_version() < 1 {
                return None;
            }

            let value = self.get_buf().get_u8_at(self.offset + 272);
            if value == 0xff_u8 { None } else { Some(value) }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn pegged_price(&self) -> Option<i64> {
            if self.acting_version() < 1 {
                return None;
            }

            let value = self.get_buf().get_i64_at(self.offset + 273);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
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
        pub fn client_order_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn client_order_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn orig_client_order_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn orig_client_order_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn commission_asset_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn commission_asset_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn reject_reason_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn reject_reason_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn counter_symbol_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn counter_symbol_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }
} // end decoder
