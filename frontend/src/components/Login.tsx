import "@near-wallet-selector/modal-ui/styles.css";
import { Button, Text } from "@mantine/core";
import { setupWalletSelector } from "@near-wallet-selector/core";
import { setupModal } from "@near-wallet-selector/modal-ui";
import { setupNearWallet } from "@near-wallet-selector/near-wallet";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";

const Login = () => {
  const signIn = async () => {
    const selector = await setupWalletSelector({
      network: "testnet",
      modules: [setupNearWallet(), setupMyNearWallet()],
    });

    const modal = setupModal(selector, { contractId: "test.testnet" });

    modal.show();
  };

  return (
    <>
      <Text>
        Please log in to your NEAR account to be able to interact with smart
        contract beyond read only methods
      </Text>
      <Button onClick={signIn}>Sign In</Button>
    </>
  );
};

export default Login;
