// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::prelude::*;

pub trait FromNative {
    type Native;

    fn from_native(native: Self::Native) -> Self;
}

pub trait FromNativeWithNetworkContext {
    type Native;

    fn from_native(native: Self::Native, network_id: u8) -> Self;
}

impl<T> FromNativeWithNetworkContext for T
where
    T: FromNative,
{
    type Native = <T as FromNative>::Native;

    fn from_native(native: Self::Native, _: u8) -> Self {
        <T as FromNative>::from_native(native)
    }
}

pub trait ToNative {
    type Native;

    fn to_native(self) -> Result<Self::Native>;
}

impl<T> ToNative for Vec<T>
where
    T: ToNative,
{
    type Native = Vec<<T as ToNative>::Native>;

    fn to_native(self) -> Result<Self::Native> {
        self.into_iter()
            .map(T::to_native)
            .collect::<Result<Vec<_>>>()
    }
}
