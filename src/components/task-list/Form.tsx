import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import ButtonGroup from "@mui/material/ButtonGroup";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import SaveIcon from "@mui/icons-material/Save";
import FormTextField from "@/components/form-text-field";
import FormHotKey from "@/components/form-hot-key";
import { useForm } from "react-hook-form";
import useStore, { Task } from "@/stores";

export default ({ data }: { data: Task }) => {
  const store = useStore();
  const { handleSubmit, control } = useForm({
    defaultValues: data,
    values: data
  });

  const isExit = data.type === "exit";

  return (
    <Box
      component="form"
      sx={{
        display: "flex",
        alignItems: "center",
        gap: "1rem",
        color: "white",
      }}
      noValidate
      autoComplete="true"
      onSubmit={handleSubmit((data) => {
        isExit
          ? store.updateExitTask({ ...data })
          : store.updateTask({ ...data });
        store.save();
      })}
    >
      <FormTextField name="title" control={control} label="标题" />
      {!isExit ? (
        <FormTextField name="duration" control={control} type="number" label="时长" />
      ) : null}
      <FormHotKey name="hotkey" control={control} label="快捷键" />
      <ButtonGroup size="small" aria-label="Small button group">
        <IconButton type="submit">
          <SaveIcon />
        </IconButton>
        {isExit
          ? null
          : [
              <IconButton onClick={store.addTask} key="add">
                <AddIcon />
              </IconButton>,
              <IconButton
                key="delete"
                onClick={() => {
                  store.deleteTask(data.id);
                  store.save();
                }}
              >
                <DeleteIcon />
              </IconButton>,
            ]}
      </ButtonGroup>
    </Box>
  );
};
