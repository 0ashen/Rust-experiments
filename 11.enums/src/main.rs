// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }
//
// fn route(ip_kind: IpAddrKind) {}

// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
//
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//
//     let home = IpAddr::V4(127, 0, 0, 1);
//
//     let loopback = IpAddr::V6(String::from("::1"));
// }


#![allow(unused)]
fn main() {
    struct Ipv4Addr {
        // --вырезано--
    }

    struct Ipv6Addr {
        // --вырезано--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}
