// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes how an uncompressed comma-separated values (CSV)-formatted input object is formatted.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CsvInput {
    /// <p>Describes the first line of input. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: First line is not a header.</p> </li>
    /// <li> <p> <code>IGNORE</code>: First line is a header, but you can't use the header values to indicate the column in an expression. You can use column position (such as _1, _2, …) to indicate the column (<code>SELECT s._1 FROM OBJECT s</code>).</p> </li>
    /// <li> <p> <code>Use</code>: First line is a header, and you can use the header value to identify a column in an expression (<code>SELECT "name" FROM OBJECT</code>). </p> </li>
    /// </ul>
    pub file_header_info: ::std::option::Option<crate::types::FileHeaderInfo>,
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row. You can specify any character to indicate a comment line. The default character is <code>#</code>.</p>
    /// <p>Default: <code>#</code> </p>
    pub comments: ::std::option::Option<::std::string::String>,
    /// <p>A single character used for escaping the quotation mark character inside an already escaped value. For example, the value <code>""" a , b """</code> is parsed as <code>" a , b "</code>.</p>
    pub quote_escape_character: ::std::option::Option<::std::string::String>,
    /// <p>A single character used to separate individual records in the input. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub record_delimiter: ::std::option::Option<::std::string::String>,
    /// <p>A single character used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub field_delimiter: ::std::option::Option<::std::string::String>,
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    /// <p>Type: String</p>
    /// <p>Default: <code>"</code> </p>
    /// <p>Ancestors: <code>CSV</code> </p>
    pub quote_character: ::std::option::Option<::std::string::String>,
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub allow_quoted_record_delimiter: bool,
}
impl CsvInput {
    /// <p>Describes the first line of input. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: First line is not a header.</p> </li>
    /// <li> <p> <code>IGNORE</code>: First line is a header, but you can't use the header values to indicate the column in an expression. You can use column position (such as _1, _2, …) to indicate the column (<code>SELECT s._1 FROM OBJECT s</code>).</p> </li>
    /// <li> <p> <code>Use</code>: First line is a header, and you can use the header value to identify a column in an expression (<code>SELECT "name" FROM OBJECT</code>). </p> </li>
    /// </ul>
    pub fn file_header_info(&self) -> ::std::option::Option<&crate::types::FileHeaderInfo> {
        self.file_header_info.as_ref()
    }
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row. You can specify any character to indicate a comment line. The default character is <code>#</code>.</p>
    /// <p>Default: <code>#</code> </p>
    pub fn comments(&self) -> ::std::option::Option<&str> {
        self.comments.as_deref()
    }
    /// <p>A single character used for escaping the quotation mark character inside an already escaped value. For example, the value <code>""" a , b """</code> is parsed as <code>" a , b "</code>.</p>
    pub fn quote_escape_character(&self) -> ::std::option::Option<&str> {
        self.quote_escape_character.as_deref()
    }
    /// <p>A single character used to separate individual records in the input. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn record_delimiter(&self) -> ::std::option::Option<&str> {
        self.record_delimiter.as_deref()
    }
    /// <p>A single character used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn field_delimiter(&self) -> ::std::option::Option<&str> {
        self.field_delimiter.as_deref()
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    /// <p>Type: String</p>
    /// <p>Default: <code>"</code> </p>
    /// <p>Ancestors: <code>CSV</code> </p>
    pub fn quote_character(&self) -> ::std::option::Option<&str> {
        self.quote_character.as_deref()
    }
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub fn allow_quoted_record_delimiter(&self) -> bool {
        self.allow_quoted_record_delimiter
    }
}
impl CsvInput {
    /// Creates a new builder-style object to manufacture [`CsvInput`](crate::types::CsvInput).
    pub fn builder() -> crate::types::builders::CsvInputBuilder {
        crate::types::builders::CsvInputBuilder::default()
    }
}

