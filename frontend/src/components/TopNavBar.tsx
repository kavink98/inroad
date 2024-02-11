import { Group } from "@mantine/core";
import Login from "./Login";

const TopNavBar = () => {
  return (
    <Group justify="flex-end" pos={"absolute"} top={0} w={"50%"} p={0}>
      <Login />
    </Group>
  );
};

export default TopNavBar;
