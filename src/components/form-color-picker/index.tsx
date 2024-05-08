import { Controller } from "react-hook-form";
import { MuiColorInput } from "mui-color-input";
import FormControlLabel from "@mui/material/FormControlLabel";
import type { FormColorPickerProps } from "./FormColorPicker";

export default function FormInputText({
  name,
  control,
  label,
}: FormColorPickerProps) {
  return (
    <Controller
      name={name}
      control={control}
      render={({ field: { onChange, value } }) => (
        <FormControlLabel
          value={value}
          control={
            <MuiColorInput
              format="hex8"
              value={value}
              onChange={onChange}
              style={{
                marginLeft: "0.8rem",
              }}
            />
          }
          label={label}
          labelPlacement="start"
        />
      )}
    />
  );
}
