use crate::*;

pub use decoder::LogonDecoder;
pub use encoder::LogonEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 14;
pub const SBE_TEMPLATE_ID: u16 = 20008;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct LogonEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for LogonEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for LogonEncoder<'a> {
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
            self.encrypt_method_opt(None);
            self.reset_seq_num_flag_opt(None);
            self.message_handling_opt(None);
            self.response_mode_opt(None);
            self.execution_report_type_opt(None);
            self.drop_copy_flag_opt(None);
            self.recv_window_opt(None);
            self
        }
    }

    impl<'a> LogonEncoder<'a> {
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

        /// primitive field 'EncryptMethod'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn encrypt_method(&mut self, value: u8) -> &mut Self {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value);
            self
        }

        /// optional primitive field 'EncryptMethod'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn encrypt_method_opt(&mut self, value: Option<u8>) -> &mut Self {
            match value {
                Some(value) => self.encrypt_method(value),
                None => self.encrypt_method(0xff_u8),
            };
            self
        }

        /// primitive field 'HeartBtInt'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0xffffffff_u32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn heart_bt_int(&mut self, value: u32) -> &mut Self {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u32_at(offset, value);
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn reset_seq_num_flag(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 5;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'ResetSeqNumFlag'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 5
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn reset_seq_num_flag_opt(&mut self, value: Option<bool_enum::BoolEnum>) -> &mut Self {
            match value {
                Some(value) => self.reset_seq_num_flag(value),
                None => self.reset_seq_num_flag(bool_enum::BoolEnum::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_handling(&mut self, value: message_handling::MessageHandling) -> &mut Self {
            let offset = self.offset + 6;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'MessageHandling'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 6
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn message_handling_opt(
            &mut self,
            value: Option<message_handling::MessageHandling>,
        ) -> &mut Self {
            match value {
                Some(value) => self.message_handling(value),
                None => self.message_handling(message_handling::MessageHandling::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn response_mode(&mut self, value: response_mode::ResponseMode) -> &mut Self {
            let offset = self.offset + 7;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'ResponseMode'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 7
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn response_mode_opt(
            &mut self,
            value: Option<response_mode::ResponseMode>,
        ) -> &mut Self {
            match value {
                Some(value) => self.response_mode(value),
                None => self.response_mode(response_mode::ResponseMode::NullVal),
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn execution_report_type(
            &mut self,
            value: execution_report_type::ExecutionReportType,
        ) -> &mut Self {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'ExecutionReportType'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn execution_report_type_opt(
            &mut self,
            value: Option<execution_report_type::ExecutionReportType>,
        ) -> &mut Self {
            match value {
                Some(value) => self.execution_report_type(value),
                None => {
                    self.execution_report_type(execution_report_type::ExecutionReportType::NullVal)
                }
            };
            self
        }

        /// REQUIRED enum
        #[inline]
        pub fn drop_copy_flag(&mut self, value: bool_enum::BoolEnum) -> &mut Self {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8);
            self
        }

        /// optional enum field 'DropCopyFlag'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 9
        /// - encodedLength: 1
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn drop_copy_flag_opt(&mut self, value: Option<bool_enum::BoolEnum>) -> &mut Self {
            match value {
                Some(value) => self.drop_copy_flag(value),
                None => self.drop_copy_flag(bool_enum::BoolEnum::NullVal),
            };
            self
        }

        /// primitive field 'RecvWindow'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0xffffffff_u32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn recv_window(&mut self, value: u32) -> &mut Self {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u32_at(offset, value);
            self
        }

        /// optional primitive field 'RecvWindow'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0xffffffff_u32
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 4
        /// - version: 0
        /// Set to `None` to encode the field null value.
        #[inline]
        pub fn recv_window_opt(&mut self, value: Option<u32>) -> &mut Self {
            match value {
                Some(value) => self.recv_window(value),
                None => self.recv_window(0xffffffff_u32),
            };
            self
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn sender_comp_id(&mut self, value: &str) -> &mut Self {
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
        pub fn target_comp_id(&mut self, value: &str) -> &mut Self {
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
        pub fn raw_data(&mut self, value: &str) -> &mut Self {
            let limit = self.get_limit();
            let data_length = value.len().min((u16::MAX - 1) as usize);
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut()
                .put_slice_at(limit + 2, &value[0..data_length].as_bytes());
            self
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn username(&mut self, value: &str) -> &mut Self {
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
    pub struct LogonDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for LogonDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for LogonDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for LogonDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> LogonDecoder<'a> {
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

        /// primitive field - 'OPTIONAL' { null_value: '0xff_u8' }
        #[inline]
        pub fn encrypt_method(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset);
            if value == 0xff_u8 { None } else { Some(value) }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn heart_bt_int(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 1)
        }

        /// REQUIRED enum
        #[inline]
        pub fn reset_seq_num_flag(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 5).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_handling(&self) -> message_handling::MessageHandling {
            self.get_buf().get_u8_at(self.offset + 6).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn response_mode(&self) -> response_mode::ResponseMode {
            self.get_buf().get_u8_at(self.offset + 7).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn execution_report_type(&self) -> execution_report_type::ExecutionReportType {
            self.get_buf().get_u8_at(self.offset + 8).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn drop_copy_flag(&self) -> bool_enum::BoolEnum {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '0xffffffff_u32' }
        #[inline]
        pub fn recv_window(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 10);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn sender_comp_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn sender_comp_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn target_comp_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn target_comp_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn raw_data_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn raw_data_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn username_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn username_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }
} // end decoder
