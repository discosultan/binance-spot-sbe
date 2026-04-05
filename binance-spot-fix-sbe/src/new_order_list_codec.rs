use crate::*;

pub use decoder::NewOrderListDecoder;
pub use encoder::NewOrderListEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 2;
pub const SBE_TEMPLATE_ID: u16 = 100;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct NewOrderListEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for NewOrderListEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for NewOrderListEncoder<'a> {
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
            self.opo_opt(None);
            self
        }
    }

    impl<'a> NewOrderListEncoder<'a> {
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

        /// REQUIRED enum
        #[inline]
        pub fn contingency_type(&mut self, value: contingency_type::ContingencyType) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn opo(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'OPO'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn opo_opt(&mut self, value: Option<bool_enum::BoolEnum>) -> &mut Self {
            match value {
                Some(value) => self.opo(value),
                None => self.opo(bool_enum::BoolEnum::NullVal),
            };
            self
        }

        /// GROUP ENCODER (id=73)
        #[inline]
        pub fn orders_encoder(
            self,
            count: u8,
            orders_encoder: OrdersEncoder<Self>,
        ) -> OrdersEncoder<Self> {
            orders_encoder.wrap(self, count)
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn cl_list_id(&mut self, value: &str) -> &mut Self {
            let limit = self.get_limit();
            let data_length = value.len().min((u8::MAX - 1) as usize);
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut()
                .put_slice_at(limit + 1, &value[0..data_length].as_bytes());
            self
        }
    }

    #[derive(Debug, Default)]
    pub struct OrdersEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for OrdersEncoder<P>
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

    impl<'a, P> Encoder<'a> for OrdersEncoder<P>
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

        /// Set all optional fields to their 'null' values.
        #[inline]
        fn nullify_optional_fields(&mut self) -> &mut Self {
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
            self.time_in_force_opt(None);
            self.max_floor_opt(None);
            self.cash_order_qty_opt(None);
            self.target_strategy_opt(None);
            self.strategy_id_opt(None);
            self.self_trade_prevention_mode_opt(None);
            self
        }
    }

    impl<'a, P> OrdersEncoder<P>
    where
        P: Encoder<'a> + Default,
    {
        #[inline]
        pub fn wrap(mut self, mut parent: P, count: u8) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent
                .get_buf_mut()
                .put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub const fn block_length() -> u16 {
            75
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

        /// primitive field 'OrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 2;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'OrderQty'
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
            let offset = self.offset + 10;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_inst(&mut self, value: exec_inst::ExecInst) -> &mut Self {
            let offset = self.offset + 11;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'ExecInst'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 11
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
        /// - encodedOffset: 12
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 12;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'Price'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 12
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
            let offset = self.offset + 20;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 20
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
            let offset = self.offset + 21;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerAction'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 21
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
        /// - encodedOffset: 22
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trigger_price(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 22;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'TriggerPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 22
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
            let offset = self.offset + 30;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerPriceType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 30
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
            let offset = self.offset + 31;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TriggerPriceDirection'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 31
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
        /// - encodedOffset: 32
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn trigger_trailing_delta_bips(&mut self, value: u64) -> &mut Self {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u64_at(offset, value);
            self
        }

        /// optional primitive field 'TriggerTrailingDeltaBips'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 32
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
        /// - encodedOffset: 40
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn peg_offset_value(&mut self, value: u8) -> &mut Self {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u8_at(offset, value);
            self
        }

        /// optional primitive field 'PegOffsetValue'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 40
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
            let offset = self.offset + 41;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'PegPriceType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 41
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
            let offset = self.offset + 42;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'PegMoveType'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 42
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
            let offset = self.offset + 43;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'PegOffsetType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 43
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

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: side::Side) -> &mut Self {
            let offset = self.offset + 44;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: time_in_force::TimeInForce) -> &mut Self {
            let offset = self.offset + 45;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'TimeInForce'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 45
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

        /// primitive field 'MaxFloor'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 46
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn max_floor(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 46;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MaxFloor'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 46
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

        /// primitive field 'CashOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 54
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn cash_order_qty(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 54;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'CashOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 54
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
        /// - encodedOffset: 62
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn target_strategy(&mut self, value: i32) -> &mut Self {
            let offset = self.offset + 62;
            self.get_buf_mut().put_i32_at(offset, value);
            self
        }

        /// optional primitive field 'TargetStrategy'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648_i32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 62
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
        /// - encodedOffset: 66
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn strategy_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 66;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'StrategyID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 66
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
        pub fn self_trade_prevention_mode(
            &mut self,
            value: self_trade_prevention_mode::SelfTradePreventionMode,
        ) -> &mut Self {
            let offset = self.offset + 74;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'SelfTradePreventionMode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 74
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

        /// GROUP ENCODER (id=25010)
        #[inline]
        pub fn list_triggering_instructions_encoder(
            self,
            count: u8,
            list_triggering_instructions_encoder: ListTriggeringInstructionsEncoder<Self>,
        ) -> ListTriggeringInstructionsEncoder<Self> {
            list_triggering_instructions_encoder.wrap(self, count)
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
    }

    #[derive(Debug, Default)]
    pub struct ListTriggeringInstructionsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for ListTriggeringInstructionsEncoder<P>
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

    impl<'a, P> Encoder<'a> for ListTriggeringInstructionsEncoder<P>
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

    impl<'a, P> ListTriggeringInstructionsEncoder<P>
    where
        P: Encoder<'a> + Default,
    {
        #[inline]
        pub fn wrap(mut self, mut parent: P, count: u8) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 2);
            parent
                .get_buf_mut()
                .put_u8_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 1, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub const fn block_length() -> u8 {
            3
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

        /// REQUIRED enum
        #[inline]
        pub fn list_trigger_type(
            &mut self,
            value: list_trigger_type::ListTriggerType,
        ) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// primitive field 'ListTriggerTriggerIndex'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn list_trigger_trigger_index(&mut self, value: u8) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u8_at(offset, value);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn list_trigger_action(
            &mut self,
            value: list_trigger_action::ListTriggerAction,
        ) -> &mut Self {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct NewOrderListDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for NewOrderListDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for NewOrderListDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for NewOrderListDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> NewOrderListDecoder<'a> {
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

        /// REQUIRED enum
        #[inline]
        pub fn contingency_type(&self) -> contingency_type::ContingencyType {
            self.get_buf().get_u8_at(self.offset).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn opo(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 1).into()
        }

        /// GROUP DECODER (id=73)
        #[inline]
        pub fn orders_decoder(self) -> OrdersDecoder<Self> {
            OrdersDecoder::default().wrap(self)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn cl_list_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn cl_list_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }

    #[derive(Debug, Default)]
    pub struct OrdersDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for OrdersDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for OrdersDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for OrdersDecoder<P>
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

    impl<'a, P> OrdersDecoder<P>
    where
        P: Decoder<'a> + ActingVersion + Default,
    {
        pub fn wrap(mut self, mut parent: P) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset);
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='Orders', referencedName='null', description='null', packageName='null', id=73, version=0, deprecated=0, encodedLength=75, offset=2, componentTokenCount=161, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u8 {
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
        pub fn order_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 2);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_type(&self) -> ord_type::OrdType {
            self.get_buf().get_u8_at(self.offset + 10).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_inst(&self) -> exec_inst::ExecInst {
            self.get_buf().get_u8_at(self.offset + 11).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 12);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_type(&self) -> trigger_type::TriggerType {
            self.get_buf().get_u8_at(self.offset + 20).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_action(&self) -> trigger_action::TriggerAction {
            self.get_buf().get_u8_at(self.offset + 21).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn trigger_price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 22);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_price_type(&self) -> trigger_price_type::TriggerPriceType {
            self.get_buf().get_u8_at(self.offset + 30).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trigger_price_direction(&self) -> trigger_price_direction::TriggerPriceDirection {
            self.get_buf().get_u8_at(self.offset + 31).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xffffffffffffffff_u64' }
        #[inline]
        pub fn trigger_trailing_delta_bips(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 32);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xff_u8' }
        #[inline]
        pub fn peg_offset_value(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 40);
            if value == 0xff_u8 { None } else { Some(value) }
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_price_type(&self) -> peg_price_type::PegPriceType {
            self.get_buf().get_u8_at(self.offset + 41).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_move_type(&self) -> peg_move_type::PegMoveType {
            self.get_buf().get_u8_at(self.offset + 42).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn peg_offset_type(&self) -> peg_offset_type::PegOffsetType {
            self.get_buf().get_u8_at(self.offset + 43).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> side::Side {
            self.get_buf().get_u8_at(self.offset + 44).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> time_in_force::TimeInForce {
            self.get_buf().get_u8_at(self.offset + 45).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn max_floor(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 46);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn cash_order_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 54);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648_i32' }
        #[inline]
        pub fn target_strategy(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 62);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn strategy_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 66);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(
            &self,
        ) -> self_trade_prevention_mode::SelfTradePreventionMode {
            self.get_buf().get_u8_at(self.offset + 74).into()
        }

        /// GROUP DECODER (id=25010)
        #[inline]
        pub fn list_triggering_instructions_decoder(
            self,
        ) -> ListTriggeringInstructionsDecoder<Self> {
            ListTriggeringInstructionsDecoder::default().wrap(self)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn cl_ord_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.parent.as_ref().expect("parent missing").get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.parent
                .as_mut()
                .unwrap()
                .set_limit(offset + 1 + data_length);
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
            let offset = self.parent.as_ref().expect("parent missing").get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.parent
                .as_mut()
                .unwrap()
                .set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn symbol_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }

    #[derive(Debug, Default)]
    pub struct ListTriggeringInstructionsDecoder<P> {
        parent: Option<P>,
        block_length: u8,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for ListTriggeringInstructionsDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for ListTriggeringInstructionsDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for ListTriggeringInstructionsDecoder<P>
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

    impl<'a, P> ListTriggeringInstructionsDecoder<P>
    where
        P: Decoder<'a> + ActingVersion + Default,
    {
        pub fn wrap(mut self, mut parent: P) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u8_at(initial_offset);
            let count = parent.get_buf().get_u8_at(initial_offset + 1);
            parent.set_limit(initial_offset + 2);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='ListTriggeringInstructions', referencedName='null', description='null', packageName='null', id=25010, version=0, deprecated=0, encodedLength=3, offset=75, componentTokenCount=24, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u8 {
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

        /// REQUIRED enum
        #[inline]
        pub fn list_trigger_type(&self) -> list_trigger_type::ListTriggerType {
            self.get_buf().get_u8_at(self.offset).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn list_trigger_trigger_index(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 1)
        }

        /// REQUIRED enum
        #[inline]
        pub fn list_trigger_action(&self) -> list_trigger_action::ListTriggerAction {
            self.get_buf().get_u8_at(self.offset + 2).into()
        }
    }
} // end decoder
