use std::io::{Read, Seek};

use anyhow::Ok;
use bytemuck::{Pod, checked::cast_slice_mut};
use num_traits::PrimInt;

pub trait ReadArrayExt: Read {
    fn read_array<T: Default + Pod + PrimInt, const N: usize>(&mut self) -> anyhow::Result<[T; N]> {
        let mut res = [T::default(); N];
        self.read_exact(cast_slice_mut(&mut res))?;
        res.iter_mut().for_each(|x| *x = x.to_be());
        Ok(res)
    }

    fn read_number<T: Default + Pod + PrimInt>(&mut self) -> anyhow::Result<T> {
        let mut res = [T::default(); 1];
        self.read_exact(cast_slice_mut(&mut res))?;
        res.iter_mut().for_each(|x| *x = x.to_be());
        Ok(res[0])
    }
}

impl<R: Read> ReadArrayExt for R {}

pub trait ParseExt {
    fn read<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self>
    where
        Self: Sized;
}
