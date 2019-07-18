#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate eth_pairings;
extern crate eth_pairings_cpp;
extern crate hex;

fuzz_target!(|data: &[u8]| {
    let native = eth_pairings::public_interface::API::run(&data);
    let cpp = eth_pairings_cpp::run(&data);
    if native.is_err() {
        if !cpp.is_err() {
            let n = native.err();
            let c = cpp.unwrap();
            println!("Input = {}", hex::encode(&data));
            println!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
            panic!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
        }
    } else {
        let n = native.expect("result");
        if cpp.is_err() {
            let c = cpp.err();
            println!("Input = {}", hex::encode(&data));
            println!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
            panic!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
        }
        let c = cpp.expect("cpp result");
        if n != c {
            println!("Input = {}", hex::encode(&data));
            println!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
            panic!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
        }
    }
});