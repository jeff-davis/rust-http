use std::util::unreachable;
use std::rt::io::{Reader, Writer};
use extra::time::Tm;
use extra::treemap::TreeMap;
use headers;
use headers::{HeaderEnum, HeaderConvertible, HeaderValueByteIterator};
use headers::serialization_utils::{push_maybe_quoted_string, maybe_unquote_string};

pub enum Header {

    // RFC 2616, Section 4.5: General Header Fields
    CacheControl(~str),  //(headers::cache_control::request::CacheControl),     // Section 14.9
    Connection(headers::connection::Connection),                     // Section 14.10
    Date(Tm),                                                        // Section 14.18
    Pragma(~str),  //(headers::pragma::Pragma),                                 // Section 14.32
    Trailer(~str),  //(headers::trailer::Trailer),                              // Section 14.40
    TransferEncoding(~str),  //(headers::transfer_encoding::TransferEncoding),  // Section 14.41
    Upgrade(~str),  //(headers::upgrade::Upgrade),                              // Section 14.42
    Via(~str),  //(headers::via::Via),                                          // Section 14.45
    Warning(~str),  //(headers::warning::Warning),                              // Section 14.46

    // RFC 2616, Section 5.3: Request Header Fields
    Accept(~str),  //(headers::accept::Accept),                                       // Section 14.1
    AcceptCharset(~str),  //(headers::accept_charset::AcceptCharset),                 // Section 14.2
    AcceptEncoding(~str),  //(headers::accept_encoding::AcceptEncoding),              // Section 14.3
    AcceptLanguage(~str),  //(headers::accept_language::AcceptLanguage),              // Section 14.4
    Authorization(~str),  //(headers::authorization::Authorization),                  // Section 14.8
    Expect(~str),  //(headers::expect::Expect),                                       // Section 14.20
    From(~str),  //(headers::from::From),                                             // Section 14.22
    Host(headers::host::Host),                                             // Section 14.23
    IfMatch(~str),  //(headers::if_match::IfMatch),                                   // Section 14.24
    IfModifiedSince(Tm),                                                   // Section 14.25
    IfNoneMatch(~str),  //(headers::if_none_match::IfNoneMatch),                      // Section 14.26
    IfRange(~str),  //(headers::if_range::IfRange),                                   // Section 14.27
    IfUnmodifiedSince(Tm),                                                 // Section 14.28
    MaxForwards(uint),                                                     // Section 14.31
    ProxyAuthorization(~str),  //(headers::proxy_authorization::ProxyAuthorization),  // Section 14.34
    Range(~str),  //(headers::range::Range),                                          // Section 14.35
    Referer(~str),  //(headers::referer::Referer),                                    // Section 14.36
    Te(~str),  //(headers::te::Te),                                                   // Section 14.39
    UserAgent(~str),  //(headers::user_agent::UserAgent),                             // Section 14.43

    // RFC 2616, Section 7.1: Entity Header Fields
    Allow(headers::allow::Allow),                                 // Section 14.7
    ContentEncoding(~str),  //(headers::content_encoding::ContentEncoding),  // Section 14.11
    ContentLanguage(~str),  //(headers::content_language::ContentLanguage),  // Section 14.12
    ContentLength(uint),                                          // Section 14.13
    ContentLocation(~str),  //(headers::content_location::ContentLocation),  // Section 14.14
    ContentMd5(~str),  //(headers::content_md5::ContentMd5),                 // Section 14.15
    ContentRange(~str),  //(headers::content_range::ContentRange),           // Section 14.16
    ContentType(~str),  //(headers::content_type::ContentType),              // Section 14.17
    Expires(Tm),                                                  // Section 14.21
    LastModified(Tm),                                             // Section 14.29
    ExtensionHeader(~str, ~str),
}

/// Intended to be used as ``request.headers``.
pub struct HeaderCollection {
    // General Header Fields
    cache_control: Option<~str>,
    connection: Option<headers::connection::Connection>,
    date: Option<Tm>,
    pragma: Option<~str>,
    trailer: Option<~str>,
    transfer_encoding: Option<~str>,
    upgrade: Option<~str>,
    via: Option<~str>,
    warning: Option<~str>,

