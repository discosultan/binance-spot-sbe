use crate::*;

pub use decoder::ExecutionReportDecoder;
pub use encoder::ExecutionReportEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 280;
pub const SBE_TEMPLATE_ID: u16 = 98;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct ExecutionReportEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for ExecutionReportEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for ExecutionReportEncoder<'a> {
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
            self.exec_id_opt(None);
            self.order_id_opt(None);
            self.order_qty_opt(None);
            self.exec_inst_opt(None);
            self.price_opt(None);
            self.trigger_type_opt(None);
            self.trigger_action_opt(None);
            self.trigger_price_opt(None);
            self.trigger_price_type_opt(None);
            self.trigger_price_direction_opt(None);
            self.trigger_trailing_delta_bips_opt(None);
            self.peg_offset_value_opt(None);
            self.peg_price_type_opt(None);
            self.peg_move_type_opt(None);
            self.peg_offset_type_opt(None);
            self.pegged_price_opt(None);
            self.time_in_force_opt(None);
            self.transact_time_opt(None);
            self.order_creation_time_opt(None);
            self.max_floor_opt(None);
            self.list_id_opt(None);
            self.cash_order_qty_opt(None);
            self.target_strategy_opt(None);
            self.strategy_id_opt(None);
            self.order_capacity_opt(None);
            self.self_trade_prevention_mode_opt(None);
            self.leaves_qty_opt(None);
            self.cum_quote_qty_opt(None);
            self.aggressor_indicator_opt(None);
            self.trade_id_opt(None);
            self.last_px_opt(None);
            self.secondary_order_id_opt(None);
            self.secondary_external_account_id_opt(None);
            self.alloc_id_opt(None);
            self.match_type_opt(None);
            self.working_floor_opt(None);
            self.working_indicator_opt(None);
            self.working_time_opt(None);
            self.trailing_time_opt(None);
            self.prevented_match_id_opt(None);
            self.prevented_execution_price_opt(None);
            self.prevented_execution_qty_opt(None);
            self.trade_group_id_opt(None);
            self.counter_order_id_opt(None);
            self.prevented_qty_opt(None);
            self.last_prevented_qty_opt(None);
            self.sor_opt(None);
            self.ord_rej_reason_opt(None);
            self.error_code_opt(None);
            self
        }
    }

    impl<'a> ExecutionReportEncoder<'a> {
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

        /// primitive field 'PriceExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn price_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'QtyExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn qty_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'ExecID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn exec_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 2;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'ExecID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn exec_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.exec_id(value),
                None => self.exec_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'OrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 10;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'OrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
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

        /// primitive field 'OrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 18
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 18;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'OrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 18
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn order_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.order_qty(value),
                None => self.order_qty(-9223372036854775808_i64),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_type(&mut self, value: ord_type::OrdType) -> &mut Self {
            let offset = self.offset + 26;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: side::Side) -> &mut Self {
            let offset = self.offset + 27;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_inst(&mut self, value: exec_inst::ExecInst) -> &mut Self {
            let offset = self.offset + 28;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'ExecInst'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 28
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn exec_inst_opt(&mut self, value: Option<exec_inst::ExecInst>) -> &mut Self {
            match value {
                Some(value) => self.exec_inst(value),
                None => self.exec_inst(exec_inst::ExecInst::NullVal),
            };
            self
        }

        /// primitive field 'Price'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 29;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'Price'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn price_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.price(value),
                None => self.price(-9223372036854775808_i64),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_type(&mut self, value: trigger_type::TriggerType) -> &mut Self {
            let offset = self.offset + 37;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 37
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trigger_type_opt(&mut self, value: Option<trigger_type::TriggerType>) -> &mut Self {
            match value {
                Some(value) => self.trigger_type(value),
                None => self.trigger_type(trigger_type::TriggerType::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_action(&mut self, value: trigger_action::TriggerAction) -> &mut Self {
            let offset = self.offset + 38;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerAction'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 38
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trigger_action_opt(
            &mut self,
            value: Option<trigger_action::TriggerAction>,
        ) -> &mut Self {
            match value {
                Some(value) => self.trigger_action(value),
                None => self.trigger_action(trigger_action::TriggerAction::NullVal),
            };
            self
        }

        /// primitive field 'TriggerPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 39
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trigger_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 39;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TriggerPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 39
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trigger_price_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.trigger_price(value),
                None => self.trigger_price(-9223372036854775808_i64),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_price_type(
            &mut self,
            value: trigger_price_type::TriggerPriceType,
        ) -> &mut Self {
            let offset = self.offset + 47;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerPriceType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 47
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trigger_price_type_opt(
            &mut self,
            value: Option<trigger_price_type::TriggerPriceType>,
        ) -> &mut Self {
            match value {
                Some(value) => self.trigger_price_type(value),
                None => self.trigger_price_type(trigger_price_type::TriggerPriceType::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_price_direction(
            &mut self,
            value: trigger_price_direction::TriggerPriceDirection,
        ) -> &mut Self {
            let offset = self.offset + 48;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerPriceDirection'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 48
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trigger_price_direction_opt(
            &mut self,
            value: Option<trigger_price_direction::TriggerPriceDirection>,
        ) -> &mut Self {
            match value {
                Some(value) => self.trigger_price_direction(value),
                None => self.trigger_price_direction(
                    trigger_price_direction::TriggerPriceDirection::NullVal,
                ),
            };
            self
        }

        /// primitive field 'TriggerTrailingDeltaBips'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 49
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trigger_trailing_delta_bips(&mut self, value: u64) -> &mut Self {
            let offset = self.offset + 49;
            self.get_buf_mut().put_u64_at(offset, value);
            self
        }

        /// optional primitive field 'TriggerTrailingDeltaBips'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 49
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn trigger_trailing_delta_bips_opt(&mut self, value: Option<u64>) -> &mut Self {
            match value {
                Some(value) => self.trigger_trailing_delta_bips(value),
                None => self.trigger_trailing_delta_bips(0xffffffffffffffff_u64),
            };
            self
        }

        /// primitive field 'PegOffsetValue'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 57
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn peg_offset_value(&mut self, value: u8) -> &mut Self {
            let offset = self.offset + 57;
            self.get_buf_mut().put_u8_at(offset, value);
            self
        }

        /// optional primitive field 'PegOffsetValue'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 57
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn peg_offset_value_opt(&mut self, value: Option<u8>) -> &mut Self {
            match value {
                Some(value) => self.peg_offset_value(value),
                None => self.peg_offset_value(0xff_u8),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_price_type(&mut self, value: peg_price_type::PegPriceType) -> &mut Self {
            let offset = self.offset + 58;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'PegPriceType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 58
        /// - encodedLength: 1
        /// - version: 0
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
        pub fn peg_move_type(&mut self, value: peg_move_type::PegMoveType) -> &mut Self {
            let offset = self.offset + 59;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'PegMoveType'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 59
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn peg_move_type_opt(
            &mut self,
            value: Option<peg_move_type::PegMoveType>,
        ) -> &mut Self {
            match value {
                Some(value) => self.peg_move_type(value),
                None => self.peg_move_type(peg_move_type::PegMoveType::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_offset_type(&mut self, value: peg_offset_type::PegOffsetType) -> &mut Self {
            let offset = self.offset + 60;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'PegOffsetType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 60
        /// - encodedLength: 1
        /// - version: 0
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

        /// primitive field 'PeggedPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 61
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn pegged_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 61;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'PeggedPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 61
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn pegged_price_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.pegged_price(value),
                None => self.pegged_price(-9223372036854775808_i64),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: time_in_force::TimeInForce) -> &mut Self {
            let offset = self.offset + 69;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TimeInForce'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 69
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn time_in_force_opt(
            &mut self,
            value: Option<time_in_force::TimeInForce>,
        ) -> &mut Self {
            match value {
                Some(value) => self.time_in_force(value),
                None => self.time_in_force(time_in_force::TimeInForce::NullVal),
            };
            self
        }

        /// primitive field 'TransactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 70
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn transact_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 70;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TransactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 70
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

        /// primitive field 'OrderCreationTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 78
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_creation_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 78;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'OrderCreationTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 78
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

        /// primitive field 'MaxFloor'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 86
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn max_floor(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 86;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MaxFloor'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 86
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn max_floor_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.max_floor(value),
                None => self.max_floor(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'ListID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 94
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn list_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 94;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'ListID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 94
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

        /// primitive field 'CashOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 102
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn cash_order_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 102;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'CashOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 102
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn cash_order_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.cash_order_qty(value),
                None => self.cash_order_qty(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'TargetStrategy'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 110
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn target_strategy(&mut self, value: i32) -> &mut Self {
            let offset = self.offset + 110;
            self.get_buf_mut().put_i32_at(offset, value);
            self
        }

        /// optional primitive field 'TargetStrategy'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 110
        /// - encodedLength: 4
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn target_strategy_opt(&mut self, value: Option<i32>) -> &mut Self {
            match value {
                Some(value) => self.target_strategy(value),
                None => self.target_strategy(-2147483648_i32),
            };
            self
        }

        /// primitive field 'StrategyID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 114
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn strategy_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 114;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'StrategyID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 114
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

        /// REQUIRED enum
        #[inline]
        pub fn order_capacity(&mut self, value: order_capacity::OrderCapacity) -> &mut Self {
            let offset = self.offset + 122;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'OrderCapacity'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 122
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn order_capacity_opt(
            &mut self,
            value: Option<order_capacity::OrderCapacity>,
        ) -> &mut Self {
            match value {
                Some(value) => self.order_capacity(value),
                None => self.order_capacity(order_capacity::OrderCapacity::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(
            &mut self,
            value: self_trade_prevention_mode::SelfTradePreventionMode,
        ) -> &mut Self {
            let offset = self.offset + 123;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'SelfTradePreventionMode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 123
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn self_trade_prevention_mode_opt(
            &mut self,
            value: Option<self_trade_prevention_mode::SelfTradePreventionMode>,
        ) -> &mut Self {
            match value {
                Some(value) => self.self_trade_prevention_mode(value),
                None => self.self_trade_prevention_mode(
                    self_trade_prevention_mode::SelfTradePreventionMode::NullVal,
                ),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_type(&mut self, value: exec_type::ExecType) -> &mut Self {
            let offset = self.offset + 124;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// primitive field 'CumQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 125
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn cum_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 125;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'LeavesQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 133
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn leaves_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 133;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'LeavesQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 133
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn leaves_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.leaves_qty(value),
                None => self.leaves_qty(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'CumQuoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 141
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn cum_quote_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 141;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'CumQuoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 141
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn cum_quote_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.cum_quote_qty(value),
                None => self.cum_quote_qty(-9223372036854775808_i64),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn aggressor_indicator(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 149;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'AggressorIndicator'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 149
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn aggressor_indicator_opt(&mut self, value: Option<bool_enum::BoolEnum>) -> &mut Self {
            match value {
                Some(value) => self.aggressor_indicator(value),
                None => self.aggressor_indicator(bool_enum::BoolEnum::NullVal),
            };
            self
        }

        /// primitive field 'TradeID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 150
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trade_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 150;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TradeID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 150
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

        /// primitive field 'LastPx'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 158
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_px(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 158;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'LastPx'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 158
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn last_px_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.last_px(value),
                None => self.last_px(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'LastQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 166
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 166;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&mut self, value: ord_status::OrdStatus) -> &mut Self {
            let offset = self.offset + 174;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// primitive field 'SecondaryOrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 175
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn secondary_order_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 175;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'SecondaryOrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 175
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn secondary_order_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.secondary_order_id(value),
                None => self.secondary_order_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'SecondaryExternalAccountID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 183
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn secondary_external_account_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 183;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'SecondaryExternalAccountID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 183
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn secondary_external_account_id_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.secondary_external_account_id(value),
                None => self.secondary_external_account_id(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'AllocID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 191
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn alloc_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 191;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'AllocID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 191
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

        /// REQUIRED enum
        #[inline]
        pub fn match_type(&mut self, value: match_type::MatchType) -> &mut Self {
            let offset = self.offset + 199;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'MatchType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 199
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn match_type_opt(&mut self, value: Option<match_type::MatchType>) -> &mut Self {
            match value {
                Some(value) => self.match_type(value),
                None => self.match_type(match_type::MatchType::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_floor(&mut self, value: working_floor::WorkingFloor) -> &mut Self {
            let offset = self.offset + 200;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'WorkingFloor'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 200
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn working_floor_opt(
            &mut self,
            value: Option<working_floor::WorkingFloor>,
        ) -> &mut Self {
            match value {
                Some(value) => self.working_floor(value),
                None => self.working_floor(working_floor::WorkingFloor::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_indicator(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 201;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'WorkingIndicator'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 201
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn working_indicator_opt(&mut self, value: Option<bool_enum::BoolEnum>) -> &mut Self {
            match value {
                Some(value) => self.working_indicator(value),
                None => self.working_indicator(bool_enum::BoolEnum::NullVal),
            };
            self
        }

        /// primitive field 'WorkingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 202
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn working_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 202;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'WorkingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 202
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

        /// primitive field 'TrailingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 210
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trailing_time(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 210;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TrailingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: UTCTimestamp
        /// - encodedOffset: 210
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

        /// primitive field 'PreventedMatchID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 218
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_match_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 218;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'PreventedMatchID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 218
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

        /// primitive field 'PreventedExecutionPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 226
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_execution_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 226;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'PreventedExecutionPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 226
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

        /// primitive field 'PreventedExecutionQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 234
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_execution_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 234;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'PreventedExecutionQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 234
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

        /// primitive field 'TradeGroupID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 242
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trade_group_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 242;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TradeGroupID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 242
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

        /// primitive field 'CounterOrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 250
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn counter_order_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 250;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'CounterOrderID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 250
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

        /// primitive field 'PreventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 258
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn prevented_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 258;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'PreventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 258
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn prevented_qty_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.prevented_qty(value),
                None => self.prevented_qty(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'LastPreventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 266
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_prevented_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 266;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'LastPreventedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 266
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

        /// REQUIRED enum
        #[inline]
        pub fn sor(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 274;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'SOR'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 274
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn sor_opt(&mut self, value: Option<bool_enum::BoolEnum>) -> &mut Self {
            match value {
                Some(value) => self.sor(value),
                None => self.sor(bool_enum::BoolEnum::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_rej_reason(&mut self, value: ord_rej_reason::OrdRejReason) -> &mut Self {
            let offset = self.offset + 275;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'OrdRejReason'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 275
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
        /// - encodedOffset: 276
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn error_code(&mut self, value: i32) -> &mut Self {
            let offset = self.offset + 276;
            self.get_buf_mut().put_i32_at(offset, value);
            self
        }

        /// optional primitive field 'ErrorCode'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 276
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

        /// GROUP ENCODER (id=136)
        #[inline]
        pub fn misc_fees_encoder(
            self,
            count: u16,
            misc_fees_encoder: MiscFeesEncoder<Self>,
        ) -> MiscFeesEncoder<Self> {
            misc_fees_encoder.wrap(self, count)
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
        pub fn orig_cl_ord_id(&mut self, value: &str) -> &mut Self {
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
        pub fn secondary_symbol(&mut self, value: &str) -> &mut Self {
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

    #[derive(Debug, Default)]
    pub struct MiscFeesEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for MiscFeesEncoder<P>
    where
        P: Writer<'a> + Default,
    {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for MiscFeesEncoder<P>
    where
        P: Encoder<'a> + Default,
    {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent
                .as_mut()
                .expect("parent missing")
                .set_limit(limit);
        }
    }

    impl<'a, P> MiscFeesEncoder<P>
    where
        P: Encoder<'a> + Default,
    {
        #[inline]
        pub fn wrap(mut self, mut parent: P, count: u16) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent
                .get_buf_mut()
                .put_u8_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u16_at(initial_limit + 1, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub const fn block_length() -> u8 {
            10
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field 'Exponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'MiscFeeAmt'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn misc_fee_amt(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn misc_fee_type(&mut self, value: misc_fee_type::MiscFeeType) -> &mut Self {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn misc_fee_curr(&mut self, value: &str) -> &mut Self {
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
    pub struct ExecutionReportDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for ExecutionReportDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for ExecutionReportDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for ExecutionReportDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> ExecutionReportDecoder<'a> {
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
        pub fn price_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn qty_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 1)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn exec_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 2);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn order_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 10);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn order_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 18);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_type(&self) -> ord_type::OrdType {
            self.get_buf().get_u8_at(self.offset + 26).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> side::Side {
            self.get_buf().get_u8_at(self.offset + 27).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_inst(&self) -> exec_inst::ExecInst {
            self.get_buf().get_u8_at(self.offset + 28).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 29);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_type(&self) -> trigger_type::TriggerType {
            self.get_buf().get_u8_at(self.offset + 37).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_action(&self) -> trigger_action::TriggerAction {
            self.get_buf().get_u8_at(self.offset + 38).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trigger_price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 39);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_price_type(&self) -> trigger_price_type::TriggerPriceType {
            self.get_buf().get_u8_at(self.offset + 47).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_price_direction(&self) -> trigger_price_direction::TriggerPriceDirection {
            self.get_buf().get_u8_at(self.offset + 48).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xffffffffffffffff_u64' }
        #[inline]
        pub fn trigger_trailing_delta_bips(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 49);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xff_u8' }
        #[inline]
        pub fn peg_offset_value(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 57);
            if value == 0xff_u8 { None } else { Some(value) }
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_price_type(&self) -> peg_price_type::PegPriceType {
            self.get_buf().get_u8_at(self.offset + 58).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_move_type(&self) -> peg_move_type::PegMoveType {
            self.get_buf().get_u8_at(self.offset + 59).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_offset_type(&self) -> peg_offset_type::PegOffsetType {
            self.get_buf().get_u8_at(self.offset + 60).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn pegged_price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 61);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> time_in_force::TimeInForce {
            self.get_buf().get_u8_at(self.offset + 69).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn transact_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 70);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn order_creation_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 78);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn max_floor(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 86);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn list_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 94);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn cash_order_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 102);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648_i32' }
        #[inline]
        pub fn target_strategy(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 110);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn strategy_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 114);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_capacity(&self) -> order_capacity::OrderCapacity {
            self.get_buf().get_u8_at(self.offset + 122).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(
            &self,
        ) -> self_trade_prevention_mode::SelfTradePreventionMode {
            self.get_buf().get_u8_at(self.offset + 123).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_type(&self) -> exec_type::ExecType {
            self.get_buf().get_u8_at(self.offset + 124).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn cum_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 125)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn leaves_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 133);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn cum_quote_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 141);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn aggressor_indicator(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 149).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trade_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 150);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn last_px(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 158);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 166)
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&self) -> ord_status::OrdStatus {
            self.get_buf().get_u8_at(self.offset + 174).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn secondary_order_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 175);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn secondary_external_account_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 183);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn alloc_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 191);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn match_type(&self) -> match_type::MatchType {
            self.get_buf().get_u8_at(self.offset + 199).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_floor(&self) -> working_floor::WorkingFloor {
            self.get_buf().get_u8_at(self.offset + 200).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_indicator(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 201).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn working_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 202);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trailing_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 210);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_match_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 218);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_execution_price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 226);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_execution_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 234);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trade_group_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 242);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn counter_order_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 250);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn prevented_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 258);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn last_prevented_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 266);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn sor(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 274).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_rej_reason(&self) -> ord_rej_reason::OrdRejReason {
            self.get_buf().get_u8_at(self.offset + 275).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648_i32' }
        #[inline]
        pub fn error_code(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 276);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// GROUP DECODER (id=136)
        #[inline]
        pub fn misc_fees_decoder(self) -> MiscFeesDecoder<Self> {
            MiscFeesDecoder::default().wrap(self)
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
        pub fn orig_cl_ord_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn orig_cl_ord_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
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
        pub fn secondary_symbol_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn secondary_symbol_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
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

    #[derive(Debug, Default)]
    pub struct MiscFeesDecoder<P> {
        parent: Option<P>,
        block_length: u8,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for MiscFeesDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for MiscFeesDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for MiscFeesDecoder<P>
    where
        P: Decoder<'a> + ActingVersion + Default,
    {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent
                .as_mut()
                .expect("parent missing")
                .set_limit(limit);
        }
    }

    impl<'a, P> MiscFeesDecoder<P>
    where
        P: Decoder<'a> + ActingVersion + Default,
    {
        pub fn wrap(mut self, mut parent: P) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u8_at(initial_offset);
            let count = parent.get_buf().get_u16_at(initial_offset + 1);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='MiscFees', referencedName='null', description='null', packageName='null', id=136, version=0, deprecated=0, encodedLength=10, offset=280, componentTokenCount=24, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u16 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn misc_fee_amt(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 1)
        }

        /// REQUIRED enum
        #[inline]
        pub fn misc_fee_type(&self) -> misc_fee_type::MiscFeeType {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn misc_fee_curr_decoder(&mut self) -> (usize, usize) {
            let offset = self.parent.as_ref().expect("parent missing").get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.parent
                .as_mut()
                .unwrap()
                .set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn misc_fee_curr_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }
} // end decoder
