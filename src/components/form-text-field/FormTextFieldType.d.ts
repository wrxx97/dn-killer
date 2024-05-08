import { TextFieldProps } from "@mui/material/TextField";
export type FormTextFieldProps = {
  name: string;
  control: any;
  label?: string;
  type?: string;
} & TextFieldProps;
