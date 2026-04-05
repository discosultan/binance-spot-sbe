use crate::*;

pub use decoder::LimitResponseDecoder;
pub use encoder::LimitResponseEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 0;
pub const SBE_TEMPLATE_ID: u16 = 121;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct LimitResponseEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for LimitResponseEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for LimitResponseEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> LimitResponseEncoder<'a> {
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

        /// GROUP ENCODER (id=25003)
        #[inline]
        pub fn limit_indicators_encoder(
            self,
            count: u8,
            limit_indicators_encoder: LimitIndicatorsEncoder<Self>,
        ) -> LimitIndicatorsEncoder<Self> {
            limit_indicators_encoder.wrap(self, count)
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn req_id(&mut self, value: &str) -> &mut Self {
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
    pub struct LimitIndicatorsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for LimitIndicatorsEncoder<P>
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

    impl<'a, P> Encoder<'a> for LimitIndicatorsEncoder<P>
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
            self.limit_reset_interval_opt(None);
            self.limit_reset_interval_resolution_opt(None);
            self
        }
    }

    impl<'a, P> LimitIndicatorsEncoder<P>
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
            22
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
        pub fn limit_type(&mut self, value: limit_type::LimitType) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// primitive field 'LimitCount'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn limit_count(&mut self, value: u64) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u64_at(offset, value);
            self
        }

        /// primitive field 'LimitMax'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 9
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn limit_max(&mut self, value: u64) -> &mut Self {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u64_at(offset, value);
            self
        }

        /// primitive field 'LimitResetInterval'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0xffffffff_u32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 17
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn limit_reset_interval(&mut self, value: u32) -> &mut Self {
            let offset = self.offset + 17;
            self.get_buf_mut().put_u32_at(offset, value);
            self
        }

        /// optional primitive field 'LimitResetInterval'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0xffffffff_u32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 17
        /// - encodedLength: 4
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn limit_reset_interval_opt(&mut self, value: Option<u32>) -> &mut Self {
            match value {
                Some(value) => self.limit_reset_interval(value),
                None => self.limit_reset_interval(0xffffffff_u32),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn limit_reset_interval_resolution(
            &mut self,
            value: limit_reset_interval_resolution::LimitResetIntervalResolution,
        ) -> &mut Self {
            let offset = self.offset + 21;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'LimitResetIntervalResolution'
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
        pub fn limit_reset_interval_resolution_opt(
            &mut self,
            value: Option<limit_reset_interval_resolution::LimitResetIntervalResolution>,
        ) -> &mut Self {
            match value {
                Some(value) => self.limit_reset_interval_resolution(value),
                None => self.limit_reset_interval_resolution(
                    limit_reset_interval_resolution::LimitResetIntervalResolution::NullVal,
                ),
            };
            self
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct LimitResponseDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for LimitResponseDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for LimitResponseDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for LimitResponseDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> LimitResponseDecoder<'a> {
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

        /// GROUP DECODER (id=25003)
        #[inline]
        pub fn limit_indicators_decoder(self) -> LimitIndicatorsDecoder<Self> {
            LimitIndicatorsDecoder::default().wrap(self)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn req_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn req_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }

    #[derive(Debug, Default)]
    pub struct LimitIndicatorsDecoder<P> {
        parent: Option<P>,
        block_length: u8,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for LimitIndicatorsDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for LimitIndicatorsDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for LimitIndicatorsDecoder<P>
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

    impl<'a, P> LimitIndicatorsDecoder<P>
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

        /// group token - Token{signal=BEGIN_GROUP, name='LimitIndicators', referencedName='null', description='null', packageName='null', id=25003, version=0, deprecated=0, encodedLength=22, offset=0, componentTokenCount=32, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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
        pub fn limit_type(&self) -> limit_type::LimitType {
            self.get_buf().get_u8_at(self.offset).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn limit_count(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 1)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn limit_max(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 9)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xffffffff_u32' }
        #[inline]
        pub fn limit_reset_interval(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 17);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn limit_reset_interval_resolution(
            &self,
        ) -> limit_reset_interval_resolution::LimitResetIntervalResolution {
            self.get_buf().get_u8_at(self.offset + 21).into()
        }
    }
} // end decoder
