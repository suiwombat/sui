//! `expand_message_xmd` based on a hash function.

// TODO(tarcieri): checked arithmetic
#![allow(clippy::integer_arithmetic)]

use core::marker::PhantomData;

use super::{Domain, ExpandMsg, Expander};
use crate::{Error, Result};
use digest::{
    core_api::BlockSizeUser,
    generic_array::{
        typenum::{IsLess, IsLessOrEqual, Unsigned, U256},
        GenericArray,
    },
    FixedOutput, HashMarker,
};

/// Placeholder type for implementing `expand_message_xmd` based on a hash function
///
/// # Errors
/// - `dst.is_empty()`
/// - `len_in_bytes == 0`
/// - `len_in_bytes > u16::MAX`
/// - `len_in_bytes > 255 * HashT::OutputSize`
pub struct ExpandMsgXmd<HashT>(PhantomData<HashT>)
where
    HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
    HashT::OutputSize: IsLess<U256>,
    HashT::OutputSize: IsLessOrEqual<HashT::BlockSize>;

/// ExpandMsgXmd implements expand_message_xmd for the ExpandMsg trait
impl<'a, HashT> ExpandMsg<'a> for ExpandMsgXmd<HashT>
where
    HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
    // If `len_in_bytes` is bigger then 256, length of the `DST` will depend on
    // the output size of the hash, which is still not allowed to be bigger then 256:
    // https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-13.html#section-5.4.1-6
    HashT::OutputSize: IsLess<U256>,
    // Constraint set by `expand_message_xmd`:
    // https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-13.html#section-5.4.1-4
    HashT::OutputSize: IsLessOrEqual<HashT::BlockSize>,
{
    type Expander = ExpanderXmd<'a, HashT>;

    fn expand_message(
        msgs: &[&[u8]],
        dsts: &'a [&'a [u8]],
        len_in_bytes: usize,
    ) -> Result<Self::Expander> {
        if len_in_bytes == 0 {
            return Err(Error);
        }

        let len_in_bytes_u16 = u16::try_from(len_in_bytes).map_err(|_| Error)?;

        let b_in_bytes = HashT::OutputSize::to_usize();
        let ell = u8::try_from((len_in_bytes + b_in_bytes - 1) / b_in_bytes).map_err(|_| Error)?;

        let domain = Domain::xmd::<HashT>(dsts)?;
        let mut b_0 = HashT::default();
        b_0.update(&GenericArray::<u8, HashT::BlockSize>::default());

        for msg in msgs {
            b_0.update(msg);
        }

        b_0.update(&len_in_bytes_u16.to_be_bytes());
        b_0.update(&[0]);
        domain.update_hash(&mut b_0);
        b_0.update(&[domain.len()]);
        let b_0 = b_0.finalize_fixed();

        let mut b_vals = HashT::default();
        b_vals.update(&b_0[..]);
        b_vals.update(&[1u8]);
        domain.update_hash(&mut b_vals);
        b_vals.update(&[domain.len()]);
        let b_vals = b_vals.finalize_fixed();

        Ok(ExpanderXmd {
            b_0,
            b_vals,
            domain,
            index: 1,
            offset: 0,
            ell,
        })
    }
}

/// [`Expander`] type for [`ExpandMsgXmd`].
pub struct ExpanderXmd<'a, HashT>
where
    HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
    HashT::OutputSize: IsLess<U256>,
    HashT::OutputSize: IsLessOrEqual<HashT::BlockSize>,
{
    b_0: GenericArray<u8, HashT::OutputSize>,
    b_vals: GenericArray<u8, HashT::OutputSize>,
    domain: Domain<'a, HashT::OutputSize>,
    index: u8,
    offset: usize,
    ell: u8,
}