    // Request Header Fields
    accept: Option<~str>,
    accept_charset: Option<~str>,
    accept_encoding: Option<~str>,
    accept_language: Option<~str>,
    authorization: Option<~str>,
    expect: Option<~str>,
    from: Option<~str>,
    host: Option<headers::host::Host>,
    if_match: Option<~str>,
    if_modified_since: Option<Tm>,
    if_none_match: Option<~str>,
    if_range: Option<~str>,
    if_unmodified_since: Option<Tm>,
    max_forwards: Option<uint>,
    proxy_authorization: Option<~str>,
    range: Option<~str>,
    referer: Option<~str>,
    te: Option<~str>,
    user_agent: Option<~str>,

    // Entity Header Fields
    allow: Option<headers::allow::Allow>,
    content_encoding: Option<~str>,
    content_language: Option<~str>,
    content_length: Option<uint>,
    content_location: Option<~str>,
    content_md5: Option<~str>,
    content_range: Option<~str>,
    content_type: Option<~str>,
    expires: Option<Tm>,
    last_modified: Option<Tm>,
    extensions: TreeMap<~str, ~str>,
}

impl HeaderCollection {
    pub fn new() -> HeaderCollection {
        HeaderCollection {
            // General Header Fields
            cache_control: None,
            connection: None,
            date: None,
            pragma: None,
            trailer: None,
            transfer_encoding: None,
            upgrade: None,
            via: None,
            warning: None,

            // Request Header Fields
            accept: None,
            accept_charset: None,
            accept_encoding: None,
            accept_language: None,
            authorization: None,
            expect: None,
            from: None,
            host: None,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_range: None,
            if_unmodified_since: None,
            max_forwards: None,
            proxy_authorization: None,
            range: None,
            referer: None,
            te: None,
            user_agent: None,

            // Entity Header Fields
            allow: None,
            content_encoding: None,
            content_language: None,
            content_length: None,
            content_location: None,
            content_md5: None,
            content_range: None,
            content_type: None,
            expires: None,
            last_modified: None,
            extensions: TreeMap::new(),
        }
    }

    /// Consume a header, putting it into this structure.
    pub fn insert(&mut self, header: Header) {
        match header {
            // General Header Fields
            CacheControl(value) => self.cache_control = Some(value),
            Connection(value) => self.connection = Some(value),
            Date(value) => self.date = Some(value),
            Pragma(value) => self.pragma = Some(value),
            Trailer(value) => self.trailer = Some(value),
            TransferEncoding(value) => self.transfer_encoding = Some(value),
            Upgrade(value) => self.upgrade = Some(value),
            Via(value) => self.via = Some(value),
            Warning(value) => self.warning = Some(value),

            // Request Header Fields
            Accept(value) => self.accept = Some(value),
            AcceptCharset(value) => self.accept_charset = Some(value),
            AcceptEncoding(value) => self.accept_encoding = Some(value),
            AcceptLanguage(value) => self.accept_language = Some(value),
            Authorization(value) => self.authorization = Some(value),
            Expect(value) => self.expect = Some(value),
            From(value) => self.from = Some(value),
            Host(value) => self.host = Some(value),
            IfMatch(value) => self.if_match = Some(value),
            IfModifiedSince(value) => self.if_modified_since = Some(value),
            IfNoneMatch(value) => self.if_none_match = Some(value),
            IfRange(value) => self.if_range = Some(value),
            IfUnmodifiedSince(value) => self.if_unmodified_since = Some(value),
            MaxForwards(value) => self.max_forwards = Some(value),
            ProxyAuthorization(value) => self.proxy_authorization = Some(value),
            Range(value) => self.range = Some(value),
            Referer(value) => self.referer = Some(value),
            Te(value) => self.te = Some(value),
            UserAgent(value) => self.user_agent = Some(value),

            // Entity Header Fields
            Allow(value) => self.allow = Some(value),
            ContentEncoding(value) => self.content_encoding = Some(value),
            ContentLanguage(value) => self.content_language = Some(value),
            ContentLength(value) => self.content_length = Some(value),
            ContentLocation(value) => self.content_location = Some(value),
            ContentMd5(value) => self.content_md5 = Some(value),
            ContentRange(value) => self.content_range = Some(value),
            ContentType(value) => self.content_type = Some(value),
            Expires(value) => self.expires = Some(value),
            LastModified(value) => self.last_modified = Some(value),
            ExtensionHeader(key, value) => { self.extensions.insert(key, value); },
        }
    }
}

