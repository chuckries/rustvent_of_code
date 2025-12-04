#![allow(non_snake_case)]

use windows::Win32::Security::Cryptography::*;
use windows_core::*;
pub struct Md5 {
    handle: BCRYPT_HASH_HANDLE,
    hash_bytes: Vec<u8>,
    hash_length: usize,
}

impl Md5 {
    pub fn new() -> Self {
        let mut hash_handle: BCRYPT_HASH_HANDLE;
        let mut hash_bytes: Vec<u8>;
        let hash_length: usize;
        unsafe {
            let mut provider_handle: BCRYPT_ALG_HANDLE = Default::default();
            let name: HSTRING = "MD5".into();
            BCryptOpenAlgorithmProvider(&mut provider_handle, &name, None, BCRYPT_HASH_REUSABLE_FLAG).ok().unwrap();

            let mut cbResult: u32 = 0;
            let mut bOutput: [u8; 4] = [0; 4];
            BCryptGetProperty(provider_handle, BCRYPT_OBJECT_LENGTH, Some(&mut bOutput), &mut cbResult, 0).ok().unwrap();
            let hash_size: u32 = u32::from_le_bytes(bOutput);

            let mut bOutput: [u8; 4] = [0; 4];
            BCryptGetProperty(provider_handle, BCRYPT_HASH_LENGTH, Some(&mut bOutput), &mut cbResult, 0).ok().unwrap();
            hash_length = u32::from_le_bytes(bOutput) as usize;

            hash_bytes = vec![0; hash_size as usize];
            hash_handle = Default::default();
            BCryptCreateHash(provider_handle, &mut hash_handle, Some(&mut hash_bytes), None, BCRYPT_HASH_REUSABLE_FLAG.0).ok().unwrap();

            provider_handle.free();
        }
        Self {
            handle: hash_handle,
            hash_bytes,
            hash_length
        }
    }

    pub fn compute<T: AsRef<[u8]>>(&self, data: T) -> Vec<u8> {
        unsafe {
            BCryptHashData(self.handle, data.as_ref(), 0).ok().unwrap();

            let mut result: Vec<u8> = vec![0; self.hash_length];
            BCryptFinishHash(self.handle, &mut result, 0).ok().unwrap();

            result
        }
    }
}

impl Drop for Md5 {
    fn drop(&mut self) {
        unsafe {
            self.handle.free();
        }
        self.hash_bytes.clear();
    }
}

#[cfg(test)]
mod test{
    use super::Md5;

    #[test]
    fn sanity() {
        let md5 = Md5::new();
        let result = md5.compute("hello");
        let s = bytes_to_string(&result);
        assert_eq!(s, "5d41402abc4b2a76b9719d911017c592");
    }

    fn bytes_to_string(bytes: &[u8]) -> String {
        fn byte_to_char(b: u8) -> char {
            (if b < 10 {
                b + b'0'
            } else {
                b - 10 + b'a'
            }) as char
        }

        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            s.push(byte_to_char(b >> 4));
            s.push(byte_to_char(b & 0x0F));
        }
        s
    }
}

