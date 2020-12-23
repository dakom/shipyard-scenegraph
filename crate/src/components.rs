use shipyard::EntityId;
use core::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};
use crate::traits::math::*;

macro_rules! makeComponent {
    ($name:ident, $data:ident)=> {

        #[derive(Debug)]
        pub struct $name<T, N> 
        where 
            T: $data<N>,
            N: Copy,
        {
            _values: T,
            phantom: PhantomData<N>
        }

        impl <T, N> $name<T, N> 
        where 
            T: $data<N>,
            N: Copy,

        {
            pub fn new(values:T) -> Self {
                Self {
                    _values: values,
                    phantom: PhantomData
                }
            }

        }
        
        impl <T, N> Borrow<T> for $name<T, N> 
        where 
            T: $data<N>,
            N: Copy,
        {
            fn borrow(&self) -> &T {
                &self._values
            }
        }

        impl <T, N> BorrowMut<T> for $name<T, N> 
        where 
            T: $data<N>,
            N: Copy,
        {
            fn borrow_mut(&mut self) -> &mut T {
                &mut self._values
            }
        }

        impl <T, N> Deref for $name<T, N> 
        where 
            T: $data<N>,
            N: Copy,
        {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self._values
            }
        }


        impl <T, N> DerefMut for $name<T, N> 
        where 
            T: $data<N>,
            N: Copy,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self._values
            }
        }
    };
}

pub struct TransformRoot(pub EntityId);
pub struct DirtyTransform(pub bool);

makeComponent!(Translation, Vec3);
makeComponent!(Rotation, Quat);
makeComponent!(Scale, Vec3);
makeComponent!(Origin, Vec3);
makeComponent!(LocalTransform, Matrix4);
makeComponent!(WorldTransform, Matrix4);