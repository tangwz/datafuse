//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use common_exception::Result;
use futures::stream::Stream;
use futures::AsyncRead;
use futures::AsyncSeek;
pub type Bytes = Vec<u8>;

#[async_trait::async_trait]
pub trait DataAccessor {
    type InputStream: AsyncRead + AsyncSeek;

    async fn get_input_stream(
        &self,
        path: &str,
        stream_len: Option<u64>,
    ) -> Result<Self::InputStream>;

    async fn get(&self, path: &str) -> Result<Bytes>;

    async fn put(&self, path: &str, content: Vec<u8>) -> common_exception::Result<()>;

    async fn put_stream<S>(
        &self,
        path: &str,
        input_stream: S,
        stream_len: usize,
    ) -> common_exception::Result<()>
    where
        S: Stream<Item = std::result::Result<Bytes, std::io::Error>> + Send + 'static;
}