impl HeaderEnum for Header {
    fn header_name(&self) -> ~str {
        match *self {
            // General headers
            CacheControl(*) =>     ~"Cache-Control",
            Connection(*) =>       ~"Connection",
            Date(*) =>             ~"Date",
            Pragma(*) =>           ~"Pragma",
            Trailer(*) =>          ~"Trailer",
            TransferEncoding(*) => ~"Transfer-Encoding",
            Upgrade(*) =>          ~"Upgrade",
            Via(*) =>              ~"Via",
            Warning(*) =>          ~"Warning",

            // Request headers
            Accept(*) =>             ~"Accept",
            AcceptCharset(*) =>      ~"Accept-Charset",
            AcceptEncoding(*) =>     ~"Accept-Encoding",
            AcceptLanguage(*) =>     ~"Accept-Language",
            Authorization(*) =>      ~"Authorization",
            Expect(*) =>             ~"Expect",
            From(*) =>               ~"From",
            Host(*) =>               ~"Host",
            IfMatch(*) =>            ~"If-Match",
            IfModifiedSince(*) =>    ~"If-Modified-Since",
            IfNoneMatch(*) =>        ~"If-None-Match",
            IfRange(*) =>            ~"If-Range",
            IfUnmodifiedSince(*) =>  ~"If-Unmodified-Since",
            MaxForwards(*) =>        ~"Max-Forwards",
            ProxyAuthorization(*) => ~"Proxy-Authorization",
            Range(*) =>              ~"Range",
            Referer(*) =>            ~"Referer",
            Te(*) =>                 ~"TE",
            UserAgent(*) =>          ~"User-Agent",

            // Entity headers
            Allow(*) =>           ~"Allow",
            ContentEncoding(*) => ~"Content-Encoding",
            ContentLanguage(*) => ~"Content-Language",
            ContentLength(*) =>   ~"Content-Length",
            ContentLocation(*) => ~"Content-Location",
            ContentMd5(*) =>      ~"Content-MD5",
            ContentRange(*) =>    ~"Content-Range",
            ContentType(*) =>     ~"Content-Type",
            Expires(*) =>         ~"Expires",
            LastModified(*) =>    ~"Last-Modified",
            ExtensionHeader(ref name, _) => name.to_owned(),
        }
    }

