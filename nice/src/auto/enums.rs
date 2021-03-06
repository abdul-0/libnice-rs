// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceCandidateTransport")]
pub enum CandidateTransport {
    #[doc(alias = "NICE_CANDIDATE_TRANSPORT_UDP")]
    Udp,
    #[doc(alias = "NICE_CANDIDATE_TRANSPORT_TCP_ACTIVE")]
    TcpActive,
    #[doc(alias = "NICE_CANDIDATE_TRANSPORT_TCP_PASSIVE")]
    TcpPassive,
    #[doc(alias = "NICE_CANDIDATE_TRANSPORT_TCP_SO")]
    TcpSo,
    #[doc(hidden)]
    __Unknown(i32),
}

impl CandidateTransport {
    #[cfg(any(feature = "v0_1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
    #[doc(alias = "nice_candidate_transport_to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::nice_candidate_transport_to_string(self.to_glib())) }
    }
}

impl fmt::Display for CandidateTransport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CandidateTransport::{}",
            match *self {
                CandidateTransport::Udp => "Udp",
                CandidateTransport::TcpActive => "TcpActive",
                CandidateTransport::TcpPassive => "TcpPassive",
                CandidateTransport::TcpSo => "TcpSo",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for CandidateTransport {
    type GlibType = ffi::NiceCandidateTransport;

    fn to_glib(&self) -> ffi::NiceCandidateTransport {
        match *self {
            CandidateTransport::Udp => ffi::NICE_CANDIDATE_TRANSPORT_UDP,
            CandidateTransport::TcpActive => ffi::NICE_CANDIDATE_TRANSPORT_TCP_ACTIVE,
            CandidateTransport::TcpPassive => ffi::NICE_CANDIDATE_TRANSPORT_TCP_PASSIVE,
            CandidateTransport::TcpSo => ffi::NICE_CANDIDATE_TRANSPORT_TCP_SO,
            CandidateTransport::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceCandidateTransport> for CandidateTransport {
    unsafe fn from_glib(value: ffi::NiceCandidateTransport) -> Self {
        skip_assert_initialized!();
        match value {
            0 => CandidateTransport::Udp,
            1 => CandidateTransport::TcpActive,
            2 => CandidateTransport::TcpPassive,
            3 => CandidateTransport::TcpSo,
            value => CandidateTransport::__Unknown(value),
        }
    }
}

impl StaticType for CandidateTransport {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_candidate_transport_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CandidateTransport {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CandidateTransport {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CandidateTransport {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceCandidateType")]
pub enum CandidateType {
    #[doc(alias = "NICE_CANDIDATE_TYPE_HOST")]
    Host,
    #[doc(alias = "NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE")]
    ServerReflexive,
    #[doc(alias = "NICE_CANDIDATE_TYPE_PEER_REFLEXIVE")]
    PeerReflexive,
    #[doc(alias = "NICE_CANDIDATE_TYPE_RELAYED")]
    Relayed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl CandidateType {
    #[cfg(any(feature = "v0_1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
    #[doc(alias = "nice_candidate_type_to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::nice_candidate_type_to_string(self.to_glib())) }
    }
}

impl fmt::Display for CandidateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CandidateType::{}",
            match *self {
                CandidateType::Host => "Host",
                CandidateType::ServerReflexive => "ServerReflexive",
                CandidateType::PeerReflexive => "PeerReflexive",
                CandidateType::Relayed => "Relayed",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for CandidateType {
    type GlibType = ffi::NiceCandidateType;

    fn to_glib(&self) -> ffi::NiceCandidateType {
        match *self {
            CandidateType::Host => ffi::NICE_CANDIDATE_TYPE_HOST,
            CandidateType::ServerReflexive => ffi::NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE,
            CandidateType::PeerReflexive => ffi::NICE_CANDIDATE_TYPE_PEER_REFLEXIVE,
            CandidateType::Relayed => ffi::NICE_CANDIDATE_TYPE_RELAYED,
            CandidateType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceCandidateType> for CandidateType {
    unsafe fn from_glib(value: ffi::NiceCandidateType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => CandidateType::Host,
            1 => CandidateType::ServerReflexive,
            2 => CandidateType::PeerReflexive,
            3 => CandidateType::Relayed,
            value => CandidateType::__Unknown(value),
        }
    }
}

impl StaticType for CandidateType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_candidate_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CandidateType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CandidateType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CandidateType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceCompatibility")]
pub enum Compatibility {
    #[doc(alias = "NICE_COMPATIBILITY_RFC5245")]
    Rfc5245,
    #[doc(alias = "NICE_COMPATIBILITY_GOOGLE")]
    Google,
    #[doc(alias = "NICE_COMPATIBILITY_MSN")]
    Msn,
    #[doc(alias = "NICE_COMPATIBILITY_WLM2009")]
    Wlm2009,
    #[doc(alias = "NICE_COMPATIBILITY_OC2007")]
    Oc2007,
    #[doc(alias = "NICE_COMPATIBILITY_OC2007R2")]
    Oc2007r2,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Compatibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Compatibility::{}",
            match *self {
                Compatibility::Rfc5245 => "Rfc5245",
                Compatibility::Google => "Google",
                Compatibility::Msn => "Msn",
                Compatibility::Wlm2009 => "Wlm2009",
                Compatibility::Oc2007 => "Oc2007",
                Compatibility::Oc2007r2 => "Oc2007r2",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Compatibility {
    type GlibType = ffi::NiceCompatibility;

    fn to_glib(&self) -> ffi::NiceCompatibility {
        match *self {
            Compatibility::Rfc5245 => ffi::NICE_COMPATIBILITY_RFC5245,
            Compatibility::Google => ffi::NICE_COMPATIBILITY_GOOGLE,
            Compatibility::Msn => ffi::NICE_COMPATIBILITY_MSN,
            Compatibility::Wlm2009 => ffi::NICE_COMPATIBILITY_WLM2009,
            Compatibility::Oc2007 => ffi::NICE_COMPATIBILITY_OC2007,
            Compatibility::Oc2007r2 => ffi::NICE_COMPATIBILITY_OC2007R2,
            Compatibility::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceCompatibility> for Compatibility {
    unsafe fn from_glib(value: ffi::NiceCompatibility) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Compatibility::Rfc5245,
            1 => Compatibility::Google,
            2 => Compatibility::Msn,
            3 => Compatibility::Wlm2009,
            4 => Compatibility::Oc2007,
            5 => Compatibility::Oc2007r2,
            value => Compatibility::__Unknown(value),
        }
    }
}

impl StaticType for Compatibility {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_compatibility_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Compatibility {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Compatibility {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Compatibility {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceComponentState")]
pub enum ComponentState {
    #[doc(alias = "NICE_COMPONENT_STATE_DISCONNECTED")]
    Disconnected,
    #[doc(alias = "NICE_COMPONENT_STATE_GATHERING")]
    Gathering,
    #[doc(alias = "NICE_COMPONENT_STATE_CONNECTING")]
    Connecting,
    #[doc(alias = "NICE_COMPONENT_STATE_CONNECTED")]
    Connected,
    #[doc(alias = "NICE_COMPONENT_STATE_READY")]
    Ready,
    #[doc(alias = "NICE_COMPONENT_STATE_FAILED")]
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl ComponentState {
    #[cfg(any(feature = "v0_1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_6")))]
    #[doc(alias = "nice_component_state_to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::nice_component_state_to_string(self.to_glib())) }
    }
}

impl fmt::Display for ComponentState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ComponentState::{}",
            match *self {
                ComponentState::Disconnected => "Disconnected",
                ComponentState::Gathering => "Gathering",
                ComponentState::Connecting => "Connecting",
                ComponentState::Connected => "Connected",
                ComponentState::Ready => "Ready",
                ComponentState::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ComponentState {
    type GlibType = ffi::NiceComponentState;

    fn to_glib(&self) -> ffi::NiceComponentState {
        match *self {
            ComponentState::Disconnected => ffi::NICE_COMPONENT_STATE_DISCONNECTED,
            ComponentState::Gathering => ffi::NICE_COMPONENT_STATE_GATHERING,
            ComponentState::Connecting => ffi::NICE_COMPONENT_STATE_CONNECTING,
            ComponentState::Connected => ffi::NICE_COMPONENT_STATE_CONNECTED,
            ComponentState::Ready => ffi::NICE_COMPONENT_STATE_READY,
            ComponentState::Failed => ffi::NICE_COMPONENT_STATE_FAILED,
            ComponentState::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceComponentState> for ComponentState {
    unsafe fn from_glib(value: ffi::NiceComponentState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ComponentState::Disconnected,
            1 => ComponentState::Gathering,
            2 => ComponentState::Connecting,
            3 => ComponentState::Connected,
            4 => ComponentState::Ready,
            5 => ComponentState::Failed,
            value => ComponentState::__Unknown(value),
        }
    }
}

impl StaticType for ComponentState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_component_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ComponentState {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ComponentState {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ComponentState {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceComponentType")]
pub enum ComponentType {
    #[doc(alias = "NICE_COMPONENT_TYPE_RTP")]
    Rtp,
    #[doc(alias = "NICE_COMPONENT_TYPE_RTCP")]
    Rtcp,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ComponentType::{}",
            match *self {
                ComponentType::Rtp => "Rtp",
                ComponentType::Rtcp => "Rtcp",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ComponentType {
    type GlibType = ffi::NiceComponentType;

    fn to_glib(&self) -> ffi::NiceComponentType {
        match *self {
            ComponentType::Rtp => ffi::NICE_COMPONENT_TYPE_RTP,
            ComponentType::Rtcp => ffi::NICE_COMPONENT_TYPE_RTCP,
            ComponentType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceComponentType> for ComponentType {
    unsafe fn from_glib(value: ffi::NiceComponentType) -> Self {
        skip_assert_initialized!();
        match value {
            1 => ComponentType::Rtp,
            2 => ComponentType::Rtcp,
            value => ComponentType::__Unknown(value),
        }
    }
}

impl StaticType for ComponentType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_component_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ComponentType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ComponentType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ComponentType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceNominationMode")]
pub enum NominationMode {
    #[doc(alias = "NICE_NOMINATION_MODE_REGULAR")]
    Regular,
    #[doc(alias = "NICE_NOMINATION_MODE_AGGRESSIVE")]
    Aggressive,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl fmt::Display for NominationMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NominationMode::{}",
            match *self {
                NominationMode::Regular => "Regular",
                NominationMode::Aggressive => "Aggressive",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[doc(hidden)]
impl ToGlib for NominationMode {
    type GlibType = ffi::NiceNominationMode;

    fn to_glib(&self) -> ffi::NiceNominationMode {
        match *self {
            NominationMode::Regular => ffi::NICE_NOMINATION_MODE_REGULAR,
            NominationMode::Aggressive => ffi::NICE_NOMINATION_MODE_AGGRESSIVE,
            NominationMode::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[doc(hidden)]
impl FromGlib<ffi::NiceNominationMode> for NominationMode {
    unsafe fn from_glib(value: ffi::NiceNominationMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => NominationMode::Regular,
            1 => NominationMode::Aggressive,
            value => NominationMode::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl StaticType for NominationMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_nomination_mode_get_type()) }
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl<'a> FromValueOptional<'a> for NominationMode {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl<'a> FromValue<'a> for NominationMode {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl SetValue for NominationMode {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceProxyType")]
pub enum ProxyType {
    #[doc(alias = "NICE_PROXY_TYPE_NONE")]
    None,
    #[doc(alias = "NICE_PROXY_TYPE_SOCKS5")]
    Socks5,
    #[doc(alias = "NICE_PROXY_TYPE_HTTP")]
    Http,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ProxyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ProxyType::{}",
            match *self {
                ProxyType::None => "None",
                ProxyType::Socks5 => "Socks5",
                ProxyType::Http => "Http",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ProxyType {
    type GlibType = ffi::NiceProxyType;

    fn to_glib(&self) -> ffi::NiceProxyType {
        match *self {
            ProxyType::None => ffi::NICE_PROXY_TYPE_NONE,
            ProxyType::Socks5 => ffi::NICE_PROXY_TYPE_SOCKS5,
            ProxyType::Http => ffi::NICE_PROXY_TYPE_HTTP,
            ProxyType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceProxyType> for ProxyType {
    unsafe fn from_glib(value: ffi::NiceProxyType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ProxyType::None,
            1 => ProxyType::Socks5,
            2 => ProxyType::Http,
            value => ProxyType::__Unknown(value),
        }
    }
}

impl StaticType for ProxyType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_proxy_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ProxyType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ProxyType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ProxyType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "NiceRelayType")]
pub enum RelayType {
    #[doc(alias = "NICE_RELAY_TYPE_TURN_UDP")]
    Udp,
    #[doc(alias = "NICE_RELAY_TYPE_TURN_TCP")]
    Tcp,
    #[doc(alias = "NICE_RELAY_TYPE_TURN_TLS")]
    Tls,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RelayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RelayType::{}",
            match *self {
                RelayType::Udp => "Udp",
                RelayType::Tcp => "Tcp",
                RelayType::Tls => "Tls",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for RelayType {
    type GlibType = ffi::NiceRelayType;

    fn to_glib(&self) -> ffi::NiceRelayType {
        match *self {
            RelayType::Udp => ffi::NICE_RELAY_TYPE_TURN_UDP,
            RelayType::Tcp => ffi::NICE_RELAY_TYPE_TURN_TCP,
            RelayType::Tls => ffi::NICE_RELAY_TYPE_TURN_TLS,
            RelayType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NiceRelayType> for RelayType {
    unsafe fn from_glib(value: ffi::NiceRelayType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RelayType::Udp,
            1 => RelayType::Tcp,
            2 => RelayType::Tls,
            value => RelayType::__Unknown(value),
        }
    }
}

impl StaticType for RelayType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_relay_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RelayType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RelayType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RelayType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
