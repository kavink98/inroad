import { Group } from "@mantine/core";
import Login from "./Login";

const TopNavBar = () => {
  return (
    <Group
      justify="flex-end"
      pos={"absolute"}
      top={0}
      left={0}
      right={0}
      m={"md"}
    >
      <Login />
    </Group>
  );
};

export default TopNavBar;