    fn write_header<T: Writer>(&self, writer: &mut T) {
        match *self {
            ExtensionHeader(ref name, ref value) => {
                // TODO: be more efficient
                let mut s = ~"";
                // Allocate for name, ": " and quoted value (typically an overallocation of 2 bytes,
                // occasionally an underallocation in case of needing to escape double quotes)
                s.reserve(name.len() + 4 + value.len());
                s.push_str(*name);
                s.push_str(": ");
                let s = push_maybe_quoted_string(s, *value);
                writer.write(s.as_bytes());
                return
            },
            _ => (),
        }

        writer.write(match *self {
            // General headers
            CacheControl(*) =>     bytes!("Cache-Control: "),
            Connection(*) =>       bytes!("Connection: "),
            Date(*) =>             bytes!("Date: "),
            Pragma(*) =>           bytes!("Pragma: "),
            Trailer(*) =>          bytes!("Trailer: "),
            TransferEncoding(*) => bytes!("Transfer-Encoding: "),
            Upgrade(*) =>          bytes!("Upgrade: "),
            Via(*) =>              bytes!("Via: "),
            Warning(*) =>          bytes!("Warning: "),

            // Request headers
            Accept(*) =>             bytes!("Accept: "),
            AcceptCharset(*) =>      bytes!("Accept-Charset: "),
            AcceptEncoding(*) =>     bytes!("Accept-Encoding: "),
            AcceptLanguage(*) =>     bytes!("Accept-Language: "),
            Authorization(*) =>      bytes!("Authorization: "),
            Expect(*) =>             bytes!("Expect: "),
            From(*) =>               bytes!("From: "),
            Host(*) =>               bytes!("Host: "),
            IfMatch(*) =>            bytes!("If-Match: "),
            IfModifiedSince(*) =>    bytes!("If-Modified-Since: "),
            IfNoneMatch(*) =>        bytes!("If-None-Match: "),
            IfRange(*) =>            bytes!("If-Range: "),
            IfUnmodifiedSince(*) =>  bytes!("If-Unmodified-Since: "),
            MaxForwards(*) =>        bytes!("Max-Forwards: "),
            ProxyAuthorization(*) => bytes!("Proxy-Authorization: "),
            Range(*) =>              bytes!("Range: "),
            Referer(*) =>            bytes!("Referer: "),
            Te(*) =>                 bytes!("TE: "),
            UserAgent(*) =>          bytes!("User-Agent: "),

            // Entity headers
            Allow(*) =>           bytes!("Allow: "),
            ContentEncoding(*) => bytes!("Content-Encoding: "),
            ContentLanguage(*) => bytes!("Content-Language: "),
            ContentLength(*) =>   bytes!("Content-Length: "),
            ContentLocation(*) => bytes!("Content-Location: "),
            ContentMd5(*) =>      bytes!("Content-MD5: "),
            ContentRange(*) =>    bytes!("Content-Range: "),
            ContentType(*) =>     bytes!("Content-Type: "),
            Expires(*) =>         bytes!("Expires: "),
            LastModified(*) =>    bytes!("Last-Modified: "),
            ExtensionHeader(*) => unreachable(),  // Already returned
        });

        // FIXME: all the `h` cases satisfy HeaderConvertible, can it be simplified?
        match *self {
            // General headers
            CacheControl(ref h) =>     h.to_stream(writer),
            Connection(ref h) =>       h.to_stream(writer),
            Date(ref h) =>             h.to_stream(writer),
            Pragma(ref h) =>           h.to_stream(writer),
            Trailer(ref h) =>          h.to_stream(writer),
            TransferEncoding(ref h) => h.to_stream(writer),
            Upgrade(ref h) =>          h.to_stream(writer),
            Via(ref h) =>              h.to_stream(writer),
            Warning(ref h) =>          h.to_stream(writer),

            // Request headers
            Accept(ref h) =>             h.to_stream(writer),
            AcceptCharset(ref h) =>      h.to_stream(writer),
            AcceptEncoding(ref h) =>     h.to_stream(writer),
            AcceptLanguage(ref h) =>     h.to_stream(writer),
            Authorization(ref h) =>      h.to_stream(writer),
            Expect(ref h) =>             h.to_stream(writer),
            From(ref h) =>               h.to_stream(writer),
            Host(ref h) =>               h.to_stream(writer),
            IfMatch(ref h) =>            h.to_stream(writer),
            IfModifiedSince(ref h) =>    h.to_stream(writer),
            IfNoneMatch(ref h) =>        h.to_stream(writer),
            IfRange(ref h) =>            h.to_stream(writer),
            IfUnmodifiedSince(ref h) =>  h.to_stream(writer),
            MaxForwards(ref h) =>        h.to_stream(writer),
            ProxyAuthorization(ref h) => h.to_stream(writer),
            Range(ref h) =>              h.to_stream(writer),
            Referer(ref h) =>            h.to_stream(writer),
            Te(ref h) =>                 h.to_stream(writer),
            UserAgent(ref h) =>          h.to_stream(writer),

            // Entity headers
            Allow(ref h) =>           h.to_stream(writer),
            ContentEncoding(ref h) => h.to_stream(writer),
            ContentLanguage(ref h) => h.to_stream(writer),
            ContentLength(ref h) =>   h.to_stream(writer),
            ContentLocation(ref h) => h.to_stream(writer),
            ContentMd5(ref h) =>      h.to_stream(writer),
            ContentRange(ref h) =>    h.to_stream(writer),
            ContentType(ref h) =>     h.to_stream(writer),
            Expires(ref h) =>         h.to_stream(writer),
            LastModified(ref h) =>    h.to_stream(writer),
            ExtensionHeader(*) =>     unreachable(),  // Already returned
        };
        writer.write(bytes!("\r\n"));
    }

