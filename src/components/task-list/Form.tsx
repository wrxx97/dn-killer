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
  const {
    register,
    setValue,
    handleSubmit,
    formState: { errors },
  } = useForm({
    defaultValues: data,
  });
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
        store.updateTask({ ...data });
        store.save();
      })}
    >
      <TextField {...register("title")} label="标题" variant="outlined" />
      <TextField
        {...register("duration")}
        label="时长"
        type="number"
        variant="outlined"
        onKeyDown={(e) => {
          console.info(e);
        }}
      />
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
        <IconButton onClick={store.addTask}>
          <AddIcon />
        </IconButton>
        <IconButton onClick={() => store.deleteTask(data.id)}>
          <DeleteIcon />
        </IconButton>
      </ButtonGroup>
    </Box>
  );
};

function formatShortcut(event: KeyboardEvent) {
  let formattedShortcut = "";
  if (event.ctrlKey) {
    formattedShortcut += "Ctrl+";
  }
  if (event.altKey) {
    formattedShortcut += "Alt+";
  }
  if (event.shiftKey) {
    formattedShortcut += "Shift+";
  }
  formattedShortcut += event.key.toUpperCase();
  return formattedShortcut;
}
