use libloading::{Library, Symbol};
// use std::ffi::CString;
use std::os::raw::{
    c_int, c_char, c_uchar, c_long
};
// use num_bigint_dig::BigUint;
// use num_traits::FromPrimitive;
// use gmp::mpz::Mpz;

pub type NodeType = c_int;

#[repr(C)]
pub struct CommitC {
    pub file_index: i64,
    pub roots: *mut *mut u8,
    pub roots_count: i32,
}

#[repr(C)]
pub struct ExpandersC {
    pub k: i64,
    pub n: i64,
    pub d: i64,
    pub size: i64,
}

#[repr(C)]
pub struct ProverNodeC {
    pub id: *mut u8,
    pub commits_buf: *mut *mut CommitC,
    pub buf_size: i32,
    pub acc: *mut u8,
    pub count: i64,
}

#[repr(C)]
// pub struct RsaKeyC {
//     pub n: Mpz,
//     pub g: Mpz,
// }
pub struct RsaKeyC {
    pub n: i64,
    pub g: i64,
}

#[repr(C)]
pub struct VerifierC {
    pub key: RsaKeyC,
    pub expanders: ExpandersC,
    pub nodes: *mut *mut ProverNodeC,
    pub nodes_count: i32,
}

#[repr(C)]
pub struct MhtProofC {
    index: NodeType,
    label: *mut c_uchar,
    paths: *mut *mut c_uchar,
    locs: *mut c_uchar,
}

#[repr(C)]
pub struct CommitProofC {
    node: *mut MhtProofC,
    parents: *mut *mut MhtProofC,
    parents_count: c_int,
}

type CallRegisterProverNodeFunc = unsafe extern "C" fn(*mut VerifierC, *mut u8, c_int);
type CallReceiveCommitsFunc =
    unsafe extern "C" fn(*mut VerifierC, *mut c_uchar, c_int, *mut *mut CommitC, c_int) -> c_int;

type CallCommitChallengesFunc = unsafe extern "C" fn(*mut VerifierC, *mut c_uchar, c_int, c_int, c_int) -> c_int;
type CallSpaceChallengesFunc = unsafe extern "C" fn(*mut VerifierC, *mut c_uchar, c_int, c_long) -> c_int;
type PerformPoisFunc = unsafe extern "C" fn(*mut RsaKeyC, c_long, c_long, c_long);


fn load_library() -> Library {
    unsafe {
        Library::new("/home/thgy/work/cess_pos_demo_v2/main.so")
            .expect("Failed to load the dynamic library")
    }
}

// Create a VerifierC instance
fn create_verifier() -> Box<VerifierC> {
    // Prepare the input parameters for NewVerifierC
    let k: i64 = 7; // Replace with the actual value
    let n: i64 = 1024 * 1024; // Replace with the actual value
    let d: i64 = 64; // Replace with the actual value

    let rsa_n: i64 = 10;
    let rsa_g: i64 = 20;

    Box::new(VerifierC {
        key: RsaKeyC { n: rsa_n, g: rsa_g },
        expanders: ExpandersC { k, n, d, size: 0 },
        nodes: std::ptr::null_mut(),
        nodes_count: 0,
    })
}

fn register_prover_node() {
    // Load the Go dynamic library
    let lib = load_library();
    unsafe {
        let call_register_prover_node: Symbol<CallRegisterProverNodeFunc> = lib
            .get(b"CallRegisterProverNode")
            .expect("Failed to get symbol");

        // Create a VerifierC instance
        let mut verifier_c = create_verifier();

        let mut id: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        // Call NewVerifierC by passing the pointer to verifier_c
        call_register_prover_node(&mut *verifier_c, id.as_mut_ptr(), id.len() as i32);

        // // Access the fields of the VerifierC struct if needed
        // let verifier = &*verifier_c;

        // // Example: Print the values of the VerifierC struct fields
        // println!("VerifierC key n: {:?}", verifier.key.n);
        // println!("VerifierC expanders k: {}", verifier.expanders.k);
        // println!("VerifierC nodes count: {}", verifier.nodes_count);

        // Cleanup the VerifierC struct
        std::mem::drop(verifier_c);
    }
}

