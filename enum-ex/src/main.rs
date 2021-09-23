use std::option::Option;

fn main() {
        
    
    
   /* enum IpAddrKind {
            v4,
            v6,
        }
//enum use inside strucure
        struct IpAddr {
            kind : IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::v4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::v6,
            address: String::from("::1"),
        }; */

        enum IpAddr {
            v4(String),
            v6(String),
        }

        let home = IpAddr::v4(String::from("127.0.0.1"));

        let loopback = IpAddr::v6(String::from("::1"));

        enum Option<T> {
            Some(T),
            None,
        }

        let some_no = Some(5);

        let y: Option<i8> = Some(5);
        let absent_no: Option<i32> = None;

        println!(" abesent no {}", absent_no);
}

