To add JSON-RPC functionality to a new "XYZ" pallet within the canvas-node codebase:

1.  Update runtime create_... function(s) (e.g., create_full):
    node/rpc.rs --or-- node/runtime/rpc/src/lib.rs:

        // add an API trait bound to the "where <<trait bounds>>" section at the top of the create_... function(s)
        C::Api: XYZ_rpc::XYZRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,

        // add this toward the bottom of the create_... function(s)
        // XYZ RPC API extension
        io.extend_with(
            XYZApi::to_delegate(XYZ::new(client.clone()))
        );

2.  Define a request struct for each endpoint (as in substrate/frame/contracts/rpc/src/lib.rs):
    pallets/rpc/src/common.rs:

        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        pub struct XYZEndpoint1Request {...}

3.  Define a JSON-RPC Result struct for each endpoint (as in substrate/frame/contracts/common/src/lib.rs):
    pallets/rpc/src/common.rs:

        pub type XYZEndpoint1Result = Result<..., ...>;

4.  As necessary, define a JSON-RPC Error for each internal error type (as in substrate/frame/contracts/rpc/src/lib.rs):
    pallets/rpc/src/common.rs:

        const ERROR_A_HARDCODE: i64 = 1;
        const ERROR_B_HARDCODE: i64 = 2;
        
        #[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
        pub enum XYZError1 {
            XYZErrorA,
            XYZErrorB,
        }

        impl From<XYZError1> for Error {
            fn from(e: XYZError1) -> Error {
                match e {
                    XYZErrorA => Error {
                        code: ErrorCode::ServerError(ERROR_A_HARDCODE),
                        message: "error A description".into(),
                        data: None,
        			},
                    XYZErrorB => Error {
                        code: ErrorCode::ServerError(ERROR_B_HARDCODE),
                        message: "error B description".into(),
                        data: None,
        			},
                }
            }
        }

5.  Define RPC API within XYZApi trait (as in substrate/frame/contracts/rpc/src/lib.rs) with RPC derive macros (which can be seen in jsonrpc/derive/src/lib.rs):
    pallets/rpc/src/lib.rs:

        #[rpc]
        pub trait XYZRPCApi<BlockHash, BlockNumber, AccountId, Balance, Hash> {
            
            #[rpc(name = "XYZ_endpoint_1")]
            fn endpoint_1(&self, endpoint_1_request: XYZEndpoint1Request, ...) -> Result<XYZEndpoint1Result>;
            
            #[rpc(name = "XYZ_endpoint_2")]
            fn endpoint_2(&self, endpoint_2_request: XYZEndpoint2Request, ...) -> Result<XYZEndpoint2Result>;
        }

6.  Add and populate a XYZ<C, B> struct (as in substrate/frame/contracts/rpc/src/lib.rs), where C is a Config type, B is a sp_runtime::traits::Block type:
    pallets/rpc/src/lib.rs:

        impl XYZ<C, B> {
            pub fn new(client: Arc<C>) {...}
        }

        impl XYZRPCApi for XYZ {
            fn endpoint_1(&self, ...) {
                let api = self.client.runtime_api();
                ...
                let exec_result = api.endpoint_1(...).map_err(...);
                Ok(exec_result)
            }
        }

7.  Add the runtime API trait using the decl_runtime_apis macro (as in frame/contracts/rpc/runtime-api/src/lib.rs):
    pallets/rpc/src/runtime_api.rs:

        sp_api::decl_runtime_apis! {
            /// The API to interact with contracts without using executive.
            pub trait XYZRuntimeApi<GenParam1, GenParam2, ...> where
                GenParam1: Codec,
                GenParam2: Codec,
                ...,
            {

                fn endpoint_1(...) -> XYZResult1<...>;
                fn endpoint_2(...) -> XYZResult2<...>;
            }
        }

8.  Add the runtime API implementation:

        