fn receive_commits() {
    // Load the Go dynamic library
    let lib = load_library();
    unsafe {
        // Get the symbol for the CallReceiveCommits function
        let call_receive_commits: Symbol<CallReceiveCommitsFunc> =
            lib.get(b"CallReceiveCommits").expect("Failed to get symbol");

        // Prepare the input parameters for CallReceiveCommits
        let mut verifier_c = create_verifier();
        let mut id: [u8; 4] = [0x01, 0x02, 0x03, 0x04];

        // Prepare sample values for CommitC
        let commit1 = CommitC {
            file_index: 0,
            roots: std::ptr::null_mut(),
            roots_count: 0,
        };
        let commit2 = CommitC {
            file_index: 1,
            roots: std::ptr::null_mut(),
            roots_count: 0,
        };
        let mut commits: [*mut CommitC; 2] = [&commit1 as *const _ as *mut _, &commit2 as *const _ as *mut _];
        let commits_buf: *mut *mut CommitC = commits.as_mut_ptr();
        let commits_count: c_int = commits.len() as c_int;

        // Call the CallReceiveCommits function
        let result = call_receive_commits(&mut *verifier_c, id.as_mut_ptr(), id.len() as i32, commits_buf, commits_count);

        // Process the result
        if result != 0 {
            println!("ReceiveCommits succeeded");
        } else {
            println!("ReceiveCommits failed");
        }

        // Cleanup the verifier_c memory
        std::mem::drop(verifier_c);
    }
}

fn call_commit_challenges() {
    // Load the Go dynamic library
    let lib = load_library();
    unsafe {
        // Get the symbol for the CallCommitChallenges function
        let call_commit_challenges: Symbol<CallCommitChallengesFunc> =
            lib.get(b"CallCommitChallenges").expect("Failed to get symbol");

        // Prepare the input parameters for CallCommitChallenges
        let mut verifier_c = create_verifier();
        let id: *mut c_uchar = std::ptr::null_mut();
        let id_length: c_int = 0;
        let left: c_int = 0;
        let right: c_int = 10;

        // Call the CallCommitChallenges function
        let result = call_commit_challenges(&mut *verifier_c, id, id_length, left, right);

        // Process the result
        if result != 0 {
            println!("CommitChallenges succeeded");
        } else {
            println!("CommitChallenges failed");
        }
    }
}

fn space_challenges() {
    // Load the Go dynamic library
    let lib = load_library();
    unsafe {
        // Get the symbol for the CallSpaceChallenges function
        let call_space_challenges: Symbol<CallSpaceChallengesFunc> =
            lib.get(b"CallSpaceChallenges").expect("Failed to get symbol");

        // Prepare the input parameters for CallSpaceChallenges
        let mut verifier_c = create_verifier();
        let id: *mut c_uchar = std::ptr::null_mut();
        let id_length: c_int = 0;
        let param: c_long = 42;

        // Call the CallSpaceChallenges function
        let result = call_space_challenges(&mut *verifier_c, id, id_length, param);

        // Process the result
        if result != 0 {
            println!("SpaceChallenges succeeded");
        } else {
            println!("SpaceChallenges failed");
        }
    }
}

fn call_perform_pois(){
    // Load the Go dynamic library
    let lib = load_library();
    unsafe {
        // Get the symbol for the PerformPois function
        let perform_pois: Symbol<PerformPoisFunc> =
            lib.get(b"PerformPois").expect("Failed to get symbol");

        let rsa_n: i64 = 10;
        let rsa_g: i64 = 20;
        let mut rsa_key_c = RsaKeyC { n: rsa_n, g: rsa_g };
        let k: i64 = 7; // Replace with the actual value
        let n: i64 = 1024 * 1024; // Replace with the actual value
        let d: i64 = 64; // Replace with the actual value


        perform_pois(&mut rsa_key_c, k, n, d);
    }
}

fn main() {
    // register_prover_node();
    // receive_commits();
    // call_commit_challenges();
    // space_challenges();
    call_perform_pois();
}