    fn value_from_stream<T: Reader>(name: ~str, value: &mut HeaderValueByteIterator<T>)
            -> Option<Header> {
        match name.as_slice() {
            // General headers
            "Cache-Control" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(CacheControl(v)),
                None => None,
            },
            "Connection" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Connection(v)),
                None => None,
            },
            "Date" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Date(v)),
                None => None,
            },
            "Pragma" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Pragma(v)),
                None => None,
            },
            "Trailer" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Trailer(v)),
                None => None,
            },
            "Transfer-Encoding" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(TransferEncoding(v)),
                None => None,
            },
            "Upgrade" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Upgrade(v)),
                None => None,
            },
            "Via" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Via(v)),
                None => None,
            },
            "Warning" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Warning(v)),
                None => None,
            },

            // Request headers
            "Accept" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Accept(v)),
                None => None,
            },
            "Accept-Charset" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(AcceptCharset(v)),
                None => None,
            },
            "Accept-Encoding" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(AcceptEncoding(v)),
                None => None,
            },
            "Accept-Language" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(AcceptLanguage(v)),
                None => None,
            },
            "Authorization" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Authorization(v)),
                None => None,
            },
            "Expect" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Expect(v)),
                None => None,
            },
            "From" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(From(v)),
                None => None,
            },
            "Host" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Host(v)),
                None => None,
            },
            "If-Match" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(IfMatch(v)),
                None => None,
            },
            "If-Modified-Since" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(IfModifiedSince(v)),
                None => None,
            },
            "If-NoneMatch" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(IfNoneMatch(v)),
                None => None,
            },
            "If-Range" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(IfRange(v)),
                None => None,
            },
            "If-Unmodified-Since" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(IfUnmodifiedSince(v)),
                None => None,
            },
            "Max-Forwards" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(MaxForwards(v)),
                None => None,
            },
            "Proxy-Authorization" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ProxyAuthorization(v)),
                None => None,
            },
            "Range" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Range(v)),
                None => None,
            },
            "Referer" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Referer(v)),
                None => None,
            },
            "Te" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Te(v)),
                None => None,
            },
            "User-Agent" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(UserAgent(v)),
                None => None,
            },

            // Entity headers
            "Allow" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Allow(v)),
                None => None,
            },
            "Content-Encoding" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentEncoding(v)),
                None => None,
            },
            "Content-Language" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentLanguage(v)),
                None => None,
            },
            "Content-Length" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentLength(v)),
                None => None,
            },
            "Content-Location" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentLocation(v)),
                None => None,
            },
            "Content-Md5" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentMd5(v)),
                None => None,
            },
            "Content-Range" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentRange(v)),
                None => None,
            },
            "Content-Type" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(ContentType(v)),
                None => None,
            },
            "Expires" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(Expires(v)),
                None => None,
            },
            "Last-Modified" => match HeaderConvertible::from_stream(value) {
                Some(v) => Some(LastModified(v)),
                None => None,
            },
            _ => match maybe_unquote_string(value.collect_to_str()) {
                Some(v) => Some(ExtensionHeader(name, v)),
                None => None,
            }
        }
    }
}
