import { useEffect } from "react";
import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import ButtonGroup from "@mui/material/ButtonGroup";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import FormTextField from "@/components/form-text-field";
import FormHotKey from "@/components/form-hot-key";
import { useForm } from "react-hook-form";
import useStore, { Task } from "@/stores";

export default ({ data }: { data: Task }) => {
  const store = useStore();
  const { control, watch, setFocus } = useForm({
    values: data,
  });

  const isExit = data.type === "exit";

  useEffect(() => {
    const subscription = watch((value) => {
      const data = value as Task;
      console.info(data);
      isExit
        ? store.updateExitTask({ ...data })
        : store.updateTask({ ...data });
      store.save();
    });
    return () => subscription.unsubscribe();
  }, [watch, isExit]);

  useEffect(() => {
    if (store.focusTask?.id === data.id) setFocus("title");
  }, [setFocus, store.focusTask]);

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
    >
      <FormTextField name="title" control={control} label="标题" />
      {!isExit ? (
        <FormTextField
          name="duration"
          control={control}
          type="number"
          label="时长"
        />
      ) : null}
      <FormHotKey name="hotkey" control={control} label="快捷键" />
      <ButtonGroup size="small" aria-label="Small button group">
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
