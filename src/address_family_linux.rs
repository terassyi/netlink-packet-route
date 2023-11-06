// SPDX-License-Identifier: MIT

const AF_KCM: u8 = 41;
const AF_QIPCRTR: u8 = 42;
const AF_SMC: u8 = 43;
const AF_XDP: u8 = 44;
const AF_MCTP: u8 = 45;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum AddressFamily {
    #[default]
    Unspec,
    Local,
    Unix,
    Inet,
    Ax25,
    Ipx,
    Appletalk,
    Netrom,
    Bridge,
    Atmpvc,
    X25,
    Inet6,
    Rose,
    Decnet,
    Netbeui,
    Security,
    Key,
    Route,
    Netlink,
    Packet,
    Ash,
    Econet,
    Atmsvc,
    Rds,
    Sna,
    Irda,
    Pppox,
    Wanpipe,
    Llc,
    Ib,
    Mpls,
    Can,
    Tipc,
    Bluetooth,
    Iucv,
    Rxrpc,
    Isdn,
    Phonet,
    Ieee802154,
    Caif,
    Alg,
    Nfc,
    Vsock,
    Kcm,
    Qipcrtr,
    Smc,
    Xdp,
    Mctp,
    Other(u8),
}

impl From<u8> for AddressFamily {
    fn from(d: u8) -> Self {
        match d {
            d if d == libc::AF_UNSPEC as u8 => Self::Unspec,
            d if d == libc::AF_UNIX as u8 => Self::Unix,
            d if d == libc::AF_LOCAL as u8 => Self::Local,
            d if d == libc::AF_INET as u8 => Self::Inet,
            d if d == libc::AF_AX25 as u8 => Self::Ax25,
            d if d == libc::AF_IPX as u8 => Self::Ipx,
            d if d == libc::AF_APPLETALK as u8 => Self::Appletalk,
            d if d == libc::AF_NETROM as u8 => Self::Netrom,
            d if d == libc::AF_BRIDGE as u8 => Self::Bridge,
            d if d == libc::AF_ATMPVC as u8 => Self::Atmpvc,
            d if d == libc::AF_X25 as u8 => Self::X25,
            d if d == libc::AF_INET6 as u8 => Self::Inet6,
            d if d == libc::AF_ROSE as u8 => Self::Rose,
            d if d == libc::AF_DECnet as u8 => Self::Decnet,
            d if d == libc::AF_NETBEUI as u8 => Self::Netbeui,
            d if d == libc::AF_SECURITY as u8 => Self::Security,
            d if d == libc::AF_KEY as u8 => Self::Key,
            d if d == libc::AF_NETLINK as u8 => Self::Netlink,
            d if d == libc::AF_ROUTE as u8 => Self::Route,
            d if d == libc::AF_PACKET as u8 => Self::Packet,
            d if d == libc::AF_ASH as u8 => Self::Ash,
            d if d == libc::AF_ECONET as u8 => Self::Econet,
            d if d == libc::AF_ATMSVC as u8 => Self::Atmsvc,
            d if d == libc::AF_RDS as u8 => Self::Rds,
            d if d == libc::AF_SNA as u8 => Self::Sna,
            d if d == libc::AF_IRDA as u8 => Self::Irda,
            d if d == libc::AF_PPPOX as u8 => Self::Pppox,
            d if d == libc::AF_WANPIPE as u8 => Self::Wanpipe,
            d if d == libc::AF_LLC as u8 => Self::Llc,
            d if d == libc::AF_IB as u8 => Self::Ib,
            d if d == libc::AF_MPLS as u8 => Self::Mpls,
            d if d == libc::AF_CAN as u8 => Self::Can,
            d if d == libc::AF_TIPC as u8 => Self::Tipc,
            d if d == libc::AF_BLUETOOTH as u8 => Self::Bluetooth,
            d if d == libc::AF_IUCV as u8 => Self::Iucv,
            d if d == libc::AF_RXRPC as u8 => Self::Rxrpc,
            d if d == libc::AF_ISDN as u8 => Self::Isdn,
            d if d == libc::AF_PHONET as u8 => Self::Phonet,
            d if d == libc::AF_IEEE802154 as u8 => Self::Ieee802154,
            d if d == libc::AF_CAIF as u8 => Self::Caif,
            d if d == libc::AF_ALG as u8 => Self::Alg,
            d if d == libc::AF_NFC as u8 => Self::Nfc,
            d if d == libc::AF_VSOCK as u8 => Self::Vsock,
            d if d == AF_KCM => Self::Kcm,
            d if d == AF_QIPCRTR => Self::Qipcrtr,
            d if d == AF_SMC => Self::Smc,
            d if d == AF_XDP => Self::Xdp,
            d if d == AF_MCTP => Self::Mctp,
            _ => Self::Other(d),
        }
    }
}

