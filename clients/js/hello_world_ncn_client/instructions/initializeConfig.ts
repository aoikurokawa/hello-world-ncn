/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
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

export const INITIALIZE_CONFIG_DISCRIMINATOR = 0;

export function getInitializeConfigDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_CONFIG_DISCRIMINATOR);
}

export type InitializeConfigInstruction<
  TProgram extends string = typeof HELLO_WORLD_NCN_PROGRAM_ADDRESS,
  TAccountConfigInfo extends string | IAccountMeta<string> = string,
  TAccountNcnInfo extends string | IAccountMeta<string> = string,
  TAccountNcnAdminInfo extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
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
      TAccountNcnAdminInfo extends string
        ? WritableSignerAccount<TAccountNcnAdminInfo> &
            IAccountSignerMeta<TAccountNcnAdminInfo>
        : TAccountNcnAdminInfo,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeConfigInstructionData = {
  discriminator: number;
  minStake: bigint;
};

export type InitializeConfigInstructionDataArgs = { minStake: number | bigint };

export function getInitializeConfigInstructionDataEncoder(): Encoder<InitializeConfigInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['minStake', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: INITIALIZE_CONFIG_DISCRIMINATOR })
  );
}

export function getInitializeConfigInstructionDataDecoder(): Decoder<InitializeConfigInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['minStake', getU64Decoder()],
  ]);
}

export function getInitializeConfigInstructionDataCodec(): Codec<
  InitializeConfigInstructionDataArgs,
  InitializeConfigInstructionData
> {
  return combineCodec(
    getInitializeConfigInstructionDataEncoder(),
    getInitializeConfigInstructionDataDecoder()
  );
}

export type InitializeConfigInput<
  TAccountConfigInfo extends string = string,
  TAccountNcnInfo extends string = string,
  TAccountNcnAdminInfo extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  configInfo: Address<TAccountConfigInfo>;
  ncnInfo: Address<TAccountNcnInfo>;
  ncnAdminInfo: TransactionSigner<TAccountNcnAdminInfo>;
  systemProgram?: Address<TAccountSystemProgram>;
  minStake: InitializeConfigInstructionDataArgs['minStake'];
};

export function getInitializeConfigInstruction<
  TAccountConfigInfo extends string,
  TAccountNcnInfo extends string,
  TAccountNcnAdminInfo extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof HELLO_WORLD_NCN_PROGRAM_ADDRESS,
>(
  input: InitializeConfigInput<
    TAccountConfigInfo,
    TAccountNcnInfo,
    TAccountNcnAdminInfo,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeConfigInstruction<
  TProgramAddress,
  TAccountConfigInfo,
  TAccountNcnInfo,
  TAccountNcnAdminInfo,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? HELLO_WORLD_NCN_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    configInfo: { value: input.configInfo ?? null, isWritable: true },
    ncnInfo: { value: input.ncnInfo ?? null, isWritable: false },
    ncnAdminInfo: { value: input.ncnAdminInfo ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.configInfo),
      getAccountMeta(accounts.ncnInfo),
      getAccountMeta(accounts.ncnAdminInfo),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeConfigInstructionDataEncoder().encode(
      args as InitializeConfigInstructionDataArgs
    ),
  } as InitializeConfigInstruction<
    TProgramAddress,
    TAccountConfigInfo,
    TAccountNcnInfo,
    TAccountNcnAdminInfo,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeConfigInstruction<
  TProgram extends string = typeof HELLO_WORLD_NCN_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    configInfo: TAccountMetas[0];
    ncnInfo: TAccountMetas[1];
    ncnAdminInfo: TAccountMetas[2];
    systemProgram: TAccountMetas[3];
  };
  data: InitializeConfigInstructionData;
};

export function parseInitializeConfigInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeConfigInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
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
      ncnAdminInfo: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeConfigInstructionDataDecoder().decode(instruction.data),
  };
}
