use codec::{Encode, Decode};
use serde::{Serialize, Deserialize};
use std::convert::{TryFrom, TryInto};
use jsonrpc_core::{Error, ErrorCode, Result};
use frame_support::{RuntimeDebug};

pub use pallet_contracts_primitives::{
    Code,
    ContractExecResult,
    ContractInstantiateResult,
};

pub use pallet_contracts_rpc::{
    CallRequest,
    InstantiateRequest,
    Contracts,
};

const RUNTIME_ERROR: i64 = 1;
const CONTRACT_DOESNT_EXIST: i64 = 2;

pub type Weight = u64;

/// A rough estimate of how much gas a decent hardware consumes per second,
/// using native execution.
/// This value is used to set the upper bound for maximal contract calls to
/// prevent blocking the RPC for too long.
///
/// As 1 gas is equal to 1 weight we base this on the conducted benchmarks which
/// determined runtime weights:
/// <https://github.com/paritytech/substrate/pull/5446>
const GAS_PER_SECOND: Weight = 1_000_000_000_000;

/// The maximum amount of weight that the call and instantiate rpcs are allowed to consume.
/// This puts a ceiling on the weight limit that is supplied to the rpc as an argument.
const GAS_LIMIT: Weight = 5 * GAS_PER_SECOND;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Encode, Decode)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NamedContract<AccountId> {
    path: Box<String>,
    at: AccountId,
}

#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum NamedContractError {
	NotFound,
}

impl From<NamedContractError> for Error {
	fn from(e: NamedContractError) -> Error {
		match e {
			NamedContractError::NotFound => Error {
				code: ErrorCode::ServerError(CONTRACT_DOESNT_EXIST),
				message: "The specified contract doesn't exist.".into(),
				data: None,
			},
		}
	}
}


pub type NamedContractResult<AccountId> = Result<NamedContract<AccountId>>;

/// Converts a runtime trap into an RPC error.
pub fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
	Error {
		code: ErrorCode::ServerError(RUNTIME_ERROR),
		message: "Runtime error".into(),
		data: Some(format!("{:?}", err).into()),
	}
}

pub fn decode_hex<H: std::fmt::Debug + Copy, T: TryFrom<H>>(from: H, name: &str) -> Result<T> {
	from.try_into().map_err(|_| Error {
		code: ErrorCode::InvalidParams,
		message: format!("{:?} does not fit into the {} type", from, name),
		data: None,
	})
}

pub fn limit_gas(gas_limit: Weight) -> Result<()> {
	if gas_limit > GAS_LIMIT {
		Err(Error {
			code: ErrorCode::InvalidParams,
			message: format!(
				"Requested gas limit is greater than maximum allowed: {} > {}",
				gas_limit, GAS_LIMIT
			),
			data: None,
		})
	} else {
		Ok(())
	}
}