impl From<AddressFamily> for u8 {
    fn from(v: AddressFamily) -> u8 {
        match v {
            AddressFamily::Unspec => libc::AF_UNSPEC as u8,
            AddressFamily::Local => libc::AF_LOCAL as u8,
            AddressFamily::Unix => libc::AF_UNIX as u8,
            AddressFamily::Inet => libc::AF_INET as u8,
            AddressFamily::Ax25 => libc::AF_AX25 as u8,
            AddressFamily::Ipx => libc::AF_IPX as u8,
            AddressFamily::Appletalk => libc::AF_APPLETALK as u8,
            AddressFamily::Netrom => libc::AF_NETROM as u8,
            AddressFamily::Bridge => libc::AF_BRIDGE as u8,
            AddressFamily::Atmpvc => libc::AF_ATMPVC as u8,
            AddressFamily::X25 => libc::AF_X25 as u8,
            AddressFamily::Inet6 => libc::AF_INET6 as u8,
            AddressFamily::Rose => libc::AF_ROSE as u8,
            AddressFamily::Decnet => libc::AF_DECnet as u8,
            AddressFamily::Netbeui => libc::AF_NETBEUI as u8,
            AddressFamily::Security => libc::AF_SECURITY as u8,
            AddressFamily::Key => libc::AF_KEY as u8,
            AddressFamily::Route => libc::AF_ROUTE as u8,
            AddressFamily::Netlink => libc::AF_NETLINK as u8,
            AddressFamily::Packet => libc::AF_PACKET as u8,
            AddressFamily::Ash => libc::AF_ASH as u8,
            AddressFamily::Econet => libc::AF_ECONET as u8,
            AddressFamily::Atmsvc => libc::AF_ATMSVC as u8,
            AddressFamily::Rds => libc::AF_RDS as u8,
            AddressFamily::Sna => libc::AF_SNA as u8,
            AddressFamily::Irda => libc::AF_IRDA as u8,
            AddressFamily::Pppox => libc::AF_PPPOX as u8,
            AddressFamily::Wanpipe => libc::AF_WANPIPE as u8,
            AddressFamily::Llc => libc::AF_LLC as u8,
            AddressFamily::Ib => libc::AF_IB as u8,
            AddressFamily::Mpls => libc::AF_MPLS as u8,
            AddressFamily::Can => libc::AF_CAN as u8,
            AddressFamily::Tipc => libc::AF_TIPC as u8,
            AddressFamily::Bluetooth => libc::AF_BLUETOOTH as u8,
            AddressFamily::Iucv => libc::AF_IUCV as u8,
            AddressFamily::Rxrpc => libc::AF_RXRPC as u8,
            AddressFamily::Isdn => libc::AF_ISDN as u8,
            AddressFamily::Phonet => libc::AF_PHONET as u8,
            AddressFamily::Ieee802154 => libc::AF_IEEE802154 as u8,
            AddressFamily::Caif => libc::AF_CAIF as u8,
            AddressFamily::Alg => libc::AF_ALG as u8,
            AddressFamily::Nfc => libc::AF_NFC as u8,
            AddressFamily::Vsock => libc::AF_VSOCK as u8,
            AddressFamily::Kcm => AF_KCM,
            AddressFamily::Qipcrtr => AF_QIPCRTR,
            AddressFamily::Smc => AF_SMC,
            AddressFamily::Xdp => AF_XDP,
            AddressFamily::Mctp => AF_MCTP,
            AddressFamily::Other(d) => d,
        }
    }
}