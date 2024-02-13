import {
  NetworkId,
  WalletSelector,
  setupWalletSelector,
} from "@near-wallet-selector/core";
import { setupModal } from "@near-wallet-selector/modal-ui";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";
import { providers, utils } from "near-api-js";
import { CodeResult } from "near-api-js/lib/providers/provider";
import { useEffect, useState } from "react";
import { create } from "zustand";

type LogFunctions = undefined | (() => void);
type CallMethod = undefined | ((contractId: string, method: string, args?: {}, gas?: string, deposit?: number) => Promise<any>)
type ViewMethod = undefined | ((contractId: string, method: string, args?: {}) => Promise<any>)

type Wallet = {
  selector: WalletSelector | undefined;
  signedAccountId: string;
  logOut: LogFunctions;
  logIn: LogFunctions;
  viewMethod: ViewMethod;
  callMethod: CallMethod;

  // TODO: Better type defs for function params
  setLogActions: (a: any) => void;
  setAuth: (a: any) => void;
  setMethods: (a: any) => void;
  setStoreSelector: (a: any) => void;
};

export const useWallet = create<Wallet>()((set) => ({
  signedAccountId: "",
  logOut: undefined,
  logIn: undefined,
  selector: undefined,
  viewMethod: undefined,
  callMethod: undefined,
  setLogActions: ({ logOut, logIn }) => set({ logOut, logIn }),
  setAuth: ({ signedAccountId }) => set({ signedAccountId }),
  setMethods: ({ viewMethod, callMethod }) => set({ viewMethod, callMethod }),
  setStoreSelector: ({ selector }) => set({ selector }),
}));

export function useInitWallet({
  createAccessKeyFor,
  networkId,
}: {
  createAccessKeyFor: string;
  networkId: NetworkId;
}) {
  const setAuth = useWallet((store) => store.setAuth);
  const setLogActions = useWallet((store) => store.setLogActions);
  const setMethods = useWallet((store) => store.setMethods);
  const setStoreSelector = useWallet((store) => store.setStoreSelector);
  const [selector, setSelector] = useState<WalletSelector>();

  useEffect(() => {
    const setUpAndSetSelector = async () => {
      const selector = setupWalletSelector({
        network: networkId,
        modules: [setupMyNearWallet()],
      });

      setSelector(await selector);
      setStoreSelector({ selector });
    };
    setUpAndSetSelector();
  }, [networkId, setStoreSelector]);

  useEffect(() => {
    if (!selector) return;

    const accounts = selector.store.getState().accounts;
    const signedAccountId =
      accounts.find((account) => account.active)?.accountId || "";
    setAuth({ signedAccountId });
  }, [selector, setAuth]);

  useEffect(() => {
    if (!selector) return;

    const logOut = async () => {
      const wallet = await selector.wallet();
      await wallet.signOut();
      setAuth({ signedAccountId: "" });
    };

    const logIn = async () => {
      const modal = setupModal(selector, { contractId: createAccessKeyFor });
      modal.show();
    };

    setLogActions({ logOut, logIn });
  }, [createAccessKeyFor, selector, setAuth, setLogActions]);

  useEffect(() => {
    if (!selector) return;

    const viewMethod = async (
      contractId: string,
      method: string,
      args = {}
    ) => {
      const { network } = (await selector).options;
      const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

      let res = await provider.query<CodeResult>({
        request_type: "call_function",
        account_id: contractId,
        method_name: method,
        args_base64: Buffer.from(JSON.stringify(args)).toString("base64"),
        finality: "optimistic",
      });
      return JSON.parse(Buffer.from(res.result).toString());
    };

    const callMethod = async (
      contractId: string,
      method: string,
      args = {},
      gas = "30000000000000",
      deposit = 0
    ) => {
      const wallet = await selector.wallet();

      const outcome = await wallet.signAndSendTransaction({
        receiverId: contractId,
        actions: [
          {
            type: "FunctionCall",
            params: {
              methodName: method,
              args,
              gas,
              deposit: utils.format.parseNearAmount(deposit.toString())!,
            },
          },
        ],
      });
      if (outcome != undefined) {
        return providers.getTransactionLastResult(outcome);
      }
    };

    setMethods({ viewMethod, callMethod });
  }, [selector, setMethods]);
}
