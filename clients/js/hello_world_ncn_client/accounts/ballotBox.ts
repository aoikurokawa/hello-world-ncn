/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  getAddressDecoder,
  getAddressEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
} from '@solana/web3.js';
import {
  getOperatorVoteDecoder,
  getOperatorVoteEncoder,
  type OperatorVote,
  type OperatorVoteArgs,
} from '../types';

export type BallotBox = {
  discriminator: bigint;
  ncn: Address;
  epoch: bigint;
  slotCreated: bigint;
  slotConsensusReached: bigint;
  operatorsVoted: bigint;
  operatorVotes: Array<OperatorVote>;
};

export type BallotBoxArgs = {
  discriminator: number | bigint;
  ncn: Address;
  epoch: number | bigint;
  slotCreated: number | bigint;
  slotConsensusReached: number | bigint;
  operatorsVoted: number | bigint;
  operatorVotes: Array<OperatorVoteArgs>;
};

export function getBallotBoxEncoder(): Encoder<BallotBoxArgs> {
  return getStructEncoder([
    ['discriminator', getU64Encoder()],
    ['ncn', getAddressEncoder()],
    ['epoch', getU64Encoder()],
    ['slotCreated', getU64Encoder()],
    ['slotConsensusReached', getU64Encoder()],
    ['operatorsVoted', getU64Encoder()],
    ['operatorVotes', getArrayEncoder(getOperatorVoteEncoder(), { size: 3 })],
  ]);
}

export function getBallotBoxDecoder(): Decoder<BallotBox> {
  return getStructDecoder([
    ['discriminator', getU64Decoder()],
    ['ncn', getAddressDecoder()],
    ['epoch', getU64Decoder()],
    ['slotCreated', getU64Decoder()],
    ['slotConsensusReached', getU64Decoder()],
    ['operatorsVoted', getU64Decoder()],
    ['operatorVotes', getArrayDecoder(getOperatorVoteDecoder(), { size: 3 })],
  ]);
}

export function getBallotBoxCodec(): Codec<BallotBoxArgs, BallotBox> {
  return combineCodec(getBallotBoxEncoder(), getBallotBoxDecoder());
}

export function decodeBallotBox<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<BallotBox, TAddress>;
export function decodeBallotBox<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<BallotBox, TAddress>;
export function decodeBallotBox<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<BallotBox, TAddress> | MaybeAccount<BallotBox, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getBallotBoxDecoder()
  );
}

export async function fetchBallotBox<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<BallotBox, TAddress>> {
  const maybeAccount = await fetchMaybeBallotBox(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeBallotBox<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<BallotBox, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeBallotBox(maybeAccount);
}

export async function fetchAllBallotBox(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<BallotBox>[]> {
  const maybeAccounts = await fetchAllMaybeBallotBox(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeBallotBox(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<BallotBox>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeBallotBox(maybeAccount));
}
