import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import TextField from "@mui/material/TextField";
import ButtonGroup from "@mui/material/ButtonGroup";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import SaveIcon from "@mui/icons-material/Save";
import { useForm } from "react-hook-form";
import useStore, { Task } from "@/stores";

export default ({ data }: { data: Task }) => {
  const store = useStore();
  const { register, setValue, handleSubmit } = useForm({
    defaultValues: data,
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
      autoComplete="off"
      onSubmit={handleSubmit((data) => {
        isExit
          ? store.updateExitTask({ ...data })
          : store.updateTask({ ...data });
        store.save();
      })}
    >
      <TextField {...register("title")} label="标题" variant="outlined" />
      {!isExit ? (
        <TextField
          {...register("duration")}
          label="时长"
          type="number"
          variant="outlined"
          onKeyDown={(e) => {
            console.info(e);
          }}
        />
      ) : null}
      <TextField
        {...register("hotkey")}
        label="快捷键"
        variant="outlined"
        onKeyDown={(e) => {
          e.stopPropagation();
          e.preventDefault();
          setValue("hotkey", formatShortcut(e));
        }}
      />
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

function formatShortcut(event: any) {
  let formattedShortcut = "";
  // if (event.ctrlKey) {
  //   formattedShortcut += "Ctrl+";
  // }
  // if (event.altKey) {
  //   formattedShortcut += "Alt+";
  // }
  // if (event.shiftKey) {
  //   formattedShortcut += "Shift+";
  // }
  formattedShortcut += event.key.toUpperCase();
  return formattedShortcut;
}
