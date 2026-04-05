use crate::*;

pub use decoder::InstrumentListDecoder;
pub use encoder::InstrumentListEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 0;
pub const SBE_TEMPLATE_ID: u16 = 201;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct InstrumentListEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for InstrumentListEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for InstrumentListEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> InstrumentListEncoder<'a> {
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

        /// GROUP ENCODER (id=146)
        #[inline]
        pub fn related_sym_encoder(
            self,
            count: u16,
            related_sym_encoder: RelatedSymEncoder<Self>,
        ) -> RelatedSymEncoder<Self> {
            related_sym_encoder.wrap(self, count)
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn instrument_req_id(&mut self, value: &str) -> &mut Self {
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
    pub struct RelatedSymEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for RelatedSymEncoder<P>
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

    impl<'a, P> Encoder<'a> for RelatedSymEncoder<P>
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
            self.min_trade_vol_opt(None);
            self.max_trade_vol_opt(None);
            self.min_qty_increment_opt(None);
            self.market_min_trade_vol_opt(None);
            self.market_max_trade_vol_opt(None);
            self.market_min_qty_increment_opt(None);
            self.start_price_range_opt(None);
            self.end_price_range_opt(None);
            self.min_price_increment_opt(None);
            self
        }
    }

    impl<'a, P> RelatedSymEncoder<P>
    where
        P: Encoder<'a> + Default,
    {
        #[inline]
        pub fn wrap(mut self, mut parent: P, count: u16) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 4);
            parent
                .get_buf_mut()
                .put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u16_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub const fn block_length() -> u16 {
            74
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

        /// primitive field 'MinTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn min_trade_vol(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 2;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MinTradeVol'
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
        pub fn min_trade_vol_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.min_trade_vol(value),
                None => self.min_trade_vol(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'MaxTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn max_trade_vol(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 10;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MaxTradeVol'
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
        pub fn max_trade_vol_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.max_trade_vol(value),
                None => self.max_trade_vol(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'MinQtyIncrement'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 18
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn min_qty_increment(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 18;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MinQtyIncrement'
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
        pub fn min_qty_increment_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.min_qty_increment(value),
                None => self.min_qty_increment(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'MarketMinTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 26
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn market_min_trade_vol(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 26;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MarketMinTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 26
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn market_min_trade_vol_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.market_min_trade_vol(value),
                None => self.market_min_trade_vol(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'MarketMaxTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 34
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn market_max_trade_vol(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 34;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MarketMaxTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 34
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn market_max_trade_vol_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.market_max_trade_vol(value),
                None => self.market_max_trade_vol(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'MarketMinQtyIncrement'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 42
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn market_min_qty_increment(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 42;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MarketMinQtyIncrement'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 42
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn market_min_qty_increment_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.market_min_qty_increment(value),
                None => self.market_min_qty_increment(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'StartPriceRange'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 50
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn start_price_range(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 50;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'StartPriceRange'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 50
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn start_price_range_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.start_price_range(value),
                None => self.start_price_range(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'EndPriceRange'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 58
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn end_price_range(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 58;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'EndPriceRange'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 58
        /// - encodedLength: 8
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn end_price_range_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.end_price_range(value),
                None => self.end_price_range(-9223372036854775808_i64),
            };
            self
        }

        /// primitive field 'MinPriceIncrement'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 66
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn min_price_increment(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 66;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MinPriceIncrement'
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
        pub fn min_price_increment_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.min_price_increment(value),
                None => self.min_price_increment(-9223372036854775808_i64),
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
        pub fn currency(&mut self, value: &str) -> &mut Self {
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
    pub struct InstrumentListDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for InstrumentListDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for InstrumentListDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for InstrumentListDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> InstrumentListDecoder<'a> {
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

        /// GROUP DECODER (id=146)
        #[inline]
        pub fn related_sym_decoder(self) -> RelatedSymDecoder<Self> {
            RelatedSymDecoder::default().wrap(self)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn instrument_req_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn instrument_req_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }

    #[derive(Debug, Default)]
    pub struct RelatedSymDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for RelatedSymDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for RelatedSymDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for RelatedSymDecoder<P>
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

    impl<'a, P> RelatedSymDecoder<P>
    where
        P: Decoder<'a> + ActingVersion + Default,
    {
        pub fn wrap(mut self, mut parent: P) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset);
            let count = parent.get_buf().get_u16_at(initial_offset + 2);
            parent.set_limit(initial_offset + 4);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='RelatedSym', referencedName='null', description='null', packageName='null', id=146, version=0, deprecated=0, encodedLength=74, offset=0, componentTokenCount=51, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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
        pub fn min_trade_vol(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 2);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn max_trade_vol(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 10);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn min_qty_increment(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 18);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn market_min_trade_vol(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 26);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn market_max_trade_vol(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 34);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn market_min_qty_increment(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 42);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn start_price_range(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 50);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn end_price_range(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 58);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn min_price_increment(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 66);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
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

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn currency_decoder(&mut self) -> (usize, usize) {
            let offset = self.parent.as_ref().expect("parent missing").get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.parent
                .as_mut()
                .unwrap()
                .set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn currency_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }
} // end decoder
