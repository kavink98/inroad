import "@near-wallet-selector/modal-ui/styles.css";
import { Button } from "@mantine/core";
import { useWallet } from "../near-utils/Wallet";
import { MouseEventHandler, useEffect, useState } from "react";

const Login = () => {
  const { logIn, logOut, signedAccountId } = useWallet();
  const [label, setLabel] = useState<string>();
  const [action, setAction] = useState<
    MouseEventHandler<HTMLButtonElement> | undefined
  >(() => {});

  useEffect(() => {
    if (signedAccountId) {
      setAction(() => logOut);
      setLabel(`Logout ${signedAccountId}`);
    } else {
      setAction(() => logIn);
      setLabel("Login");
    }
  }, [signedAccountId, logOut, logIn, setAction, setLabel]);

  return (
    <>
      <Button onClick={action} variant="transparent">
        {label}
      </Button>
    </>
  );
};

export default Login;
