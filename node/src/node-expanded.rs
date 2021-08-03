#![feature(prelude_import)]
//! Substrate Node CLI library.
#![warn(missing_docs)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod chain_spec {
    use sp_core::{Pair, Public, sr25519};
    use compose_runtime::{
        AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, SudoConfig,
        SystemConfig, WASM_BINARY, Signature,
    };
    use sp_consensus_aura::sr25519::AuthorityId as AuraId;
    use sp_finality_grandpa::AuthorityId as GrandpaId;
    use sp_runtime::traits::{Verify, IdentifyAccount};
    use sc_service::ChainType;
    use hex_literal::hex;
    /// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
    pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
    /// Helper function to generate a crypto pair from seed
    pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
        TPublic::Pair::from_string(
            &{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["//"],
                    &match (&seed,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            },
            None,
        )
        .expect("static values are valid; qed")
        .public()
    }
    type AccountPublic = <Signature as Verify>::Signer;
    /// Helper function to generate an account ID from seed
    pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
    {
        AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
    }
    /// Helper function to generate an authority key for Aura
    pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
        (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
    }
    pub fn testnet_authorities() -> Vec<(AuraId, GrandpaId)> {
        use sp_core::crypto::UncheckedInto;
        let initial_authorities : Vec < (AccountId , AccountId , AuraId , GrandpaId) > = < [_] > :: into_vec (box [({ # [allow (dead_code)] enum ProcMacroHack { Value = ("\"4ad2d854d1bc5bbfdf93b689cb1aa3986684126c2b16d86924019de6798b0971\"" , 0) . 1 , } [0x4au8 , 0xd2u8 , 0xd8u8 , 0x54u8 , 0xd1u8 , 0xbcu8 , 0x5bu8 , 0xbfu8 , 0xdfu8 , 0x93u8 , 0xb6u8 , 0x89u8 , 0xcbu8 , 0x1au8 , 0xa3u8 , 0x98u8 , 0x66u8 , 0x84u8 , 0x12u8 , 0x6cu8 , 0x2bu8 , 0x16u8 , 0xd8u8 , 0x69u8 , 0x24u8 , 0x01u8 , 0x9du8 , 0xe6u8 , 0x79u8 , 0x8bu8 , 0x09u8 , 0x71u8] } . into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"526d7d3d5357d20e4cd75d1a452bd5b4903caf160f55d6bd19807efbdf165319\"" , 0) . 1 , } [0x52u8 , 0x6du8 , 0x7du8 , 0x3du8 , 0x53u8 , 0x57u8 , 0xd2u8 , 0x0eu8 , 0x4cu8 , 0xd7u8 , 0x5du8 , 0x1au8 , 0x45u8 , 0x2bu8 , 0xd5u8 , 0xb4u8 , 0x90u8 , 0x3cu8 , 0xafu8 , 0x16u8 , 0x0fu8 , 0x55u8 , 0xd6u8 , 0xbdu8 , 0x19u8 , 0x80u8 , 0x7eu8 , 0xfbu8 , 0xdfu8 , 0x16u8 , 0x53u8 , 0x19u8] } . into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"bc09354c12c054c8f6b3da208485eacec4ac648bad348895273b37bab5a0937c\"" , 0) . 1 , } [0xbcu8 , 0x09u8 , 0x35u8 , 0x4cu8 , 0x12u8 , 0xc0u8 , 0x54u8 , 0xc8u8 , 0xf6u8 , 0xb3u8 , 0xdau8 , 0x20u8 , 0x84u8 , 0x85u8 , 0xeau8 , 0xceu8 , 0xc4u8 , 0xacu8 , 0x64u8 , 0x8bu8 , 0xadu8 , 0x34u8 , 0x88u8 , 0x95u8 , 0x27u8 , 0x3bu8 , 0x37u8 , 0xbau8 , 0xb5u8 , 0xa0u8 , 0x93u8 , 0x7cu8] } . unchecked_into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"7bc6fd5dc6e832b294bbf2ae21df67f990a526793a9ded12a5e54e40a5a94d1d\"" , 0) . 1 , } [0x7bu8 , 0xc6u8 , 0xfdu8 , 0x5du8 , 0xc6u8 , 0xe8u8 , 0x32u8 , 0xb2u8 , 0x94u8 , 0xbbu8 , 0xf2u8 , 0xaeu8 , 0x21u8 , 0xdfu8 , 0x67u8 , 0xf9u8 , 0x90u8 , 0xa5u8 , 0x26u8 , 0x79u8 , 0x3au8 , 0x9du8 , 0xedu8 , 0x12u8 , 0xa5u8 , 0xe5u8 , 0x4eu8 , 0x40u8 , 0xa5u8 , 0xa9u8 , 0x4du8 , 0x1du8] } . unchecked_into ()) , ({ # [allow (dead_code)] enum ProcMacroHack { Value = ("\"d0e0ece66fb861b82383e85326a2e179316021105492820ca544ea8743620b59\"" , 0) . 1 , } [0xd0u8 , 0xe0u8 , 0xecu8 , 0xe6u8 , 0x6fu8 , 0xb8u8 , 0x61u8 , 0xb8u8 , 0x23u8 , 0x83u8 , 0xe8u8 , 0x53u8 , 0x26u8 , 0xa2u8 , 0xe1u8 , 0x79u8 , 0x31u8 , 0x60u8 , 0x21u8 , 0x10u8 , 0x54u8 , 0x92u8 , 0x82u8 , 0x0cu8 , 0xa5u8 , 0x44u8 , 0xeau8 , 0x87u8 , 0x43u8 , 0x62u8 , 0x0bu8 , 0x59u8] } . into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"88e95527362f479ebf30502db2f7d88329e034f5d77aed585042c548fa93ae01\"" , 0) . 1 , } [0x88u8 , 0xe9u8 , 0x55u8 , 0x27u8 , 0x36u8 , 0x2fu8 , 0x47u8 , 0x9eu8 , 0xbfu8 , 0x30u8 , 0x50u8 , 0x2du8 , 0xb2u8 , 0xf7u8 , 0xd8u8 , 0x83u8 , 0x29u8 , 0xe0u8 , 0x34u8 , 0xf5u8 , 0xd7u8 , 0x7au8 , 0xedu8 , 0x58u8 , 0x50u8 , 0x42u8 , 0xc5u8 , 0x48u8 , 0xfau8 , 0x93u8 , 0xaeu8 , 0x01u8] } . into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"66be63b7bcbfb91040e5248e2d1ceb822cf219c57848c5924ffa3a1f8e67ba72\"" , 0) . 1 , } [0x66u8 , 0xbeu8 , 0x63u8 , 0xb7u8 , 0xbcu8 , 0xbfu8 , 0xb9u8 , 0x10u8 , 0x40u8 , 0xe5u8 , 0x24u8 , 0x8eu8 , 0x2du8 , 0x1cu8 , 0xebu8 , 0x82u8 , 0x2cu8 , 0xf2u8 , 0x19u8 , 0xc5u8 , 0x78u8 , 0x48u8 , 0xc5u8 , 0x92u8 , 0x4fu8 , 0xfau8 , 0x3au8 , 0x1fu8 , 0x8eu8 , 0x67u8 , 0xbau8 , 0x72u8] } . unchecked_into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"eb17972691ec3a7d09a316baddc8838362ade2c12a21a506d697903e16577bfd\"" , 0) . 1 , } [0xebu8 , 0x17u8 , 0x97u8 , 0x26u8 , 0x91u8 , 0xecu8 , 0x3au8 , 0x7du8 , 0x09u8 , 0xa3u8 , 0x16u8 , 0xbau8 , 0xddu8 , 0xc8u8 , 0x83u8 , 0x83u8 , 0x62u8 , 0xadu8 , 0xe2u8 , 0xc1u8 , 0x2au8 , 0x21u8 , 0xa5u8 , 0x06u8 , 0xd6u8 , 0x97u8 , 0x90u8 , 0x3eu8 , 0x16u8 , 0x57u8 , 0x7bu8 , 0xfdu8] } . unchecked_into ())]) ;
        initial_authorities
            .into_iter()
            .map(|(_, _, aura_id, grandpa_id)| (aura_id, grandpa_id))
            .collect()
    }
    pub fn testnet_root() -> AccountId {
        {
            #[allow(dead_code)]
            enum ProcMacroHack {
                Value = (
                    "\"baa78c7154c7f82d6d377177e20bcab65d327eca0086513f9964f5a0f6bdad56\"",
                    0,
                )
                    .1,
            }
            [
                0xbau8, 0xa7u8, 0x8cu8, 0x71u8, 0x54u8, 0xc7u8, 0xf8u8, 0x2du8, 0x6du8, 0x37u8,
                0x71u8, 0x77u8, 0xe2u8, 0x0bu8, 0xcau8, 0xb6u8, 0x5du8, 0x32u8, 0x7eu8, 0xcau8,
                0x00u8, 0x86u8, 0x51u8, 0x3fu8, 0x99u8, 0x64u8, 0xf5u8, 0xa0u8, 0xf6u8, 0xbdu8,
                0xadu8, 0x56u8,
            ]
        }
        .into()
    }
    pub fn development_config() -> Result<ChainSpec, String> {
        let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;
        Ok(ChainSpec::from_genesis(
            "Development",
            "dev",
            ChainType::Development,
            move || {
                testnet_genesis(
                    wasm_binary,
                    <[_]>::into_vec(box [authority_keys_from_seed("Alice")]),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    <[_]>::into_vec(box [
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                    ]),
                )
            },
            ::alloc::vec::Vec::new(),
            None,
            None,
            None,
            None,
        ))
    }
    pub fn testnet_config() -> Result<ChainSpec, String> {
        let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".to_string(), "CAN".into());
        properties.insert("tokenDecimals".to_string(), 12.into());
        properties.insert("ss58Format".to_string(), 42.into());
        Ok (ChainSpec :: from_genesis ("Canvas Testnet" , "canvas_testnet2" , ChainType :: Live , move | | testnet_genesis (wasm_binary , testnet_authorities () , testnet_root () , < [_] > :: into_vec (box [testnet_root () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"18c64aa111a8a0e6e4eed41d6d906c7614d745e48be3cfc13b6128e1d51f4405\"" , 0) . 1 , } [0x18u8 , 0xc6u8 , 0x4au8 , 0xa1u8 , 0x11u8 , 0xa8u8 , 0xa0u8 , 0xe6u8 , 0xe4u8 , 0xeeu8 , 0xd4u8 , 0x1du8 , 0x6du8 , 0x90u8 , 0x6cu8 , 0x76u8 , 0x14u8 , 0xd7u8 , 0x45u8 , 0xe4u8 , 0x8bu8 , 0xe3u8 , 0xcfu8 , 0xc1u8 , 0x3bu8 , 0x61u8 , 0x28u8 , 0xe1u8 , 0xd5u8 , 0x1fu8 , 0x44u8 , 0x05u8] } . into () , { # [allow (dead_code)] enum ProcMacroHack { Value = ("\"0e47e2344d523c3cc5c34394b0d58b9a4200e813a038e6c5a6163cc07d70b069\"" , 0) . 1 , } [0x0eu8 , 0x47u8 , 0xe2u8 , 0x34u8 , 0x4du8 , 0x52u8 , 0x3cu8 , 0x3cu8 , 0xc5u8 , 0xc3u8 , 0x43u8 , 0x94u8 , 0xb0u8 , 0xd5u8 , 0x8bu8 , 0x9au8 , 0x42u8 , 0x00u8 , 0xe8u8 , 0x13u8 , 0xa0u8 , 0x38u8 , 0xe6u8 , 0xc5u8 , 0xa6u8 , 0x16u8 , 0x3cu8 , 0xc0u8 , 0x7du8 , 0x70u8 , 0xb0u8 , 0x69u8] } . into ()])) , < [_] > :: into_vec (box ["/ip4/34.90.191.237/tcp/30333/p2p/12D3KooWKg3Rpxcr9oJ8n6khoxpGKWztCZydtUZk2cojHqnfLrpj" . parse () . expect ("MultiaddrWithPeerId") , "/ip4/35.204.68.28/tcp/30333/p2p/12D3KooWPEXYrz8tHU3nDtPoPw4V7ou5dzMEWSTuUj7vaWiYVAVh" . parse () . expect ("MultiaddrWithPeerId") , "/ip4/34.90.139.15/tcp/30333/p2p/12D3KooWEVU8AFNary4nP4qEnEcwJaRuy59Wefekzdu9pKbnVEhk" . parse () . expect ("MultiaddrWithPeerId") , "/ip4/35.204.99.97/tcp/30333/p2p/12D3KooWP6pV3ZmcXzGDjv8ZMgA6nZxfAKDxSz4VNiLx6vVCQgJX" . parse () . expect ("MultiaddrWithPeerId")]) , None , Some ("prc") , Some (properties) , None))
    }
    fn testnet_genesis(
        wasm_binary: &[u8],
        initial_authorities: Vec<(AuraId, GrandpaId)>,
        root_key: AccountId,
        endowed_accounts: Vec<AccountId>,
    ) -> GenesisConfig {
        GenesisConfig {
            system: SystemConfig {
                code: wasm_binary.to_vec(),
                changes_trie_config: Default::default(),
            },
            balances: BalancesConfig {
                balances: endowed_accounts
                    .iter()
                    .cloned()
                    .map(|k| (k, 1 << 60))
                    .collect(),
            },
            aura: AuraConfig {
                authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
            },
            grandpa: GrandpaConfig {
                authorities: initial_authorities
                    .iter()
                    .map(|x| (x.1.clone(), 1))
                    .collect(),
            },
            sudo: SudoConfig { key: root_key },
        }
    }
}
#[macro_use]
mod service {
    //! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
    use std::{sync::Arc, time::Duration};
    use sc_client_api::{ExecutorProvider, RemoteBackend};
    use compose_runtime::{self, opaque::Block, RuntimeApi};
    use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
    use sc_executor::native_executor_instance;
    pub use sc_executor::NativeExecutor;
    use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
    use sc_consensus_aura::{ImportQueueParams, StartAuraParams, SlotProportion};
    use sc_finality_grandpa::SharedVoterState;
    use sc_keystore::LocalKeystore;
    use sc_telemetry::{Telemetry, TelemetryWorker};
    use sp_consensus::SlotData;
    /// A unit struct which implements `NativeExecutionDispatch` feeding in the
    /// hard-coded runtime.
    pub struct Executor;
    impl ::sc_executor::NativeExecutionDispatch for Executor {
        type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
        fn dispatch(
            ext: &mut dyn ::sc_executor::Externalities,
            method: &str,
            data: &[u8],
        ) -> ::sc_executor::error::Result<Vec<u8>> {
            ::sc_executor::with_externalities_safe(ext, move || {
                compose_runtime::api::dispatch(method, data)
            })?
            .ok_or_else(|| ::sc_executor::error::Error::MethodNotFound(method.to_owned()))
        }
        fn native_version() -> ::sc_executor::NativeVersion {
            compose_runtime::native_version()
        }
    }
    type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
    type FullBackend = sc_service::TFullBackend<Block>;
    type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
    pub fn new_partial(
        config: &Configuration,
    ) -> Result<
        sc_service::PartialComponents<
            FullClient,
            FullBackend,
            FullSelectChain,
            sp_consensus::DefaultImportQueue<Block, FullClient>,
            sc_transaction_pool::FullPool<Block, FullClient>,
            (
                sc_finality_grandpa::GrandpaBlockImport<
                    FullBackend,
                    Block,
                    FullClient,
                    FullSelectChain,
                >,
                sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
                Option<Telemetry>,
            ),
        >,
        ServiceError,
    > {
        if config.keystore_remote.is_some() {
            return Err(ServiceError::Other({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Remote Keystores are not supported."],
                    &match () {
                        () => [],
                    },
                ));
                res
            }));
        }
        let telemetry = config
            .telemetry_endpoints
            .clone()
            .filter(|x| !x.is_empty())
            .map(|endpoints| -> Result<_, sc_telemetry::Error> {
                let worker = TelemetryWorker::new(16)?;
                let telemetry = worker.handle().new_telemetry(endpoints);
                Ok((worker, telemetry))
            })
            .transpose()?;
        let (client, backend, keystore_container, task_manager) =
            sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
                &config,
                telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            )?;
        let client = Arc::new(client);
        let telemetry = telemetry.map(|(worker, telemetry)| {
            task_manager.spawn_handle().spawn("telemetry", worker.run());
            telemetry
        });
        let select_chain = sc_consensus::LongestChain::new(backend.clone());
        let transaction_pool = sc_transaction_pool::BasicPool::new_full(
            config.transaction_pool.clone(),
            config.role.is_authority().into(),
            config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
        );
        let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
            client.clone(),
            &(client.clone() as Arc<_>),
            select_chain.clone(),
            telemetry.as_ref().map(|x| x.handle()),
        )?;
        let slot_duration = sc_consensus_aura::slot_duration(&*client)?.slot_duration();
        let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _, _>(
            ImportQueueParams {
                block_import: grandpa_block_import.clone(),
                justification_import: Some(Box::new(grandpa_block_import.clone())),
                client: client.clone(),
                create_inherent_data_providers: move |_, ()| async move {
                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
                    let slot = sp_consensus_aura :: inherents :: InherentDataProvider :: from_timestamp_and_duration (* timestamp , slot_duration) ;
                    Ok((timestamp, slot))
                },
                spawner: &task_manager.spawn_essential_handle(),
                can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(
                    client.executor().clone(),
                ),
                registry: config.prometheus_registry(),
                check_for_equivocation: Default::default(),
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            },
        )?;
        Ok(sc_service::PartialComponents {
            client,
            backend,
            task_manager,
            import_queue,
            keystore_container,
            select_chain,
            transaction_pool,
            other: (grandpa_block_import, grandpa_link, telemetry),
        })
    }
    fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
        Err("Remote Keystore not supported.")
    }
    /// Builds a new service for a full client.
    pub fn new_full(mut config: Configuration) -> Result<TaskManager, ServiceError> {
        let sc_service::PartialComponents {
            client,
            backend,
            mut task_manager,
            import_queue,
            mut keystore_container,
            select_chain,
            transaction_pool,
            other: (block_import, grandpa_link, mut telemetry),
        } = new_partial(&config)?;
        if let Some(url) = &config.keystore_remote {
            match remote_keystore(url) {
                Ok(k) => keystore_container.set_remote_keystore(k),
                Err(e) => {
                    return Err(ServiceError::Other({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error hooking up remote keystore for ", ": "],
                            &match (&url, &e) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                }
            };
        }
        config
            .network
            .extra_sets
            .push(sc_finality_grandpa::grandpa_peers_set_config());
        let (network, system_rpc_tx, network_starter) =
            sc_service::build_network(sc_service::BuildNetworkParams {
                config: &config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_queue,
                on_demand: None,
                block_announce_validator_builder: None,
            })?;
        if config.offchain_worker.enabled {
            sc_service::build_offchain_workers(
                &config,
                task_manager.spawn_handle(),
                client.clone(),
                network.clone(),
            );
        }
        let role = config.role.clone();
        let force_authoring = config.force_authoring;
        let backoff_authoring_blocks: Option<()> = None;
        let name = config.network.node_name.clone();
        let enable_grandpa = !config.disable_grandpa;
        let prometheus_registry = config.prometheus_registry().cloned();
        let rpc_extensions_builder = {
            let client = client.clone();
            let pool = transaction_pool.clone();
            Box::new(move |deny_unsafe, _| {
                let deps = crate::rpc::FullDeps {
                    client: client.clone(),
                    pool: pool.clone(),
                    deny_unsafe,
                };
                crate::rpc::create_full(deps)
            })
        };
        let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
            network: network.clone(),
            client: client.clone(),
            keystore: keystore_container.sync_keystore(),
            task_manager: &mut task_manager,
            transaction_pool: transaction_pool.clone(),
            rpc_extensions_builder,
            on_demand: None,
            remote_blockchain: None,
            backend,
            system_rpc_tx,
            config,
            telemetry: telemetry.as_mut(),
        })?;
        if role.is_authority() {
            let proposer_factory = sc_basic_authorship::ProposerFactory::new(
                task_manager.spawn_handle(),
                client.clone(),
                transaction_pool,
                prometheus_registry.as_ref(),
                telemetry.as_ref().map(|x| x.handle()),
            );
            let can_author_with =
                sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());
            let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
            let raw_slot_duration = slot_duration.slot_duration();
            let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _, _>(
                StartAuraParams {
                    slot_duration,
                    client: client.clone(),
                    select_chain,
                    block_import,
                    proposer_factory,
                    create_inherent_data_providers: move |_, ()| async move {
                        let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
                        let slot = sp_consensus_aura :: inherents :: InherentDataProvider :: from_timestamp_and_duration (* timestamp , raw_slot_duration) ;
                        Ok((timestamp, slot))
                    },
                    force_authoring,
                    backoff_authoring_blocks,
                    keystore: keystore_container.sync_keystore(),
                    can_author_with,
                    sync_oracle: network.clone(),
                    justification_sync_link: network.clone(),
                    block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
                    max_block_proposal_slot_portion: None,
                    telemetry: telemetry.as_ref().map(|x| x.handle()),
                },
            )?;
            task_manager
                .spawn_essential_handle()
                .spawn_blocking("aura", aura);
        }
        let keystore = if role.is_authority() {
            Some(keystore_container.sync_keystore())
        } else {
            None
        };
        let grandpa_config = sc_finality_grandpa::Config {
            gossip_duration: Duration::from_millis(333),
            justification_period: 512,
            name: Some(name),
            observer_enabled: false,
            keystore,
            local_role: role,
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        };
        if enable_grandpa {
            let grandpa_config = sc_finality_grandpa::GrandpaParams {
                config: grandpa_config,
                link: grandpa_link,
                network,
                voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
                prometheus_registry,
                shared_voter_state: SharedVoterState::empty(),
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            };
            task_manager.spawn_essential_handle().spawn_blocking(
                "grandpa-voter",
                sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
            );
        }
        network_starter.start_network();
        Ok(task_manager)
    }
    /// Builds a new service for a light client.
    pub fn new_light(mut config: Configuration) -> Result<TaskManager, ServiceError> {
        let telemetry = config
            .telemetry_endpoints
            .clone()
            .filter(|x| !x.is_empty())
            .map(|endpoints| -> Result<_, sc_telemetry::Error> {
                let worker = TelemetryWorker::new(16)?;
                let telemetry = worker.handle().new_telemetry(endpoints);
                Ok((worker, telemetry))
            })
            .transpose()?;
        let (client, backend, keystore_container, mut task_manager, on_demand) =
            sc_service::new_light_parts::<Block, RuntimeApi, Executor>(
                &config,
                telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            )?;
        let mut telemetry = telemetry.map(|(worker, telemetry)| {
            task_manager.spawn_handle().spawn("telemetry", worker.run());
            telemetry
        });
        config
            .network
            .extra_sets
            .push(sc_finality_grandpa::grandpa_peers_set_config());
        let select_chain = sc_consensus::LongestChain::new(backend.clone());
        let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
            config.transaction_pool.clone(),
            config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
            on_demand.clone(),
        ));
        let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
            client.clone(),
            &(client.clone() as Arc<_>),
            select_chain.clone(),
            telemetry.as_ref().map(|x| x.handle()),
        )?;
        let slot_duration = sc_consensus_aura::slot_duration(&*client)?.slot_duration();
        let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _, _>(
            ImportQueueParams {
                block_import: grandpa_block_import.clone(),
                justification_import: Some(Box::new(grandpa_block_import.clone())),
                client: client.clone(),
                create_inherent_data_providers: move |_, ()| async move {
                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
                    let slot = sp_consensus_aura :: inherents :: InherentDataProvider :: from_timestamp_and_duration (* timestamp , slot_duration) ;
                    Ok((timestamp, slot))
                },
                spawner: &task_manager.spawn_essential_handle(),
                can_author_with: sp_consensus::NeverCanAuthor,
                registry: config.prometheus_registry(),
                check_for_equivocation: Default::default(),
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            },
        )?;
        let (network, system_rpc_tx, network_starter) =
            sc_service::build_network(sc_service::BuildNetworkParams {
                config: &config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_queue,
                on_demand: Some(on_demand.clone()),
                block_announce_validator_builder: None,
            })?;
        if config.offchain_worker.enabled {
            sc_service::build_offchain_workers(
                &config,
                task_manager.spawn_handle(),
                client.clone(),
                network.clone(),
            );
        }
        let enable_grandpa = !config.disable_grandpa;
        if enable_grandpa {
            let name = config.network.node_name.clone();
            let config = sc_finality_grandpa::Config {
                gossip_duration: std::time::Duration::from_millis(333),
                justification_period: 512,
                name: Some(name),
                observer_enabled: false,
                keystore: None,
                local_role: config.role.clone(),
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            };
            task_manager.spawn_handle().spawn_blocking(
                "grandpa-observer",
                sc_finality_grandpa::run_grandpa_observer(config, grandpa_link, network.clone())?,
            );
        }
        sc_service::spawn_tasks(sc_service::SpawnTasksParams {
            remote_blockchain: Some(backend.remote_blockchain()),
            transaction_pool,
            task_manager: &mut task_manager,
            on_demand: Some(on_demand),
            rpc_extensions_builder: Box::new(|_, _| ()),
            config,
            client,
            keystore: keystore_container.sync_keystore(),
            backend,
            network,
            system_rpc_tx,
            telemetry: telemetry.as_mut(),
        })?;
        network_starter.start_network();
        Ok(task_manager)
    }
}
mod cli {
    use structopt::StructOpt;
    use sc_cli::RunCmd;
    pub struct Cli {
        #[structopt(subcommand)]
        pub subcommand: Option<Subcommand>,
        #[structopt(flatten)]
        pub run: RunCmd,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Cli {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Cli {
                    subcommand: ref __self_0_0,
                    run: ref __self_0_1,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Cli");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "subcommand",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "run",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[allow(unused_variables)]
    #[allow(unknown_lints)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo
    )]
    #[deny(clippy::correctness)]
    #[allow(dead_code, unreachable_code)]
    impl ::structopt::StructOpt for Cli {
        fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
            let app = ::structopt::clap::App::new("compose-node");
            <Self as ::structopt::StructOptInternal>::augment_clap(app)
        }
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
            Cli {
                subcommand: <Subcommand as ::structopt::StructOptInternal>::from_subcommand(
                    matches.subcommand(),
                ),
                run: ::structopt::StructOpt::from_clap(matches),
            }
        }
    }
    #[allow(unused_variables)]
    #[allow(unknown_lints)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo
    )]
    #[deny(clippy::correctness)]
    #[allow(dead_code, unreachable_code)]
    impl ::structopt::StructOptInternal for Cli {
        fn augment_clap<'a, 'b>(
            app: ::structopt::clap::App<'a, 'b>,
        ) -> ::structopt::clap::App<'a, 'b> {
            {
                let app = app;
                let app = <RunCmd as ::structopt::StructOptInternal>::augment_clap(app);
                let app = if <RunCmd as ::structopt::StructOptInternal>::is_subcommand() {
                    app.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                } else {
                    app
                };
                let app = <Subcommand as ::structopt::StructOptInternal>::augment_clap(app);
                app.version("0.1.0")
            }
        }
        fn is_subcommand() -> bool {
            false
        }
    }
    pub enum Subcommand {
        /// Key management cli utilities
        Key(sc_cli::KeySubcommand),
        /// Build a chain specification.
        BuildSpec(sc_cli::BuildSpecCmd),
        /// Validate blocks.
        CheckBlock(sc_cli::CheckBlockCmd),
        /// Export blocks.
        ExportBlocks(sc_cli::ExportBlocksCmd),
        /// Export the state of a given block into a chain spec.
        ExportState(sc_cli::ExportStateCmd),
        /// Import blocks.
        ImportBlocks(sc_cli::ImportBlocksCmd),
        /// Remove the whole chain.
        PurgeChain(sc_cli::PurgeChainCmd),
        /// Revert the chain to a previous state.
        Revert(sc_cli::RevertCmd),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Subcommand {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Subcommand::Key(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Key");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::BuildSpec(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "BuildSpec");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::CheckBlock(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "CheckBlock");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::ExportBlocks(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ExportBlocks");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::ExportState(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ExportState");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::ImportBlocks(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ImportBlocks");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::PurgeChain(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "PurgeChain");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Subcommand::Revert(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Revert");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[allow(unknown_lints)]
    #[allow(unused_variables, dead_code, unreachable_code)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo
    )]
    #[deny(clippy::correctness)]
    impl ::structopt::StructOpt for Subcommand {
        fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
            let app = ::structopt::clap::App::new("compose-node")
                .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
            <Self as ::structopt::StructOptInternal>::augment_clap(app)
        }
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
            <Self as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand()).expect(
                "structopt misuse: You likely tried to #[flatten] a struct \
                         that contains #[subcommand]. This is forbidden.",
            )
        }
    }
    #[allow(unused_variables)]
    #[allow(unknown_lints)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo
    )]
    #[deny(clippy::correctness)]
    #[allow(dead_code, unreachable_code)]
    impl ::structopt::StructOptInternal for Subcommand {
        fn augment_clap<'a, 'b>(
            app: ::structopt::clap::App<'a, 'b>,
        ) -> ::structopt::clap::App<'a, 'b> {
            let app = app;
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("key");
                let subcommand = {
                    let subcommand =
                        <sc_cli::KeySubcommand as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::KeySubcommand as ::structopt::StructOptInternal>::is_subcommand() {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand
                    .about("Key management cli utilities")
                    .version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("build-spec");
                let subcommand = {
                    let subcommand =
                        <sc_cli::BuildSpecCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::BuildSpecCmd as ::structopt::StructOptInternal>::is_subcommand() {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand
                    .about("Build a chain specification")
                    .version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("check-block");
                let subcommand = {
                    let subcommand =
                        <sc_cli::CheckBlockCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::CheckBlockCmd as ::structopt::StructOptInternal>::is_subcommand() {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand.about("Validate blocks").version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("export-blocks");
                let subcommand = {
                    let subcommand =
                        <sc_cli::ExportBlocksCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::ExportBlocksCmd as ::structopt::StructOptInternal>::is_subcommand()
                    {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand.about("Export blocks").version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("export-state");
                let subcommand = {
                    let subcommand =
                        <sc_cli::ExportStateCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::ExportStateCmd as ::structopt::StructOptInternal>::is_subcommand() {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand
                    .about("Export the state of a given block into a chain spec")
                    .version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("import-blocks");
                let subcommand = {
                    let subcommand =
                        <sc_cli::ImportBlocksCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::ImportBlocksCmd as ::structopt::StructOptInternal>::is_subcommand()
                    {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand.about("Import blocks").version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("purge-chain");
                let subcommand = {
                    let subcommand =
                        <sc_cli::PurgeChainCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::PurgeChainCmd as ::structopt::StructOptInternal>::is_subcommand() {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand.about("Remove the whole chain").version("0.1.0")
            });
            let app = app.subcommand({
                let subcommand = ::structopt::clap::SubCommand::with_name("revert");
                let subcommand = {
                    let subcommand =
                        <sc_cli::RevertCmd as ::structopt::StructOptInternal>::augment_clap(
                            subcommand,
                        );
                    if <sc_cli::RevertCmd as ::structopt::StructOptInternal>::is_subcommand() {
                        subcommand
                            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand
                    .about("Revert the chain to a previous state")
                    .version("0.1.0")
            });
            app.version("0.1.0")
        }
        fn from_subcommand<'a, 'b>(
            sub: (&'b str, Option<&'b ::structopt::clap::ArgMatches<'a>>),
        ) -> Option<Self> {
            match sub {
                ("key", Some(matches)) => Some(Subcommand::Key(
                    <sc_cli::KeySubcommand as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("build-spec", Some(matches)) => Some(Subcommand::BuildSpec(
                    <sc_cli::BuildSpecCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("check-block", Some(matches)) => Some(Subcommand::CheckBlock(
                    <sc_cli::CheckBlockCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("export-blocks", Some(matches)) => Some(Subcommand::ExportBlocks(
                    <sc_cli::ExportBlocksCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("export-state", Some(matches)) => Some(Subcommand::ExportState(
                    <sc_cli::ExportStateCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("import-blocks", Some(matches)) => Some(Subcommand::ImportBlocks(
                    <sc_cli::ImportBlocksCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("purge-chain", Some(matches)) => Some(Subcommand::PurgeChain(
                    <sc_cli::PurgeChainCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                ("revert", Some(matches)) => Some(Subcommand::Revert(
                    <sc_cli::RevertCmd as ::structopt::StructOpt>::from_clap(matches),
                )),
                other => {
                    None
                }
            }
        }
        fn is_subcommand() -> bool {
            true
        }
    }
}
mod command {
    use crate::{chain_spec, service};
    use crate::cli::{Cli, Subcommand};
    use sc_cli::{SubstrateCli, RuntimeVersion, Role, ChainSpec};
    use sc_service::PartialComponents;
    impl SubstrateCli for Cli {
        fn impl_name() -> String {
            "Canvas Node".into()
        }
        fn impl_version() -> String {
            "0.1.0-9d6bbc1-x86_64-linux-gnu".into()
        }
        fn description() -> String {
            "".into()
        }
        fn author() -> String {
            "Compose".into()
        }
        fn support_url() -> String {
            "https://github.com/paritytech/canvas-node/issues/new".into()
        }
        fn copyright_start_year() -> i32 {
            2020
        }
        fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
            Ok(match id {
                "dev" => Box::new(chain_spec::development_config()?),
                "" => Box::new(chain_spec::testnet_config()?),
                path => Box::new(chain_spec::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?),
            })
        }
        fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
            &compose_runtime::VERSION
        }
    }
    /// Parse and run command line arguments
    pub fn run() -> sc_cli::Result<()> {
        let cli = Cli::from_args();
        match &cli.subcommand {
            Some(Subcommand::Key(cmd)) => cmd.run(&cli),
            Some(Subcommand::BuildSpec(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
            }
            Some(Subcommand::CheckBlock(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.async_run(|config| {
                    let PartialComponents {
                        client,
                        task_manager,
                        import_queue,
                        ..
                    } = service::new_partial(&config)?;
                    Ok((cmd.run(client, import_queue), task_manager))
                })
            }
            Some(Subcommand::ExportBlocks(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.async_run(|config| {
                    let PartialComponents {
                        client,
                        task_manager,
                        ..
                    } = service::new_partial(&config)?;
                    Ok((cmd.run(client, config.database), task_manager))
                })
            }
            Some(Subcommand::ExportState(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.async_run(|config| {
                    let PartialComponents {
                        client,
                        task_manager,
                        ..
                    } = service::new_partial(&config)?;
                    Ok((cmd.run(client, config.chain_spec), task_manager))
                })
            }
            Some(Subcommand::ImportBlocks(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.async_run(|config| {
                    let PartialComponents {
                        client,
                        task_manager,
                        import_queue,
                        ..
                    } = service::new_partial(&config)?;
                    Ok((cmd.run(client, import_queue), task_manager))
                })
            }
            Some(Subcommand::PurgeChain(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.sync_run(|config| cmd.run(config.database))
            }
            Some(Subcommand::Revert(cmd)) => {
                let runner = cli.create_runner(cmd)?;
                runner.async_run(|config| {
                    let PartialComponents {
                        client,
                        task_manager,
                        backend,
                        ..
                    } = service::new_partial(&config)?;
                    Ok((cmd.run(client, backend), task_manager))
                })
            }
            None => {
                let runner = cli.create_runner(&cli.run)?;
                runner.run_node_until_exit(|config| async move {
                    match config.role {
                        Role::Light => service::new_light(config),
                        _ => service::new_full(config),
                    }
                    .map_err(sc_cli::Error::Service)
                })
            }
        }
    }
}
mod rpc {
    //! A collection of node-specific RPC methods.
    //! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
    //! used by Substrate nodes. This file extends those RPC definitions with
    //! capabilities that are specific to this project's runtime configuration.
    #![warn(missing_docs)]
    use std::sync::Arc;
    use compose_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Index};
    use pallet_contracts_rpc::{Contracts, ContractsApi};
    use sp_api::ProvideRuntimeApi;
    use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
    use sp_block_builder::BlockBuilder;
    pub use sc_rpc_api::DenyUnsafe;
    use sc_transaction_pool_api::TransactionPool;
    /// Full client dependencies.
    pub struct FullDeps<C, P> {
        /// The client instance to use.
        pub client: Arc<C>,
        /// Transaction pool instance.
        pub pool: Arc<P>,
        /// Whether to deny unsafe calls
        pub deny_unsafe: DenyUnsafe,
    }
    /// Instantiate all full RPC extensions.
    pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
    where
        C: ProvideRuntimeApi<Block>,
        C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
        C: Send + Sync + 'static,
        C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
        C::Api:
            pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
        C::Api: compose_rpc::ComposeRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
        C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
        C::Api: BlockBuilder<Block>,
        P: TransactionPool + 'static,
    {
        use substrate_frame_rpc_system::{FullSystem, SystemApi};
        use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
        let mut io = jsonrpc_core::IoHandler::default();
        let FullDeps {
            client,
            pool,
            deny_unsafe,
        } = deps;
        io.extend_with(SystemApi::to_delegate(FullSystem::new(
            client.clone(),
            pool,
            deny_unsafe,
        )));
        io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
            client.clone(),
        )));
        io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));
        io.extend_with(compose_rpc::ComposeRuntimeApi::to_delegate(Compose::new(client.clone())));
        io
    }
}
#[allow(dead_code)]
fn main() -> sc_cli::Result<()> {
    command::run()
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
