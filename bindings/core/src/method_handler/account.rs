// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use std::str::FromStr;

use iota_sdk::{
    client::api::{
        input_selection::Burn, PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData,
        SignedTransactionDataDto,
    },
    types::block::{
        output::{dto::OutputDto, Output, Rent},
        Error,
    },
    wallet::{
        account::{
            types::{AccountBalanceDto, TransactionDto},
            Account, CreateAliasParams, MintTokenTransactionDto, OutputDataDto, OutputParams, TransactionOptions,
        },
        MintNativeTokenParams, MintNftParams,
    },
};
use primitive_types::U256;

use crate::{method::AccountMethod, Response, Result};

pub(crate) async fn call_account_method_internal(account: &Account, method: AccountMethod) -> Result<Response> {
    let response = match method {
        AccountMethod::Burn { burn, options } => {
            let transaction = account
                .burn(
                    Burn::try_from(&burn)?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::ConsolidateOutputs {
            force,
            output_consolidation_threshold,
        } => {
            let transaction = account
                .consolidate_outputs(force, output_consolidation_threshold)
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::CreateAliasOutput { params, options } => {
            let params = params
                .map(|options| CreateAliasParams::try_from(&options))
                .transpose()?;

            let transaction = account
                .create_alias_output(
                    params,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::GenerateAddresses { amount, options } => {
            let address = account.generate_addresses(amount, options).await?;
            Response::GeneratedAddress(address)
        }
        AccountMethod::GetOutputsWithAdditionalUnlockConditions { outputs_to_claim } => {
            let output_ids = account
                .get_unlockable_outputs_with_additional_unlock_conditions(outputs_to_claim)
                .await?;
            Response::OutputIds(output_ids)
        }
        AccountMethod::GetOutput { output_id } => {
            let output_data = account.get_output(&output_id).await;
            Response::OutputData(output_data.as_ref().map(OutputDataDto::from).map(Box::new))
        }
        AccountMethod::GetFoundryOutput { token_id } => {
            let output = account.get_foundry_output(token_id).await?;
            Response::Output(OutputDto::from(&output))
        }
        AccountMethod::GetTransaction { transaction_id } => {
            let transaction = account.get_transaction(&transaction_id).await;
            Response::Transaction(transaction.as_ref().map(TransactionDto::from).map(Box::new))
        }
        AccountMethod::GetIncomingTransaction { transaction_id } => {
            let transaction = account.get_incoming_transaction(&transaction_id).await;

            transaction.map_or_else(
                || Response::Transaction(None),
                |transaction| Response::Transaction(Some(Box::new(TransactionDto::from(&transaction)))),
            )
        }
        AccountMethod::Addresses => {
            let addresses = account.addresses().await?;
            Response::Addresses(addresses)
        }
        AccountMethod::AddressesWithUnspentOutputs => {
            let addresses = account.addresses_with_unspent_outputs().await?;
            Response::AddressesWithUnspentOutputs(addresses)
        }
        AccountMethod::Outputs { filter_options } => {
            let outputs = account.outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        AccountMethod::UnspentOutputs { filter_options } => {
            let outputs = account.unspent_outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        AccountMethod::IncomingTransactions => {
            let transactions = account.incoming_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::Transactions => {
            let transactions = account.transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::PendingTransactions => {
            let transactions = account.pending_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::DecreaseNativeTokenSupply {
            token_id,
            melt_amount,
            options,
        } => {
            let transaction = account
                .decrease_native_token_supply(
                    token_id,
                    U256::try_from(&melt_amount).map_err(|_| Error::InvalidField("melt_amount"))?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::IncreaseNativeTokenSupply {
            token_id,
            mint_amount,
            options,
        } => {
            let transaction = account
                .increase_native_token_supply(
                    token_id,
                    U256::try_from(&mint_amount).map_err(|_| Error::InvalidField("mint_amount"))?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::MintTokenTransaction(MintTokenTransactionDto::from(&transaction))
        }
        AccountMethod::MintNativeToken { params, options } => {
            let transaction = account
                .mint_native_token(
                    MintNativeTokenParams::try_from(&params)?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::MintTokenTransaction(MintTokenTransactionDto::from(&transaction))
        }
        AccountMethod::MinimumRequiredStorageDeposit { output } => {
            let output = Output::try_from_dto(&output, account.client().get_token_supply().await?)?;
            let rent_structure = account.client().get_rent_structure().await?;

            let minimum_storage_deposit = output.rent_cost(&rent_structure);

            Response::MinimumRequiredStorageDeposit(minimum_storage_deposit.to_string())
        }
        AccountMethod::MintNfts { params, options } => {
            let transaction = account
                .mint_nfts(
                    params
                        .iter()
                        .map(MintNftParams::try_from)
                        .collect::<iota_sdk::wallet::Result<Vec<MintNftParams>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::GetBalance => Response::Balance(AccountBalanceDto::from(&account.balance().await?)),
        AccountMethod::PrepareOutput {
            params: options,
            transaction_options,
        } => {
            let output = account
                .prepare_output(
                    OutputParams::try_from(&options)?,
                    transaction_options
                        .as_ref()
                        .map(TransactionOptions::try_from_dto)
                        .transpose()?,
                )
                .await?;
            Response::Output(OutputDto::from(&output))
        }
        AccountMethod::PrepareSendAmount { params, options } => {
            let data = account
                .prepare_send_amount(
                    params,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareTransaction { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let data = account
                .prepare_transaction(
                    outputs
                        .iter()
                        .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                        .collect::<Result<Vec<Output>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::RetryTransactionUntilIncluded {
            transaction_id,
            interval,
            max_attempts,
        } => {
            let block_id = account
                .retry_transaction_until_included(&transaction_id, interval, max_attempts)
                .await?;
            Response::BlockId(block_id)
        }
        AccountMethod::Sync { options } => Response::Balance(AccountBalanceDto::from(&account.sync(options).await?)),
        AccountMethod::SendAmount { params, options } => {
            let transaction = account
                .send_amount(
                    params,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SendNativeTokens { params, options } => {
            let transaction = account
                .send_native_tokens(
                    params.clone(),
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SendNft { params, options } => {
            let transaction = account
                .send_nft(
                    params.clone(),
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SetAlias { alias } => {
            account.set_alias(&alias).await?;
            Response::Ok
        }
        AccountMethod::SetDefaultSyncOptions { options } => {
            account.set_default_sync_options(options).await?;
            Response::Ok
        }
        AccountMethod::SendOutputs { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let transaction = account
                .send(
                    outputs
                        .iter()
                        .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                        .collect::<iota_sdk::wallet::Result<Vec<Output>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SignTransactionEssence {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = account
                .sign_transaction_essence(&PreparedTransactionData::try_from_dto(
                    &prepared_transaction_data,
                    &account.client().get_protocol_parameters().await?,
                )?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        AccountMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto(
                &signed_transaction_data,
                &account.client().get_protocol_parameters().await?,
            )?;
            let transaction = account.submit_and_store_transaction(signed_transaction_data).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = account.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::Vote { event_id, answers } => {
            let transaction = account.vote(event_id, answers).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::StopParticipating { event_id } => {
            let transaction = account.stop_participating(event_id).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetVotingPower => {
            let voting_power = account.get_voting_power().await?;
            Response::VotingPower(voting_power.to_string())
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationOverview { event_ids } => {
            let overview = account.get_participation_overview(event_ids).await?;
            Response::AccountParticipationOverview(overview)
        }
        #[cfg(feature = "participation")]
        AccountMethod::IncreaseVotingPower { amount } => {
            let transaction = account
                .increase_voting_power(
                    u64::from_str(&amount).map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::DecreaseVotingPower { amount } => {
            let transaction = account
                .decrease_voting_power(
                    u64::from_str(&amount).map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::RegisterParticipationEvents { options } => {
            let events = account.register_participation_events(&options).await?;
            Response::ParticipationEvents(events)
        }
        #[cfg(feature = "participation")]
        AccountMethod::DeregisterParticipationEvent { event_id } => {
            account.deregister_participation_event(&event_id).await?;
            Response::Ok
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEvent { event_id } => {
            let event_and_nodes = account.get_participation_event(event_id).await?;
            Response::ParticipationEvent(event_and_nodes)
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEventIds { node, event_type } => {
            let event_ids = account.get_participation_event_ids(&node, event_type).await?;
            Response::ParticipationEventIds(event_ids)
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEventStatus { event_id } => {
            let event_status = account.get_participation_event_status(&event_id).await?;
            Response::ParticipationEventStatus(event_status)
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEvents => {
            let events = account.get_participation_events().await?;
            Response::ParticipationEvents(events)
        }
    };
    Ok(response)
}
