import { Controller } from "react-hook-form";
import TextField from "@mui/material/TextField";
import { FormHotKeyProps } from "./FormHotKey";

export default function FormInputText({
  name,
  control,
  label,
}: FormHotKeyProps) {
  return (
    <Controller
      name={name}
      control={control}
      render={({
        field: { onChange, value },
        fieldState: { error },
        formState,
      }) => (
        <TextField
          helperText={error ? error.message : null}
          error={!!error}
          onKeyDown={(e) => {
            e.stopPropagation();
            e.preventDefault();
            onChange(formatShortcut(e));
          }}
          value={value}
          label={label}
          variant="outlined"
        />
      )}
    />
  );
}

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
