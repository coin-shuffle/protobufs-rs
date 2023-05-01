// @generated
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod shuffle_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct ShuffleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShuffleServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ShuffleServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ShuffleServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ShuffleServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///
        pub async fn join_shuffle_room(
            &mut self,
            request: impl tonic::IntoRequest<super::JoinShuffleRoomRequest>,
        ) -> Result<tonic::Response<super::JoinShuffleRoomResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/coin_shuffle.v1.ShuffleService/JoinShuffleRoom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Protected with token got from `JoinShuffleRoom` in header
*/
        pub async fn is_ready_for_shuffle(
            &mut self,
            request: impl tonic::IntoRequest<super::IsReadyForShuffleRequest>,
        ) -> Result<tonic::Response<super::IsReadyForShuffleResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/coin_shuffle.v1.ShuffleService/IsReadyForShuffle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Protected with token got from `JoinShuffleRoom` in header
*/
        pub async fn connect_shuffle_room(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectShuffleRoomRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ShuffleEvent>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/coin_shuffle.v1.ShuffleService/ConnectShuffleRoom",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /** Protected with token got from `ConnectShuffleRoom` from event `ShuffleInfo` in header.
*/
        pub async fn shuffle_round(
            &mut self,
            request: impl tonic::IntoRequest<super::ShuffleRoundRequest>,
        ) -> Result<tonic::Response<super::ShuffleRoundResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/coin_shuffle.v1.ShuffleService/ShuffleRound",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Protected with token got from `ConnectShuffleRoom` from event `ShuffleInfo` in header.
*/
        pub async fn sign_shuffle_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::SignShuffleTxRequest>,
        ) -> Result<tonic::Response<super::SignShuffleTxResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/coin_shuffle.v1.ShuffleService/SignShuffleTx",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod shuffle_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ShuffleServiceServer.
    #[async_trait]
    pub trait ShuffleService: Send + Sync + 'static {
        ///
        async fn join_shuffle_room(
            &self,
            request: tonic::Request<super::JoinShuffleRoomRequest>,
        ) -> Result<tonic::Response<super::JoinShuffleRoomResponse>, tonic::Status>;
        /** Protected with token got from `JoinShuffleRoom` in header
*/
        async fn is_ready_for_shuffle(
            &self,
            request: tonic::Request<super::IsReadyForShuffleRequest>,
        ) -> Result<tonic::Response<super::IsReadyForShuffleResponse>, tonic::Status>;
        /// Server streaming response type for the ConnectShuffleRoom method.
        type ConnectShuffleRoomStream: futures_core::Stream<
                Item = Result<super::ShuffleEvent, tonic::Status>,
            >
            + Send
            + 'static;
        /** Protected with token got from `JoinShuffleRoom` in header
*/
        async fn connect_shuffle_room(
            &self,
            request: tonic::Request<super::ConnectShuffleRoomRequest>,
        ) -> Result<tonic::Response<Self::ConnectShuffleRoomStream>, tonic::Status>;
        /** Protected with token got from `ConnectShuffleRoom` from event `ShuffleInfo` in header.
*/
        async fn shuffle_round(
            &self,
            request: tonic::Request<super::ShuffleRoundRequest>,
        ) -> Result<tonic::Response<super::ShuffleRoundResponse>, tonic::Status>;
        /** Protected with token got from `ConnectShuffleRoom` from event `ShuffleInfo` in header.
*/
        async fn sign_shuffle_tx(
            &self,
            request: tonic::Request<super::SignShuffleTxRequest>,
        ) -> Result<tonic::Response<super::SignShuffleTxResponse>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct ShuffleServiceServer<T: ShuffleService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ShuffleService> ShuffleServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ShuffleServiceServer<T>
    where
        T: ShuffleService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/coin_shuffle.v1.ShuffleService/JoinShuffleRoom" => {
                    #[allow(non_camel_case_types)]
                    struct JoinShuffleRoomSvc<T: ShuffleService>(pub Arc<T>);
                    impl<
                        T: ShuffleService,
                    > tonic::server::UnaryService<super::JoinShuffleRoomRequest>
                    for JoinShuffleRoomSvc<T> {
                        type Response = super::JoinShuffleRoomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JoinShuffleRoomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).join_shuffle_room(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = JoinShuffleRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/coin_shuffle.v1.ShuffleService/IsReadyForShuffle" => {
                    #[allow(non_camel_case_types)]
                    struct IsReadyForShuffleSvc<T: ShuffleService>(pub Arc<T>);
                    impl<
                        T: ShuffleService,
                    > tonic::server::UnaryService<super::IsReadyForShuffleRequest>
                    for IsReadyForShuffleSvc<T> {
                        type Response = super::IsReadyForShuffleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsReadyForShuffleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).is_ready_for_shuffle(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsReadyForShuffleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/coin_shuffle.v1.ShuffleService/ConnectShuffleRoom" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectShuffleRoomSvc<T: ShuffleService>(pub Arc<T>);
                    impl<
                        T: ShuffleService,
                    > tonic::server::ServerStreamingService<
                        super::ConnectShuffleRoomRequest,
                    > for ConnectShuffleRoomSvc<T> {
                        type Response = super::ShuffleEvent;
                        type ResponseStream = T::ConnectShuffleRoomStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectShuffleRoomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).connect_shuffle_room(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectShuffleRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/coin_shuffle.v1.ShuffleService/ShuffleRound" => {
                    #[allow(non_camel_case_types)]
                    struct ShuffleRoundSvc<T: ShuffleService>(pub Arc<T>);
                    impl<
                        T: ShuffleService,
                    > tonic::server::UnaryService<super::ShuffleRoundRequest>
                    for ShuffleRoundSvc<T> {
                        type Response = super::ShuffleRoundResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShuffleRoundRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).shuffle_round(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShuffleRoundSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/coin_shuffle.v1.ShuffleService/SignShuffleTx" => {
                    #[allow(non_camel_case_types)]
                    struct SignShuffleTxSvc<T: ShuffleService>(pub Arc<T>);
                    impl<
                        T: ShuffleService,
                    > tonic::server::UnaryService<super::SignShuffleTxRequest>
                    for SignShuffleTxSvc<T> {
                        type Response = super::SignShuffleTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignShuffleTxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).sign_shuffle_tx(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignShuffleTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ShuffleService> Clone for ShuffleServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ShuffleService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ShuffleService> tonic::server::NamedService for ShuffleServiceServer<T> {
        const NAME: &'static str = "coin_shuffle.v1.ShuffleService";
    }
}