/// A builder for [`CsvInput`](crate::types::CsvInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CsvInputBuilder {
    pub(crate) file_header_info: ::std::option::Option<crate::types::FileHeaderInfo>,
    pub(crate) comments: ::std::option::Option<::std::string::String>,
    pub(crate) quote_escape_character: ::std::option::Option<::std::string::String>,
    pub(crate) record_delimiter: ::std::option::Option<::std::string::String>,
    pub(crate) field_delimiter: ::std::option::Option<::std::string::String>,
    pub(crate) quote_character: ::std::option::Option<::std::string::String>,
    pub(crate) allow_quoted_record_delimiter: ::std::option::Option<bool>,
}
impl CsvInputBuilder {
    /// <p>Describes the first line of input. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: First line is not a header.</p> </li>
    /// <li> <p> <code>IGNORE</code>: First line is a header, but you can't use the header values to indicate the column in an expression. You can use column position (such as _1, _2, …) to indicate the column (<code>SELECT s._1 FROM OBJECT s</code>).</p> </li>
    /// <li> <p> <code>Use</code>: First line is a header, and you can use the header value to identify a column in an expression (<code>SELECT "name" FROM OBJECT</code>). </p> </li>
    /// </ul>
    pub fn file_header_info(mut self, input: crate::types::FileHeaderInfo) -> Self {
        self.file_header_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the first line of input. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: First line is not a header.</p> </li>
    /// <li> <p> <code>IGNORE</code>: First line is a header, but you can't use the header values to indicate the column in an expression. You can use column position (such as _1, _2, …) to indicate the column (<code>SELECT s._1 FROM OBJECT s</code>).</p> </li>
    /// <li> <p> <code>Use</code>: First line is a header, and you can use the header value to identify a column in an expression (<code>SELECT "name" FROM OBJECT</code>). </p> </li>
    /// </ul>
    pub fn set_file_header_info(mut self, input: ::std::option::Option<crate::types::FileHeaderInfo>) -> Self {
        self.file_header_info = input;
        self
    }
    /// <p>Describes the first line of input. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: First line is not a header.</p> </li>
    /// <li> <p> <code>IGNORE</code>: First line is a header, but you can't use the header values to indicate the column in an expression. You can use column position (such as _1, _2, …) to indicate the column (<code>SELECT s._1 FROM OBJECT s</code>).</p> </li>
    /// <li> <p> <code>Use</code>: First line is a header, and you can use the header value to identify a column in an expression (<code>SELECT "name" FROM OBJECT</code>). </p> </li>
    /// </ul>
    pub fn get_file_header_info(&self) -> &::std::option::Option<crate::types::FileHeaderInfo> {
        &self.file_header_info
    }
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row. You can specify any character to indicate a comment line. The default character is <code>#</code>.</p>
    /// <p>Default: <code>#</code> </p>
    pub fn comments(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comments = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row. You can specify any character to indicate a comment line. The default character is <code>#</code>.</p>
    /// <p>Default: <code>#</code> </p>
    pub fn set_comments(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comments = input;
        self
    }
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row. You can specify any character to indicate a comment line. The default character is <code>#</code>.</p>
    /// <p>Default: <code>#</code> </p>
    pub fn get_comments(&self) -> &::std::option::Option<::std::string::String> {
        &self.comments
    }
    /// <p>A single character used for escaping the quotation mark character inside an already escaped value. For example, the value <code>""" a , b """</code> is parsed as <code>" a , b "</code>.</p>
    pub fn quote_escape_character(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.quote_escape_character = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used for escaping the quotation mark character inside an already escaped value. For example, the value <code>""" a , b """</code> is parsed as <code>" a , b "</code>.</p>
    pub fn set_quote_escape_character(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.quote_escape_character = input;
        self
    }
    /// <p>A single character used for escaping the quotation mark character inside an already escaped value. For example, the value <code>""" a , b """</code> is parsed as <code>" a , b "</code>.</p>
    pub fn get_quote_escape_character(&self) -> &::std::option::Option<::std::string::String> {
        &self.quote_escape_character
    }
    /// <p>A single character used to separate individual records in the input. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn record_delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.record_delimiter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used to separate individual records in the input. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn set_record_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.record_delimiter = input;
        self
    }
    /// <p>A single character used to separate individual records in the input. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn get_record_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        &self.record_delimiter
    }
    /// <p>A single character used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn field_delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.field_delimiter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn set_field_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.field_delimiter = input;
        self
    }
    /// <p>A single character used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn get_field_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        &self.field_delimiter
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    /// <p>Type: String</p>
    /// <p>Default: <code>"</code> </p>
    /// <p>Ancestors: <code>CSV</code> </p>
    pub fn quote_character(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.quote_character = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    /// <p>Type: String</p>
    /// <p>Default: <code>"</code> </p>
    /// <p>Ancestors: <code>CSV</code> </p>
    pub fn set_quote_character(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.quote_character = input;
        self
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    /// <p>Type: String</p>
    /// <p>Default: <code>"</code> </p>
    /// <p>Ancestors: <code>CSV</code> </p>
    pub fn get_quote_character(&self) -> &::std::option::Option<::std::string::String> {
        &self.quote_character
    }
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub fn allow_quoted_record_delimiter(mut self, input: bool) -> Self {
        self.allow_quoted_record_delimiter = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub fn set_allow_quoted_record_delimiter(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_quoted_record_delimiter = input;
        self
    }
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub fn get_allow_quoted_record_delimiter(&self) -> &::std::option::Option<bool> {
        &self.allow_quoted_record_delimiter
    }
    /// Consumes the builder and constructs a [`CsvInput`](crate::types::CsvInput).
    pub fn build(self) -> crate::types::CsvInput {
        crate::types::CsvInput {
            file_header_info: self.file_header_info,
            comments: self.comments,
            quote_escape_character: self.quote_escape_character,
            record_delimiter: self.record_delimiter,
            field_delimiter: self.field_delimiter,
            quote_character: self.quote_character,
            allow_quoted_record_delimiter: self.allow_quoted_record_delimiter.unwrap_or_default(),
        }
    }
}