impl<'a, HashT> ExpanderXmd<'a, HashT>
where
    HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
    HashT::OutputSize: IsLess<U256>,
    HashT::OutputSize: IsLessOrEqual<HashT::BlockSize>,
{
    fn next(&mut self) -> bool {
        if self.index < self.ell {
            self.index += 1;
            self.offset = 0;
            // b_0 XOR b_(idx - 1)
            let mut tmp = GenericArray::<u8, HashT::OutputSize>::default();
            self.b_0
                .iter()
                .zip(&self.b_vals[..])
                .enumerate()
                .for_each(|(j, (b0val, bi1val))| tmp[j] = b0val ^ bi1val);
            let mut b_vals = HashT::default();
            b_vals.update(&tmp);
            b_vals.update(&[self.index]);
            self.domain.update_hash(&mut b_vals);
            b_vals.update(&[self.domain.len()]);
            self.b_vals = b_vals.finalize_fixed();
            true
        } else {
            false
        }
    }
}

impl<'a, HashT> Expander for ExpanderXmd<'a, HashT>
where
    HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
    HashT::OutputSize: IsLess<U256>,
    HashT::OutputSize: IsLessOrEqual<HashT::BlockSize>,
{
    fn fill_bytes(&mut self, okm: &mut [u8]) {
        for b in okm {
            if self.offset == self.b_vals.len() && !self.next() {
                return;
            }
            *b = self.b_vals[self.offset];
            self.offset += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::mem;
    use generic_array::{
        typenum::{U128, U32},
        ArrayLength,
    };
    use hex_literal::hex;
    use sha2::Sha256;

    fn assert_message<HashT>(
        msg: &[u8],
        domain: &Domain<'_, HashT::OutputSize>,
        len_in_bytes: u16,
        bytes: &[u8],
    ) where
        HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
        HashT::OutputSize: IsLess<U256>,
    {
        let block = HashT::BlockSize::to_usize();
        assert_eq!(
            GenericArray::<u8, HashT::BlockSize>::default().as_slice(),
            &bytes[..block]
        );

        let msg_len = block + msg.len();
        assert_eq!(msg, &bytes[block..msg_len]);

        let l = msg_len + mem::size_of::<u16>();
        assert_eq!(len_in_bytes.to_be_bytes(), &bytes[msg_len..l]);

        let pad = l + mem::size_of::<u8>();
        assert_eq!([0], &bytes[l..pad]);

        let dst = pad + usize::from(domain.len());
        domain.assert(&bytes[pad..dst]);

        let dst_len = dst + mem::size_of::<u8>();
        assert_eq!([domain.len()], &bytes[dst..dst_len]);

        assert_eq!(dst_len, bytes.len());
    }

    struct TestVector {
        msg: &'static [u8],
        msg_prime: &'static [u8],
        uniform_bytes: &'static [u8],
    }

    impl TestVector {
        fn assert<HashT, L: ArrayLength<u8>>(
            &self,
            dst: &'static [u8],
            domain: &Domain<'_, HashT::OutputSize>,
        ) -> Result<()>
        where
            HashT: BlockSizeUser + Default + FixedOutput + HashMarker,
            HashT::OutputSize: IsLess<U256> + IsLessOrEqual<HashT::BlockSize>,
        {
            assert_message::<HashT>(self.msg, domain, L::to_u16(), self.msg_prime);

            let dst = [dst];
            let mut expander =
                ExpandMsgXmd::<HashT>::expand_message(&[self.msg], &dst, L::to_usize())?;

            let mut uniform_bytes = GenericArray::<u8, L>::default();
            expander.fill_bytes(&mut uniform_bytes);

            assert_eq!(uniform_bytes.as_slice(), self.uniform_bytes);
            Ok(())
        }
    }

    #[test]
    fn expand_message_xmd_sha_256() -> Result<()> {
        const DST: &[u8] = b"QUUX-V01-CS02-with-expander-SHA256-128";
        const DST_PRIME: &[u8] =
            &hex!("515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826");

        let dst_prime = Domain::xmd::<Sha256>(&[DST])?;
        dst_prime.assert_dst(DST_PRIME);

        const TEST_VECTORS_32: &[TestVector] = &[
            TestVector {
                msg: b"",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("68a985b87eb6b46952128911f2a4412bbc302a9d759667f87f7a21d803f07235"),
            },
            TestVector {
                msg: b"abc",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000616263002000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("d8ccab23b5985ccea865c6c97b6e5b8350e794e603b4b97902f53a8a0d605615"),
            },
            TestVector {
                msg: b"abcdef0123456789",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000061626364656630313233343536373839002000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("eff31487c770a893cfb36f912fbfcbff40d5661771ca4b2cb4eafe524333f5c1"),
            },
            TestVector {
                msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000713132385f7171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171002000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("b23a1d2b4d97b2ef7785562a7e8bac7eed54ed6e97e29aa51bfe3f12ddad1ff9"),
            },
            TestVector {
                msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000613531325f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161002000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("4623227bcc01293b8c130bf771da8c298dede7383243dc0993d2d94823958c4c"),
            },
        ];

        for test_vector in TEST_VECTORS_32 {
            test_vector.assert::<Sha256, U32>(DST, &dst_prime)?;
        }

        const TEST_VECTORS_128: &[TestVector] = &[
            TestVector {
                msg: b"",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("af84c27ccfd45d41914fdff5df25293e221afc53d8ad2ac06d5e3e29485dadbee0d121587713a3e0dd4d5e69e93eb7cd4f5df4cd103e188cf60cb02edc3edf18eda8576c412b18ffb658e3dd6ec849469b979d444cf7b26911a08e63cf31f9dcc541708d3491184472c2c29bb749d4286b004ceb5ee6b9a7fa5b646c993f0ced"),
            },            TestVector {
                msg: b"abc",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000616263008000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("abba86a6129e366fc877aab32fc4ffc70120d8996c88aee2fe4b32d6c7b6437a647e6c3163d40b76a73cf6a5674ef1d890f95b664ee0afa5359a5c4e07985635bbecbac65d747d3d2da7ec2b8221b17b0ca9dc8a1ac1c07ea6a1e60583e2cb00058e77b7b72a298425cd1b941ad4ec65e8afc50303a22c0f99b0509b4c895f40"),
            },            TestVector {
                msg: b"abcdef0123456789",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000061626364656630313233343536373839008000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("ef904a29bffc4cf9ee82832451c946ac3c8f8058ae97d8d629831a74c6572bd9ebd0df635cd1f208e2038e760c4994984ce73f0d55ea9f22af83ba4734569d4bc95e18350f740c07eef653cbb9f87910d833751825f0ebefa1abe5420bb52be14cf489b37fe1a72f7de2d10be453b2c9d9eb20c7e3f6edc5a60629178d9478df"),
            },            TestVector {
                msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000713132385f7171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171008000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("80be107d0884f0d881bb460322f0443d38bd222db8bd0b0a5312a6fedb49c1bbd88fd75d8b9a09486c60123dfa1d73c1cc3169761b17476d3c6b7cbbd727acd0e2c942f4dd96ae3da5de368d26b32286e32de7e5a8cb2949f866a0b80c58116b29fa7fabb3ea7d520ee603e0c25bcaf0b9a5e92ec6a1fe4e0391d1cdbce8c68a"),
            },            TestVector {
                msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000613531325f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161008000515555582d5630312d435330322d776974682d657870616e6465722d5348413235362d31323826"),
                uniform_bytes: &hex!("546aff5444b5b79aa6148bd81728704c32decb73a3ba76e9e75885cad9def1d06d6792f8a7d12794e90efed817d96920d728896a4510864370c207f99bd4a608ea121700ef01ed879745ee3e4ceef777eda6d9e5e38b90c86ea6fb0b36504ba4a45d22e86f6db5dd43d98a294bebb9125d5b794e9d2a81181066eb954966a487"),
            },
        ];

        for test_vector in TEST_VECTORS_128 {
            test_vector.assert::<Sha256, U128>(DST, &dst_prime)?;
        }

        Ok(())
    }

    #[test]
    fn expand_message_xmd_sha_256_long() -> Result<()> {
        const DST: &[u8] = b"QUUX-V01-CS02-with-expander-SHA256-128-long-DST-1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
        const DST_PRIME: &[u8] =
            &hex!("412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620");

        let dst_prime = Domain::xmd::<Sha256>(&[DST])?;
        dst_prime.assert_dst(DST_PRIME);

        const TEST_VECTORS_32: &[TestVector] = &[
            TestVector {
                msg: b"",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("e8dc0c8b686b7ef2074086fbdd2f30e3f8bfbd3bdf177f73f04b97ce618a3ed3"),
            },
            TestVector {
                msg: b"abc",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000616263002000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("52dbf4f36cf560fca57dedec2ad924ee9c266341d8f3d6afe5171733b16bbb12"),
            },
            TestVector {
                msg: b"abcdef0123456789",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000061626364656630313233343536373839002000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("35387dcf22618f3728e6c686490f8b431f76550b0b2c61cbc1ce7001536f4521"),
            },
            TestVector {
                msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000713132385f7171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171002000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("01b637612bb18e840028be900a833a74414140dde0c4754c198532c3a0ba42bc"),
            },
            TestVector {
                msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000613531325f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161002000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("20cce7033cabc5460743180be6fa8aac5a103f56d481cf369a8accc0c374431b"),
            },
        ];

        for test_vector in TEST_VECTORS_32 {
            test_vector.assert::<Sha256, U32>(DST, &dst_prime)?;
        }

        const TEST_VECTORS_128: &[TestVector] = &[
            TestVector {
                msg: b"",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("14604d85432c68b757e485c8894db3117992fc57e0e136f71ad987f789a0abc287c47876978e2388a02af86b1e8d1342e5ce4f7aaa07a87321e691f6fba7e0072eecc1218aebb89fb14a0662322d5edbd873f0eb35260145cd4e64f748c5dfe60567e126604bcab1a3ee2dc0778102ae8a5cfd1429ebc0fa6bf1a53c36f55dfc"),
            },
            TestVector {
                msg: b"abc",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000616263008000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("1a30a5e36fbdb87077552b9d18b9f0aee16e80181d5b951d0471d55b66684914aef87dbb3626eaabf5ded8cd0686567e503853e5c84c259ba0efc37f71c839da2129fe81afdaec7fbdc0ccd4c794727a17c0d20ff0ea55e1389d6982d1241cb8d165762dbc39fb0cee4474d2cbbd468a835ae5b2f20e4f959f56ab24cd6fe267"),
            },
            TestVector {
                msg: b"abcdef0123456789",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000061626364656630313233343536373839008000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("d2ecef3635d2397f34a9f86438d772db19ffe9924e28a1caf6f1c8f15603d4028f40891044e5c7e39ebb9b31339979ff33a4249206f67d4a1e7c765410bcd249ad78d407e303675918f20f26ce6d7027ed3774512ef5b00d816e51bfcc96c3539601fa48ef1c07e494bdc37054ba96ecb9dbd666417e3de289d4f424f502a982"),
            },
            TestVector {
                msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000713132385f7171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171008000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("ed6e8c036df90111410431431a232d41a32c86e296c05d426e5f44e75b9a50d335b2412bc6c91e0a6dc131de09c43110d9180d0a70f0d6289cb4e43b05f7ee5e9b3f42a1fad0f31bac6a625b3b5c50e3a83316783b649e5ecc9d3b1d9471cb5024b7ccf40d41d1751a04ca0356548bc6e703fca02ab521b505e8e45600508d32"),
            },
            TestVector {
                msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                msg_prime: &hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000613531325f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161008000412717974da474d0f8c420f320ff81e8432adb7c927d9bd082b4fb4d16c0a23620"),
                uniform_bytes: &hex!("78b53f2413f3c688f07732c10e5ced29a17c6a16f717179ffbe38d92d6c9ec296502eb9889af83a1928cd162e845b0d3c5424e83280fed3d10cffb2f8431f14e7a23f4c68819d40617589e4c41169d0b56e0e3535be1fd71fbb08bb70c5b5ffed953d6c14bf7618b35fc1f4c4b30538236b4b08c9fbf90462447a8ada60be495"),
            },
        ];

        for test_vector in TEST_VECTORS_128 {
            test_vector.assert::<Sha256, U128>(DST, &dst_prime)?;
        }

        Ok(())
    }

    #[test]
    fn expand_message_xmd_sha_512() -> Result<()> {
        use sha2::Sha512;

        const DST: &[u8] = b"QUUX-V01-CS02-with-expander-SHA512-256";
        const DST_PRIME: &[u8] =
            &hex!("515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626");

        let dst_prime = Domain::xmd::<Sha512>(&[DST])?;
        dst_prime.assert_dst(DST_PRIME);

        const TEST_VECTORS_32: &[TestVector] = &[
            TestVector {
                msg: b"",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("6b9a7312411d92f921c6f68ca0b6380730a1a4d982c507211a90964c394179ba"),
            },
            TestVector {
                msg: b"abc",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000616263002000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("0da749f12fbe5483eb066a5f595055679b976e93abe9be6f0f6318bce7aca8dc"),
            },
            TestVector {
                msg: b"abcdef0123456789",
                msg_prime: &hex!("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000061626364656630313233343536373839002000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("087e45a86e2939ee8b91100af1583c4938e0f5fc6c9db4b107b83346bc967f58"),
            },
            TestVector {
                msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000713132385f7171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171002000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("7336234ee9983902440f6bc35b348352013becd88938d2afec44311caf8356b3"),
            },
            TestVector {
                msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000613531325f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161002000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("57b5f7e766d5be68a6bfe1768e3c2b7f1228b3e4b3134956dd73a59b954c66f4"),
            },
        ];

        for test_vector in TEST_VECTORS_32 {
            test_vector.assert::<Sha512, U32>(DST, &dst_prime)?;
        }

        const TEST_VECTORS_128: &[TestVector] = &[
            TestVector {
                msg: b"",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("41b037d1734a5f8df225dd8c7de38f851efdb45c372887be655212d07251b921b052b62eaed99b46f72f2ef4cc96bfaf254ebbbec091e1a3b9e4fb5e5b619d2e0c5414800a1d882b62bb5cd1778f098b8eb6cb399d5d9d18f5d5842cf5d13d7eb00a7cff859b605da678b318bd0e65ebff70bec88c753b159a805d2c89c55961"),
            },
            TestVector {
                msg: b"abc",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000616263008000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("7f1dddd13c08b543f2e2037b14cefb255b44c83cc397c1786d975653e36a6b11bdd7732d8b38adb4a0edc26a0cef4bb45217135456e58fbca1703cd6032cb1347ee720b87972d63fbf232587043ed2901bce7f22610c0419751c065922b488431851041310ad659e4b23520e1772ab29dcdeb2002222a363f0c2b1c972b3efe1"),
            },
            TestVector {
                msg: b"abcdef0123456789",
                msg_prime: &hex!("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000061626364656630313233343536373839008000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("3f721f208e6199fe903545abc26c837ce59ac6fa45733f1baaf0222f8b7acb0424814fcb5eecf6c1d38f06e9d0a6ccfbf85ae612ab8735dfdf9ce84c372a77c8f9e1c1e952c3a61b7567dd0693016af51d2745822663d0c2367e3f4f0bed827feecc2aaf98c949b5ed0d35c3f1023d64ad1407924288d366ea159f46287e61ac"),
            },
            TestVector {
                msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000713132385f7171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171717171008000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("b799b045a58c8d2b4334cf54b78260b45eec544f9f2fb5bd12fb603eaee70db7317bf807c406e26373922b7b8920fa29142703dd52bdf280084fb7ef69da78afdf80b3586395b433dc66cde048a258e476a561e9deba7060af40adf30c64249ca7ddea79806ee5beb9a1422949471d267b21bc88e688e4014087a0b592b695ed"),
            },
            TestVector {
                msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                msg_prime: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000613531325f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161008000515555582d5630312d435330322d776974682d657870616e6465722d5348413531322d32353626"),
                uniform_bytes: &hex!("05b0bfef265dcee87654372777b7c44177e2ae4c13a27f103340d9cd11c86cb2426ffcad5bd964080c2aee97f03be1ca18e30a1f14e27bc11ebbd650f305269cc9fb1db08bf90bfc79b42a952b46daf810359e7bc36452684784a64952c343c52e5124cd1f71d474d5197fefc571a92929c9084ffe1112cf5eea5192ebff330b"),
            },
        ];

        for test_vector in TEST_VECTORS_128 {
            test_vector.assert::<Sha512, U128>(DST, &dst_prime)?;
        }

        Ok(())
    }
}
