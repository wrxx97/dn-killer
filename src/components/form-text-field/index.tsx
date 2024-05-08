import { Controller } from "react-hook-form";
import TextField from "@mui/material/TextField";
import type { FormTextFieldProps } from "./FormTextFieldType";

export default function FormInputText({
  name,
  control,
  label,
  type,
  ...rest
}: FormTextFieldProps) {
  return (
    <Controller
      name={name}
      control={control}
      render={({ field: { onChange, value, ref }, fieldState: { error } }) => (
        <TextField
          helperText={error ? error.message : null}
          error={!!error}
          onChange={onChange}
          value={value}
          label={label}
          variant="outlined"
          type={type}
          inputRef={ref}
          fullWidth
          {...rest}
        />
      )}
    />
  );
}
