import { Button, TextInput } from "@mantine/core";

const CreateProject = () => {
  const deployProject = () => {};
  return (
    <>
      <TextInput variant="filled" label="Project Name" />
      <TextInput variant="filled" label="Project Description" />
      <TextInput variant="filled" label="Project ID" />
      <Button mt="lg" onClick={deployProject}>
        {" "}
        Submit{" "}
      </Button>
    </>
  );
};

export default CreateProject;
