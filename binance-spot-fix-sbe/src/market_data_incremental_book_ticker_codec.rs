use crate::*;

pub use decoder::MarketDataIncrementalBookTickerDecoder;
pub use encoder::MarketDataIncrementalBookTickerEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 10;
pub const SBE_TEMPLATE_ID: u16 = 206;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct MarketDataIncrementalBookTickerEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for MarketDataIncrementalBookTickerEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for MarketDataIncrementalBookTickerEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> MarketDataIncrementalBookTickerEncoder<'a> {
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

        /// primitive field 'LastBookUpdateID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn last_book_update_id(&mut self, value: i64) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'PriceExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn price_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// primitive field 'QtyExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128_i8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 9
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn qty_exponent(&mut self, value: i8) -> &mut Self {
            let offset = self.offset + 9;
            self.get_buf_mut().put_i8_at(offset, value);
            self
        }

        /// GROUP ENCODER (id=25047)
        #[inline]
        pub fn md_entries_bids_encoder(
            self,
            count: u8,
            md_entries_bids_encoder: MDEntriesBidsEncoder<Self>,
        ) -> MDEntriesBidsEncoder<Self> {
            md_entries_bids_encoder.wrap(self, count)
        }

        /// GROUP ENCODER (id=25048)
        #[inline]
        pub fn md_entries_asks_encoder(
            self,
            count: u8,
            md_entries_asks_encoder: MDEntriesAsksEncoder<Self>,
        ) -> MDEntriesAsksEncoder<Self> {
            md_entries_asks_encoder.wrap(self, count)
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
    pub struct MDEntriesBidsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for MDEntriesBidsEncoder<P>
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

    impl<'a, P> Encoder<'a> for MDEntriesBidsEncoder<P>
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
            self.md_entry_size_opt(None);
            self
        }
    }

    impl<'a, P> MDEntriesBidsEncoder<P>
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
            16
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

        /// primitive field 'MDEntryPx'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn md_entry_px(&mut self, value: i64) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'MDEntrySize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn md_entry_size(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MDEntrySize'
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
        pub fn md_entry_size_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.md_entry_size(value),
                None => self.md_entry_size(-9223372036854775808_i64),
            };
            self
        }
    }

    #[derive(Debug, Default)]
    pub struct MDEntriesAsksEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for MDEntriesAsksEncoder<P>
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

    impl<'a, P> Encoder<'a> for MDEntriesAsksEncoder<P>
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
            self.md_entry_size_opt(None);
            self
        }
    }

    impl<'a, P> MDEntriesAsksEncoder<P>
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
            16
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

        /// primitive field 'MDEntryPx'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn md_entry_px(&mut self, value: i64) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// primitive field 'MDEntrySize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808_i64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn md_entry_size(&mut self, value: i64) -> &mut Self {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
            self
        }

        /// optional primitive field 'MDEntrySize'
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
        pub fn md_entry_size_opt(&mut self, value: Option<i64>) -> &mut Self {
            match value {
                Some(value) => self.md_entry_size(value),
                None => self.md_entry_size(-9223372036854775808_i64),
            };
            self
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct MarketDataIncrementalBookTickerDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for MarketDataIncrementalBookTickerDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for MarketDataIncrementalBookTickerDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for MarketDataIncrementalBookTickerDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> MarketDataIncrementalBookTickerDecoder<'a> {
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
        pub fn last_book_update_id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn qty_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 9)
        }

        /// GROUP DECODER (id=25047)
        #[inline]
        pub fn md_entries_bids_decoder(self) -> MDEntriesBidsDecoder<Self> {
            MDEntriesBidsDecoder::default().wrap(self)
        }

        /// GROUP DECODER (id=25048)
        #[inline]
        pub fn md_entries_asks_decoder(self) -> MDEntriesAsksDecoder<Self> {
            MDEntriesAsksDecoder::default().wrap(self)
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
    }

    #[derive(Debug, Default)]
    pub struct MDEntriesBidsDecoder<P> {
        parent: Option<P>,
        block_length: u8,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for MDEntriesBidsDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for MDEntriesBidsDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for MDEntriesBidsDecoder<P>
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

    impl<'a, P> MDEntriesBidsDecoder<P>
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

        /// group token - Token{signal=BEGIN_GROUP, name='MDEntriesBids', referencedName='null', description='null', packageName='null', id=25047, version=0, deprecated=0, encodedLength=16, offset=10, componentTokenCount=12, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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
        pub fn md_entry_px(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn md_entry_size(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 8);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct MDEntriesAsksDecoder<P> {
        parent: Option<P>,
        block_length: u8,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for MDEntriesAsksDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for MDEntriesAsksDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for MDEntriesAsksDecoder<P>
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

    impl<'a, P> MDEntriesAsksDecoder<P>
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

        /// group token - Token{signal=BEGIN_GROUP, name='MDEntriesAsks', referencedName='null', description='null', packageName='null', id=25048, version=0, deprecated=0, encodedLength=16, offset=-1, componentTokenCount=12, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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
        pub fn md_entry_px(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808_i64' }
        #[inline]
        pub fn md_entry_size(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 8);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }
    }
} // end decoder
