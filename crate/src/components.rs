use crate::traits::required::*;
use core::marker::PhantomData;
use shipyard::{track, Component, EntityId};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

macro_rules! makeComponent {
    ($name:ident, $data:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name<T, N>
        where
            T: $data<N>,
            N: Copy,
        {
            _values: T,
            phantom: PhantomData<N>,
        }

        impl<T, N> Component for $name<T, N>
        where
            T: $data<N> + 'static,
            N: Copy + 'static,
        {
            type Tracking = track::Modification;
        }

        impl<T, N> $name<T, N>
        where
            T: $data<N>,
            N: Copy,
        {
            pub fn new(values: T) -> Self {
                Self {
                    _values: values,
                    phantom: PhantomData,
                }
            }
        }

        impl<T, N> Borrow<T> for $name<T, N>
        where
            T: $data<N>,
            N: Copy,
        {
            fn borrow(&self) -> &T {
                &self._values
            }
        }

        impl<T, N> BorrowMut<T> for $name<T, N>
        where
            T: $data<N>,
            N: Copy,
        {
            fn borrow_mut(&mut self) -> &mut T {
                &mut self._values
            }
        }

        impl<T, N> Deref for $name<T, N>
        where
            T: $data<N>,
            N: Copy,
        {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self._values
            }
        }

        impl<T, N> DerefMut for $name<T, N>
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

#[derive(Component)]
pub struct TransformRoot(pub EntityId);
#[derive(Component)]
pub struct DirtyTransform(pub bool);

makeComponent!(Translation, Vec3Ext);
makeComponent!(Rotation, QuatExt);
makeComponent!(Scale, Vec3Ext);
makeComponent!(Origin, Vec3Ext);
makeComponent!(LocalTransform, Matrix4Ext);
makeComponent!(WorldTransform, Matrix4Ext);
