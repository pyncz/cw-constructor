import type { ExecuteInstruction } from '@cosmjs/cosmwasm-stargate';
import { type CallOptions, type Coin, type ExecInstruction, type Msg, NoContractAddressError, NotConnectedError } from '~/types';

export const useContract = (contractAddress?: MaybeRefOrGetter<string | undefined>) => {
  const { clientAsync: queryClientAsync } = useRpcClient();
  const { client: signingClient, address } = useConnect();

  const query = async <T>(msg: Msg, options?: CallOptions): Promise<T> => {
    const contract = options?.contractAddress ?? toValue(contractAddress);
    if (!contract) {
      throw new NoContractAddressError();
    }

    const queryClient = await queryClientAsync.value;
    return await queryClient.queryContractSmart(contract, msg);
  };

  const execute = async (msg: Msg, options?: CallOptions & { memo?: string; funds?: Coin[]; gasAdjustment?: number }) => {
    const { memo, funds, gasAdjustment } = options ?? {};

    if (!address.value || !signingClient.value) {
      throw new NotConnectedError();
    }
    const contract = options?.contractAddress ?? toValue(contractAddress);
    if (!contract) {
      throw new NoContractAddressError();
    }

    return await signingClient.value.execute(address.value, contract, msg, gasAdjustment || 'auto', memo, funds);
  };

  const executeMultiple = async (instructions: ExecInstruction[], options?: CallOptions & { memo?: string; gasAdjustment?: number }) => {
    const { memo, gasAdjustment } = options ?? {};

    if (!address.value || !signingClient.value) {
      throw new NotConnectedError();
    }
    const contract = options?.contractAddress ?? toValue(contractAddress);
    const parsedInstructions: ExecuteInstruction[] = instructions.map((instruction) => {
      if (!contract && !instruction.contractAddress) {
        throw new NoContractAddressError();
      }
      return {
        ...instruction,
        contractAddress: instruction.contractAddress ?? contract!,
      };
    });

    return await signingClient.value.executeMultiple(address.value, parsedInstructions, gasAdjustment || 'auto', memo);
  };

  return {
    query,
    execute,
    executeMultiple,
  };
};
