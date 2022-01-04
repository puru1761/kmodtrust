use bincode::Options;
use openssl::{
    cms::{CMSOptions, CmsContentInfo},
    pkey::{PKey, Private},
    x509::X509,
};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fs::File,
    io::{Read, Write},
};

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

const PKEY_ID_PKCS7: u8 = 2;
const MAGIC: &str = "~Module signature appended~\n";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleSignature {
    pub algo: u8,
    pub hash: u8,
    pub id_type: u8,
    pub signer_len: u8,
    pub key_id_len: u8,
    pad: [u8; 3],
    pub sig_len: u32,
}

impl ModuleSignature {
    pub fn new() -> Self {
        ModuleSignature {
            algo: 0,
            hash: 0,
            id_type: PKEY_ID_PKCS7,
            signer_len: 0,
            key_id_len: 0,
            pad: [0, 0, 0],
            sig_len: 0,
        }
    }
}

impl Default for ModuleSignature {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SignerOptions {
    key_file: String,
    x509_file: String,
    module: String,
    raw_sig: bool,
    raw_sigfile: String,
    save_sig: bool,
    sign_only: bool,
    replace_orig: bool,
    dest: String,
}

impl SignerOptions {
    pub fn new(matches: &ArgMatches) -> Self {
        /*
         * Unfortunately openssl-rust does not yet support setting of the
         * digest algorithm with CmsContentInfo, for the purpose of signing.
         * Therefore, we cannot still provide this functionality until the
         * the following is resolved:
         *
         *    https://github.com/sfackler/rust-openssl/pull/1034/files
         *
         * Once this is done modsec will support his functionality as well.
         * As for now, 'sha256' is the only available digest to be used.
         *
        let hash_alg = matches
            .value_of("HASH_ALGO")
            .unwrap_or_default()
            .to_string();
         */
        let key_file = matches.value_of("KEY").unwrap().to_string();
        let x509_file = matches.value_of("X509").unwrap().to_string();
        let module = matches.value_of("MODULE").unwrap().to_string();

        let save_sig = matches.is_present("save_sig");
        let sign_only = matches.is_present("sign_only");

        let mut raw_sig = false;
        let raw_sigfile = match matches.is_present("raw_sig") {
            true => {
                raw_sig = true;
                matches.value_of("raw_sig").unwrap().to_string()
            }
            false => "".to_string(),
        };

        let mut replace_orig = true;
        let dest = match matches.is_present("dest") {
            true => {
                replace_orig = false;
                matches.value_of("dest").unwrap().to_string()
            }
            false => {
                let mut d = module.clone();
                d.push_str(".~signed~");
                d
            }
        };

        SignerOptions {
            key_file,
            x509_file,
            module,
            raw_sig,
            raw_sigfile,
            save_sig,
            sign_only,
            replace_orig,
            dest,
        }
    }
}

fn read_private_key(path: &str) -> PKey<Private> {
    let mut key =
        File::open(path).unwrap_or_else(|_| panic!("Failed to open private key {}", path));
    let mut key_data = String::new();
    key.read_to_string(&mut key_data)
        .expect("Failed to read private key");

    let pkey = PKey::private_key_from_pem(key_data.as_bytes())
        .expect("Failed to read PEM formatted private key");

    pkey
}

fn read_x509(path: &str) -> X509 {
    let mut x509 = File::open(path).unwrap_or_else(|_| panic!("Failed to open x509 cert {}", path));
    let mut x509_data: Vec<u8> = Vec::new();
    x509.read_to_end(&mut x509_data)
        .expect("Failed to read x509 data");

    let cert: X509;
    if x509_data[0] == 0x30 && x509_data[1] >= 0x81 && x509_data[1] <= 0x84 {
        cert = X509::from_der(&x509_data).expect("Failed to read DER encoded X509 certificate");
    } else {
        cert = X509::from_pem(&x509_data).expect("Failed to read PEM Formatted X509 Certificate")
    }
    cert
}

fn lkm_sign(opts: SignerOptions) {
    let mut sig_info = ModuleSignature::new();

    let mut module = File::open(&opts.module)
        .unwrap_or_else(|_| panic!("Could not open module {}", opts.module));
    let mut kmod_contents: Vec<u8> = Vec::new();
    module
        .read_to_end(&mut kmod_contents)
        .expect("Failed to read contents of module");

    let sig: CmsContentInfo;
    let mut signature = Vec::new();
    if !opts.raw_sig {
        let key = read_private_key(&opts.key_file);
        let x509 = read_x509(&opts.x509_file);

        sig = CmsContentInfo::sign(
            Some(x509.as_ref()),
            Some(key.as_ref()),
            None,
            Some(&kmod_contents.to_vec()),
            CMSOptions::CMS_NOCERTS
                | CMSOptions::BINARY
                | CMSOptions::DETACHED
                | CMSOptions::NOATTR
                | CMSOptions::NOSMIMECAP,
        )
        .expect("Failed to sign kernel module using CMS");

        signature = sig
            .to_der()
            .expect("Failed to convert signature to DER format");

        if opts.save_sig {
            let mut sig_file_name = opts.module.to_owned();
            sig_file_name.push_str(".p7s");
            let mut sig_file = File::create(&sig_file_name)
                .unwrap_or_else(|_| panic!("Failed to create signature file {}", sig_file_name));

            sig_file
                .write_all(&signature)
                .expect("Failed to write signature file");
        }

        kmod_contents.extend(signature.clone().into_iter());

        if opts.sign_only {
            return;
        }
    } else {
        let mut raw_sigfile = File::open(&opts.raw_sigfile)
            .unwrap_or_else(|_| panic!("Failed to open signature file {}", &opts.raw_sigfile));

        raw_sigfile
            .read_to_end(&mut signature)
            .expect("Failed to read signature from raw signature file");

        kmod_contents.extend(signature.clone().into_iter());
    }

    sig_info.sig_len = signature
        .len()
        .try_into()
        .expect("Failed to convert usize into u32");

    let siginfo_bytes = bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(&sig_info)
        .unwrap();

    kmod_contents.extend(siginfo_bytes.into_iter());
    kmod_contents.extend(MAGIC.as_bytes().iter());

    let mut dest = File::create(&opts.dest).expect("Failed to create signed LKM file");

    dest.write_all(&kmod_contents)
        .unwrap_or_else(|_| panic!("Failed to write LKM data to '{}'", &opts.dest));

    if opts.replace_orig {
        std::fs::rename(&opts.dest, &opts.module)
            .expect("Failed to replace unsigned LKM with signed LKM");
    }
}

fn main() {
    let yaml = load_yaml!("app.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(sign_opts) = matches.subcommand_matches("sign") {
        let opts = SignerOptions::new(sign_opts);
        lkm_sign(opts);
    }
}
