import { Controller } from "react-hook-form";
import Slider from "@mui/material/Slider";
import { FormSliderProps } from "./FormSlider";

export default function FormInputText({
  name,
  control,
  label,
}: FormSliderProps) {
  return (
    <Controller
      name={name}
      control={control}
      render={({ field: { onChange, value } }) => (
        <Slider
          aria-label={label}
          color="primary"
          value={value}
          onChange={onChange}
        />
      )}
    />
  );
}
