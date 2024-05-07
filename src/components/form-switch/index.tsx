import { Controller } from "react-hook-form";
import Switch from "@mui/material/Switch";
import FormControlLabel from "@mui/material/FormControlLabel";
import { FormSwitchProps } from "./FormSwitch";

export default function FormInputText({
  name,
  control,
  label,
}: FormSwitchProps) {
  return (
    <Controller
      name={name}
      control={control}
      render={({ field: { onChange, value } }) => (
        <FormControlLabel
          value={value}
          control={
            <Switch color="primary" checked={value} onChange={onChange} />
          }
          label={label}
        />
      )}
    />
  );
}
