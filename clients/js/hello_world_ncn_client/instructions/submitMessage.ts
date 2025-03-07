/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  addDecoderSizePrefix,
  addEncoderSizePrefix,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  getU8Decoder,
  getU8Encoder,
  getUtf8Decoder,
  getUtf8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/web3.js';
import { HELLO_WORLD_NCN_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const SUBMIT_MESSAGE_DISCRIMINATOR = 3;

export function getSubmitMessageDiscriminatorBytes() {
  return getU8Encoder().encode(SUBMIT_MESSAGE_DISCRIMINATOR);
}

export type SubmitMessageInstruction<
  TProgram extends string = typeof HELLO_WORLD_NCN_PROGRAM_ADDRESS,
  TAccountConfigInfo extends string | IAccountMeta<string> = string,
  TAccountNcnInfo extends string | IAccountMeta<string> = string,
  TAccountOperatorInfo extends string | IAccountMeta<string> = string,
  TAccountMessageInfo extends string | IAccountMeta<string> = string,
  TAccountBallotBoxInfo extends string | IAccountMeta<string> = string,
  TAccountOperatorVoterInfo extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfigInfo extends string
        ? WritableAccount<TAccountConfigInfo>
        : TAccountConfigInfo,
      TAccountNcnInfo extends string
        ? ReadonlyAccount<TAccountNcnInfo>
        : TAccountNcnInfo,
      TAccountOperatorInfo extends string
        ? ReadonlyAccount<TAccountOperatorInfo>
        : TAccountOperatorInfo,
      TAccountMessageInfo extends string
        ? ReadonlyAccount<TAccountMessageInfo>
        : TAccountMessageInfo,
      TAccountBallotBoxInfo extends string
        ? WritableAccount<TAccountBallotBoxInfo>
        : TAccountBallotBoxInfo,
      TAccountOperatorVoterInfo extends string
        ? WritableSignerAccount<TAccountOperatorVoterInfo> &
            IAccountSignerMeta<TAccountOperatorVoterInfo>
        : TAccountOperatorVoterInfo,
      ...TRemainingAccounts,
    ]
  >;

export type SubmitMessageInstructionData = {
  discriminator: number;
  message: string;
};

export type SubmitMessageInstructionDataArgs = { message: string };

export function getSubmitMessageInstructionDataEncoder(): Encoder<SubmitMessageInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['message', addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder())],
    ]),
    (value) => ({ ...value, discriminator: SUBMIT_MESSAGE_DISCRIMINATOR })
  );
}

export function getSubmitMessageInstructionDataDecoder(): Decoder<SubmitMessageInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['message', addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder())],
  ]);
}

export function getSubmitMessageInstructionDataCodec(): Codec<
  SubmitMessageInstructionDataArgs,
  SubmitMessageInstructionData
> {
  return combineCodec(
    getSubmitMessageInstructionDataEncoder(),
    getSubmitMessageInstructionDataDecoder()
  );
}

export type SubmitMessageInput<
  TAccountConfigInfo extends string = string,
  TAccountNcnInfo extends string = string,
  TAccountOperatorInfo extends string = string,
  TAccountMessageInfo extends string = string,
  TAccountBallotBoxInfo extends string = string,
  TAccountOperatorVoterInfo extends string = string,
> = {
  configInfo: Address<TAccountConfigInfo>;
  ncnInfo: Address<TAccountNcnInfo>;
  operatorInfo: Address<TAccountOperatorInfo>;
  messageInfo: Address<TAccountMessageInfo>;
  ballotBoxInfo: Address<TAccountBallotBoxInfo>;
  operatorVoterInfo: TransactionSigner<TAccountOperatorVoterInfo>;
  message: SubmitMessageInstructionDataArgs['message'];
};

export function getSubmitMessageInstruction<
  TAccountConfigInfo extends string,
  TAccountNcnInfo extends string,
  TAccountOperatorInfo extends string,
  TAccountMessageInfo extends string,
  TAccountBallotBoxInfo extends string,
  TAccountOperatorVoterInfo extends string,
  TProgramAddress extends Address = typeof HELLO_WORLD_NCN_PROGRAM_ADDRESS,
>(
  input: SubmitMessageInput<
    TAccountConfigInfo,
    TAccountNcnInfo,
    TAccountOperatorInfo,
    TAccountMessageInfo,
    TAccountBallotBoxInfo,
    TAccountOperatorVoterInfo
  >,
  config?: { programAddress?: TProgramAddress }
): SubmitMessageInstruction<
  TProgramAddress,
  TAccountConfigInfo,
  TAccountNcnInfo,
  TAccountOperatorInfo,
  TAccountMessageInfo,
  TAccountBallotBoxInfo,
  TAccountOperatorVoterInfo
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? HELLO_WORLD_NCN_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    configInfo: { value: input.configInfo ?? null, isWritable: true },
    ncnInfo: { value: input.ncnInfo ?? null, isWritable: false },
    operatorInfo: { value: input.operatorInfo ?? null, isWritable: false },
    messageInfo: { value: input.messageInfo ?? null, isWritable: false },
    ballotBoxInfo: { value: input.ballotBoxInfo ?? null, isWritable: true },
    operatorVoterInfo: {
      value: input.operatorVoterInfo ?? null,
      isWritable: true,
    },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.configInfo),
      getAccountMeta(accounts.ncnInfo),
      getAccountMeta(accounts.operatorInfo),
      getAccountMeta(accounts.messageInfo),
      getAccountMeta(accounts.ballotBoxInfo),
      getAccountMeta(accounts.operatorVoterInfo),
    ],
    programAddress,
    data: getSubmitMessageInstructionDataEncoder().encode(
      args as SubmitMessageInstructionDataArgs
    ),
  } as SubmitMessageInstruction<
    TProgramAddress,
    TAccountConfigInfo,
    TAccountNcnInfo,
    TAccountOperatorInfo,
    TAccountMessageInfo,
    TAccountBallotBoxInfo,
    TAccountOperatorVoterInfo
  >;

  return instruction;
}

export type ParsedSubmitMessageInstruction<
  TProgram extends string = typeof HELLO_WORLD_NCN_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    configInfo: TAccountMetas[0];
    ncnInfo: TAccountMetas[1];
    operatorInfo: TAccountMetas[2];
    messageInfo: TAccountMetas[3];
    ballotBoxInfo: TAccountMetas[4];
    operatorVoterInfo: TAccountMetas[5];
  };
  data: SubmitMessageInstructionData;
};

export function parseSubmitMessageInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedSubmitMessageInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 6) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      configInfo: getNextAccount(),
      ncnInfo: getNextAccount(),
      operatorInfo: getNextAccount(),
      messageInfo: getNextAccount(),
      ballotBoxInfo: getNextAccount(),
      operatorVoterInfo: getNextAccount(),
    },
    data: getSubmitMessageInstructionDataDecoder().decode(instruction.data),
  };
}
